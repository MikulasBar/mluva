use std::{collections::HashMap, path::Path};

use codespan_reporting::{
    files::SimpleFiles,
    term::{
        Config as CodespanConfig, emit_to_io_write,
        termcolor::{ColorChoice, StandardStream},
    },
};
use common::module::{
    module_code::{self, ModuleCode},
    module_code_manager::{self, ModuleCodeManager},
    module_signiture::ModuleSigniture,
};
use compiler::compiler::Compiler;
use typechecker::TypeChecker;

use crate::{
    commands::create_meta_storage,
    config::Config,
    error::CliError,
    module_metadata::{ModuleMetadata, ModuleMetadataStorage},
};

pub fn command() -> Result<(Config, ModuleCodeManager), ()> {
    println!("Building the Mluva project...");

    let config = Config::load_from_file()?;
    if !config.root_module_file_exists() {
        eprintln!("Root module '{}' not found", config.root_module_file_path());
        return Err(());
    }

    create_meta_storage()?;

    let mut module_meta_storage = ModuleMetadataStorage::load_from_file()?;
    let mut module_code_manager = ModuleCodeManager::new();
    let mut dependencies: HashMap<String, ModuleSigniture> = HashMap::new();
    let mut parent_stack: Vec<String> = vec![];
    let mut files = SimpleFiles::new();

    let compile_result = compile_module(
        &config.root_module,
        &mut module_code_manager,
        &mut dependencies,
        &mut module_meta_storage,
        &mut parent_stack,
        &mut files,
    );

    match compile_result {
        Ok(_) => {
            module_meta_storage.save_to_file()?;
            println!("Build completed!");
            Ok((config, module_code_manager))
        }
        Err(CliError::Compile(e)) => {
            let diag = e.to_diagnostic();
            let writer = StandardStream::stderr(ColorChoice::Auto);
            let Ok(_) = emit_to_io_write(
                &mut writer.lock(),
                &CodespanConfig::default(),
                &files,
                &diag,
            ) else {
                eprintln!("Failed to write diagnostics");
                return Err(());
            };
            Err(())
        }
        Err(CliError::Encode(e)) => {
            eprintln!("Encoding error during compilation: {}", e);
            Err(())
        }
        Err(CliError::Decode(e)) => {
            eprintln!("Decoding error during compilation: {}", e);
            Err(())
        }
        Err(CliError::Other(msg)) => {
            eprintln!("Error during compilation: {}", msg);
            Err(())
        }
        Err(CliError::Handled) => {
            // Error already reported
            Err(())
        }
    }
}

fn compile_module(
    source_module: &str,
    module_code_manager: &mut ModuleCodeManager,
    dependencies: &mut HashMap<String, ModuleSigniture>,
    module_meta_storage: &mut ModuleMetadataStorage,
    parent_stack: &mut Vec<String>, // TODO: change to something that is not O(n) on search but has ordering
    files: &mut SimpleFiles<String, String>,
) -> Result<(), CliError> {
    let source_path = Path::new(source_module)
        .with_extension("mv")
        .to_string_lossy()
        .to_string();

    if module_code_manager.contains_mod(source_module) {
        return Ok(());
    }

    if parent_stack.iter().any(|p| p == source_module) {
        eprintln!(
            "Cyclic dependency detected: {} -> {}",
            parent_stack.join(" -> "),
            source_module
        );
        return Err(CliError::Handled);
    }

    parent_stack.push(source_module.to_string());

    let Ok(content) = std::fs::read(&source_path) else {
        eprintln!("Failed to read module file: {}", source_path);
        return Err(CliError::Handled);
    };

    let Ok(content_str) = String::from_utf8(content.clone()) else {
        eprintln!("Module file is not valid UTF-8: {}", source_path);
        return Err(CliError::Handled);
    };

    let file_id = files.add(source_path.clone(), content_str.clone());
    let mut module_ast = frontend::parse_source(&content_str, file_id)?;

    for import in module_ast.imports() {
        // TODO: resolve full path
        let import_path_str = import.get_tail().unwrap();
        let import_path = Path::new(import_path_str).with_extension("mv");

        if !import_path.exists() {
            eprintln!(
                "Dependecy module {} of module {} not found",
                import_path.display(),
                source_path
            );
            return Err(CliError::Handled);
        }

        compile_module(
            import_path_str,
            module_code_manager,
            dependencies,
            module_meta_storage,
            parent_stack,
            files,
        )?;
    }

    let sign_path_str = ModuleMetadata::source_to_signiture_path(&source_path);
    let code_path_str = ModuleMetadata::source_to_code_path(&source_path);
    let sign_path = Path::new(&sign_path_str);
    let code_path = Path::new(&code_path_str);

    let needs_compilation = module_meta_storage.needs_recompilation(&source_path, &content);

    if needs_compilation || !code_path.exists() || !sign_path.exists() {
        TypeChecker::new(&mut module_ast, dependencies).check();
        let mod_code = Compiler::new(&module_ast, dependencies).compile()?;
        let mod_sign = module_ast.to_signiture();
        let bytecode = mod_code.serialize()?;
        let sign_bytecode = mod_sign.serialize()?;

        let Ok(_) = std::fs::write(&code_path, bytecode) else {
            eprintln!("Failed to write bytecode file for module {}", source_path);
            return Err(CliError::Handled);
        };

        let Ok(_) = std::fs::write(&sign_path, sign_bytecode) else {
            eprintln!("Failed to write signiture file for module {}", source_path);
            return Err(CliError::Handled);
        };

        module_meta_storage.update_hash(&source_path, &content);
        module_code_manager.add(source_module.to_string(), mod_code);
    } else {
        // load from cached bytecode
        let Ok(bytecode) = std::fs::read(&code_path) else {
            eprintln!("Failed to read bytecode file for module {}", source_path);
            return Err(CliError::Handled);
        };

        let mod_code = ModuleCode::deserialize(&bytecode)?;

        let Ok(sign_bytecode) = std::fs::read(&sign_path) else {
            eprintln!("Failed to read signiture file for module {}", source_path);
            return Err(CliError::Handled);
        };

        let mod_sign = ModuleSigniture::deserialize(&sign_bytecode)?;

        module_code_manager.add(source_module.to_string(), mod_code);
        dependencies.insert(source_module.to_string(), mod_sign);
    }

    parent_stack.pop();

    Ok(())
}
