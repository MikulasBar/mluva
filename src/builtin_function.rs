use crate::{
    arena::Arena, errors::RuntimeError, string_object::StringObject, value_stack::ValueStack,
};

pub struct BuiltinFunction;

impl BuiltinFunction {
    pub fn execute(
        slot: u32,
        argc: u32,
        value_stack: &mut ValueStack,
        arena: &mut Arena,
    ) -> Result<(), RuntimeError> {
        match slot {
            0 => print(argc, value_stack, arena),
            _ => Err(RuntimeError::Unknown),
        }
    }
}

fn print(argc: u32, value_stack: &mut ValueStack, arena: &mut Arena) -> Result<(), RuntimeError> {
    if argc != 1 {
        return Err(RuntimeError::Other(format!(
            "print expects 1 argument, got {}",
            argc
        )));
    }

    let handle = value_stack.pop()?.as_hhandle();
    let string = arena.get::<StringObject>(&handle)?;

    print!("{}", string.as_str());

    Ok(())
}
