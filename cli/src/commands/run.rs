use mluva::runtime::Runtime;

use crate::commands;

pub fn command() -> Result<(), ()> {
    let (config, modules) = commands::build::command()?;
    let main_module_name = &config.root_module;
    let Some(main_module) = modules.get(main_module_name) else {
        eprintln!(
            "Root module '{}' not found in compiled modules",
            main_module_name
        );
        return Err(());
    };

    println!("Running the Mluva project...\n");

    let runtime = Runtime::new(main_module, &modules);
    let result = runtime.execute();

    match result {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Runtime error: {}", e);
            return Err(());
        }
    }

    Ok(())
}
