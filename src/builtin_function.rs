use crate::{errors::RuntimeError, string_object::StringObject, vm::Vm};

pub struct BuiltinFunction;

impl BuiltinFunction {
    pub fn execute(slot: u32, argc: u32, vm: &mut Vm) -> Result<(), RuntimeError> {
        match slot {
            0 => execute_print(argc, vm),
            _ => Err(RuntimeError::Unknown),
        }
    }
}

fn execute_print(argc: u32, vm: &mut Vm) -> Result<(), RuntimeError> {
    if argc != 1 {
        return Err(RuntimeError::Other(format!(
            "print expects 1 argument, got {}",
            argc
        )));
    }

    let handle = vm.pop()?.as_hhandle();
    let string = vm.arena.get::<StringObject>(&handle)?;

    print!("{}", string.as_str());

    Ok(())
}
