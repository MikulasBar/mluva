use std::path::Path;

use mluva::module::Module;

use crate::{commands::{META_DIR, MODULES_META_FILE}, config::Config, module_metadata::ModuleMetadata};

pub fn command() -> Result<Module, Box<dyn std::error::Error>> {
    println!("Building the Mluva project...");

    let meta_dir = Path::new(META_DIR);

    // Load config
    let config_path = Path::new(super::CONFIG_FILE);
    if !config_path.exists() {
        return Err("No mluva.yaml found. Run 'mluva init' first.".into());
    }

    let config = Config::load_from_file(config_path)?;
    let root_module_path = Path::new(&config.root_module).with_extension("mv");

    if !root_module_path.exists() {
        return Err(format!("Root module '{}' not found", root_module_path.display()).into());
    }

    let modules_dir = meta_dir.join("modules");

    // Load or create module hashes
    let module_meta_file = meta_dir.join(MODULES_META_FILE);
    let mut module_hashes = if module_meta_file.exists() {
        ModuleMetadata::load_from_file(&module_meta_file)?
    } else {
        return Err("Module hashes file not found.".into());
    };

    // Read root module content
    let source_content = std::fs::read(&root_module_path)?;
    let source_path_str = root_module_path.to_string_lossy();

    // Generate bytecode filename
    let bytecode_filename = ModuleMetadata::source_to_bytecode_filename(&source_path_str);
    let bytecode_path = modules_dir.join(&bytecode_filename);

    // Check if we need to recompile
    let needs_compilation = module_hashes.needs_recompilation(&source_path_str, &source_content);

    if needs_compilation || !bytecode_path.exists() {
        println!(
            "Compiling {} -> {}",
            root_module_path.display(),
            bytecode_filename
        );

        // Compile source to bytecode
        let source_str = String::from_utf8(source_content.clone())?;
        let module = Module::from_string(&source_str).map_err(|e| {
            e.to_string()
        })?;

        let bytecode = module.to_bytecode();

        // Write bytecode to modules directory
        if !bytecode_path.exists() {
            let _ = std::fs::File::create(&bytecode_path)?;
        }
        std::fs::write(&bytecode_path, bytecode)?;

        // Update hash
        module_hashes.update_hash(&source_path_str, &source_content);

        println!("Compiled successfully");
        module_hashes.save_to_file(&module_meta_file)?;
        println!("Build completed!");
        Ok(module)
    } else {
        println!("Bytecode is up to date. No recompilation needed.");
        let bytecode = std::fs::read(&bytecode_path)?;
        let module = Module::from_bytecode_bytes(&bytecode).map_err(|e| {
            e.to_string()
        })?;

        println!("Build completed!");
        Ok(module)
    }
}
