use common::{compile_error::CompileError, module::module_ast::ModuleAST};

use crate::{lexer::tokenize, parser::Parser};

mod lexer;
mod parser;

#[macro_export]
macro_rules! expect_token {
    ($pattern:pat $(,$span:pat)? in $parser:expr) => {
        let __token = match $parser.next() {
            Some(t) => t,
            None => {
                let __span = $parser
                    .peek()
                    .map(|t| t.span)
                    .or_else(|| {
                        $parser
                            .tokens
                            .get($parser.index.saturating_sub(1))
                            .map(|t| t.span)
                    })
                    .unwrap_or_else(|| common::diagnostics::Span::new(0, 0, 0));
                return Err(common::compile_error::CompileError::unexpected_end_of_file(
                    __span.file,
                ));
            }
        };

        let Token {
            kind: __kind,
            span: __span,
        } = __token;

        let $pattern = __kind else {
            return Err(common::compile_error::CompileError::unexpected_token_at(
                __kind, __span,
            ));
        };

        $(
            let $span = __span;
        )?
    };
}

pub fn parse_source(source: &str, file_id: usize) -> Result<ModuleAST, CompileError> {
    let tokens = tokenize(source, file_id)?;
    Parser::new(&tokens, file_id).parse()
}
