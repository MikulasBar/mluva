use std::{collections::HashMap, path::Path};

use mluva::{ast::Ast, module::Module};

use crate::{
    commands::{META_DIR, MODULES_META_FILE},
    config::Config,
    module_metadata::ModuleMetadata,
};

pub fn command() -> Result<(Config, HashMap<String, Module>), Box<dyn std::error::Error>> {
    println!("Building the Mluva project...");

    let meta_dir = Path::new(META_DIR);

    let config_path = Path::new(super::CONFIG_FILE);
    if !config_path.exists() {
        return Err("No mluva.yaml found. Run 'mluva init' first.".into());
    }

    let config = Config::load_from_file(config_path)?;
    let root_module_path = Path::new(&config.root_module).with_extension("mv");

    if !root_module_path.exists() {
        return Err(format!("Root module '{}' not found", root_module_path.display()).into());
    }

    let module_meta_file = meta_dir.join(MODULES_META_FILE);
    let mut module_hashes = if module_meta_file.exists() {
        ModuleMetadata::load_from_file(&module_meta_file)?
    } else {
        return Err("Module hashes file not found.".into());
    };

    let mut compiled_modules: HashMap<String, Module> = HashMap::new();
    let mut parent_stack: Vec<String> = vec![];

    compile_module(
        &config.root_module,
        &mut compiled_modules,
        &mut module_hashes,
        &mut parent_stack,
    )?;
    
    module_hashes.save_to_file(&module_meta_file)?;
    println!("Build completed!");

    Ok((config, compiled_modules))
}

fn compile_module(
    source_module: &str,
    compiled_modules: &mut HashMap<String, Module>,
    module_metadata: &mut ModuleMetadata,
    parent_stack: &mut Vec<String>, // TODO: change to something that is not O(n) on search but has ordering
) -> Result<(), Box<dyn std::error::Error>> {
    let source_path = Path::new(source_module).with_extension("mv")
        .to_string_lossy()
        .to_string();

    if compiled_modules.contains_key(source_module) {
        return Ok(());
    }

    if parent_stack.iter().any(|p| p == source_module) {
        return Err(format!(
            "Cyclic dependency detected: {} -> {}",
            parent_stack.join(" -> "),
            source_module
        )
        .into());
    }

    parent_stack.push(source_module.to_string());

    let content = std::fs::read(&source_path)?;
    let content_str = String::from_utf8(content.clone())?;

    let ast = Ast::from_string(&content_str).map_err(|e| e.to_string())?;

    for import in ast.get_imports() {
        // TODO: resolve full path
        let import_path_str = import.get_tail().unwrap();
        let import_path = Path::new(import_path_str).with_extension("mv");
        
        if !import_path.exists() {
            return Err(format!("Dependecy module {} of module {} not found", import_path.display(), source_path).into());
        }

        compile_module(
            import_path_str,
            compiled_modules,
            module_metadata,
            parent_stack,
        )?;
    }

    let modules_dir = Path::new(META_DIR).join("modules");
    
    std::fs::create_dir_all(&modules_dir)?;
    
    let bytecode_filename = ModuleMetadata::source_to_bytecode_filename(&source_path);
    let bytecode_path = modules_dir.join(&bytecode_filename);

    let needs_compilation = module_metadata.needs_recompilation(&source_path, &content);
    
    if needs_compilation || !bytecode_path.exists() {
        let module = Module::from_ast_and_dependencies(ast, compiled_modules)
            .map_err(|e| e.to_string())?;

        let bytecode = module.to_bytecode();

        std::fs::write(&bytecode_path, bytecode)?;

        module_metadata.update_hash(&source_path, &content);
        compiled_modules.insert(source_module.to_string(), module);
    } else {
        // load from cached bytecode
        let bytecode = std::fs::read(&bytecode_path)?;
        let module = Module::from_bytecode_bytes(&bytecode)
            .map_err(|e| e.to_string())?;
        compiled_modules.insert(source_module.to_string(), module);
    }

    parent_stack.pop();

    Ok(())
}
