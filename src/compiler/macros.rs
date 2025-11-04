#[macro_export]
macro_rules! bin_op_pat {
    (COMPARISON) => {
        BinaryOp::Equal | BinaryOp::NotEqual
    };

    (NUMERIC) => {
        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Modulo
    };

    (NUMERIC_COMPARISON) => {
        BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual
    };

    (LOGICAL) => {
        BinaryOp::And | BinaryOp::Or
    };
}

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
                    .unwrap_or_else(|| crate::diagnostics::Span::new(0, 0, 0));
                return Err(crate::errors::CompileError::unexpected_end_of_file_at(
                    __span,
                ));
            }
        };

        let Token {
            kind: __kind,
            span: __span,
        } = __token;

        let $pattern = __kind else {
            return Err(crate::errors::CompileError::unexpected_token_at(
                __kind, __span,
            ));
        };

        $(
            let $span = __span;
        )?
    };
}
