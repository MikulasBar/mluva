mod source;
mod external;
mod internal;
mod definition_ref;


pub use source::FunctionSource;
pub use external::{ExternalFunctionDefinition, ExternalFunctionSource};
pub use internal::{InternalFunctionDefinition, InternalFunctionSource};
pub use definition_ref::FunctionDefinitionRef;