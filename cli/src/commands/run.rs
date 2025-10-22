use crate::commands;

pub fn command() -> Result<(), Box<dyn std::error::Error>> {
    let module = commands::build::command()?;
    
    println!("Running the Mluva project...");
    let result = module.execute_without_dependencies()
        .map_err(|e| format!("Failed to execute project: {:?}", e))?;

    println!("Execution result: {:?}", result);

    Ok(())
}