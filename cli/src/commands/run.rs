use mluva::runtime::Runtime;

use crate::commands;

pub fn command() -> Result<(), Box<dyn std::error::Error>> {
    let (config, modules) = commands::build::command()?;
    let main_module_name = config.root_module.as_str();
    let main_module = modules
        .get(main_module_name)
        .ok_or(format!("Root module {} not found in compiled modules", main_module_name))?;
    
    println!("Running the Mluva project...");
    
    let runtime = Runtime::new(main_module, &modules);
    let result = runtime.execute()
        .map_err(|e| format!("Failed to execute project: {:?}", e))?;

    println!("Execution result: {:?}", result);

    Ok(())
}