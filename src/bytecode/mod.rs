use std::collections::HashMap;

use crate::{
    executable_module::ExecutableModule,
    function::{InternalFunctionDefinition, InternalFunctionSource},
};
pub use header::BytecodeHeader;
pub use serializable::BytecodeSerializable;

mod header;
mod serializable;
