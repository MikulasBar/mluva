use std::{collections::HashMap, path::Path};

use codespan_reporting::{
    files::SimpleFiles,
    term::{
        Config as CodespanConfig, emit_to_io_write,
        termcolor::{ColorChoice, StandardStream},
    },
};
use mluva::{ast::Ast, errors::CompileError, module::Module};

use crate::{
    commands::create_meta_storage,
    config::Config,
    module_metadata::{ModuleMetadata, ModuleMetadataStorage},
};

pub fn command() -> Result<(Config, HashMap<String, Module>), ()> {
    println!("Building the Mluva project...");

    let config = Config::load_from_file()?;
    if !config.root_module_file_exists() {
        eprintln!("Root module '{}' not found", config.root_module_file_path());
        return Err(());
    }

    create_meta_storage()?;

    let mut module_meta_storage = ModuleMetadataStorage::load_from_file()?;
    let mut compiled_modules: HashMap<String, Module> = HashMap::new();
    let mut parent_stack: Vec<String> = vec![];
    let mut files = SimpleFiles::new();

    let compile_result = compile_module(
        &config.root_module,
        &mut compiled_modules,
        &mut module_meta_storage,
        &mut parent_stack,
        &mut files,
    );

    match compile_result {
        Ok(_) => {
            module_meta_storage.save_to_file()?;
            println!("Build completed!");
            Ok((config, compiled_modules))
        }
        Err(Some(e)) => {
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
        Err(None) => {
            // Error already reported
            Err(())
        }
    }
}

fn compile_module(
    source_module: &str,
    compiled_modules: &mut HashMap<String, Module>,
    module_meta_storage: &mut ModuleMetadataStorage,
    parent_stack: &mut Vec<String>, // TODO: change to something that is not O(n) on search but has ordering
    files: &mut SimpleFiles<String, String>,
) -> Result<(), Option<CompileError>> {
    let source_path = Path::new(source_module)
        .with_extension("mv")
        .to_string_lossy()
        .to_string();

    if compiled_modules.contains_key(source_module) {
        return Ok(());
    }

    if parent_stack.iter().any(|p| p == source_module) {
        eprintln!(
            "Cyclic dependency detected: {} -> {}",
            parent_stack.join(" -> "),
            source_module
        );
        return Err(None);
    }

    parent_stack.push(source_module.to_string());

    let Ok(content) = std::fs::read(&source_path) else {
        eprintln!("Failed to read module file: {}", source_path);
        return Err(None);
    };

    let Ok(content_str) = String::from_utf8(content.clone()) else {
        eprintln!("Module file is not valid UTF-8: {}", source_path);
        return Err(None);
    };

    let file_id = files.add(source_path.clone(), content_str.clone());
    let ast = Ast::from_string(&content_str, file_id)?;

    for import in ast.get_imports() {
        // TODO: resolve full path
        let import_path_str = import.get_tail().unwrap();
        let import_path = Path::new(import_path_str).with_extension("mv");

        if !import_path.exists() {
            eprintln!(
                "Dependecy module {} of module {} not found",
                import_path.display(),
                source_path
            );
            return Err(None);
        }

        compile_module(
            import_path_str,
            compiled_modules,
            module_meta_storage,
            parent_stack,
            files,
        )?;
    }

    let bytecode_path_str = ModuleMetadata::source_to_bytecode_path(&source_path);
    let bytecode_path = Path::new(&bytecode_path_str);
    let needs_compilation = module_meta_storage.needs_recompilation(&source_path, &content);

    if needs_compilation || !bytecode_path.exists() {
        let module = Module::from_ast_and_dependencies(ast, compiled_modules)?;
        let bytecode = module.to_bytecode();

        let Ok(_) = std::fs::write(&bytecode_path, bytecode) else {
            eprintln!("Failed to write bytecode file for module {}", source_path);
            return Err(None);
        };

        module_meta_storage.update_hash(&source_path, &content);
        compiled_modules.insert(source_module.to_string(), module);
    } else {
        // load from cached bytecode
        let Ok(bytecode) = std::fs::read(&bytecode_path) else {
            eprintln!("Failed to read bytecode file for module {}", source_path);
            return Err(None);
        };

        let module = Module::from_bytecode_bytes(&bytecode).map_err(|e| {
            eprintln!("Failed to load module {} from bytecode: {}", source_path, e);
            None
        })?;

        compiled_modules.insert(source_module.to_string(), module);
    }

    parent_stack.pop();

    Ok(())
}
