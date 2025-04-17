use super::external::ExternalFunctionSource;
use super::internal::InternalFunctionSource;


#[derive(Debug, Clone)]
pub enum FunctionSource {
    External(ExternalFunctionSource),
    Internal(InternalFunctionSource),
}

mod froms {
    use crate::function::{ExternalFunctionSource, InternalFunctionSource};
    use super::FunctionSource;

    impl From<ExternalFunctionSource> for FunctionSource {
        fn from(source: ExternalFunctionSource) -> Self {
            FunctionSource::External(source)
        }
    }

    impl From<InternalFunctionSource> for FunctionSource {
        fn from(source: InternalFunctionSource) -> Self {
            FunctionSource::Internal(source)
        }
    }
}