use crate::commands;
use vm::Vm;

pub fn command() -> Result<(), ()> {
    let (config, modules) = commands::build::command()?;
    let main_module_name = &config.root_module;
    let Some(_) = modules.get_slot(&main_module_name) else {
        eprintln!(
            "Root module '{}' not found in compiled modules",
            main_module_name
        );
        return Err(());
    };

    println!("Running the Mluva project...\n");

    let mut vm = Vm::new(modules);
    let result = vm.execute();

    match result {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Runtime error: {}", e);
            return Err(());
        }
    }

    Ok(())
}
