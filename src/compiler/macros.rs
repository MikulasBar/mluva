#[macro_export]
macro_rules! str_pat {
    (WS) => {
        ' ' | '\n' | '\r' | '\t'
    };

    (EOL) => {
        '\n' | '\r'
    };

    (IDENT) => {
        '_' | 'a'..='z' | 'A'..='Z'
    };

    (NUM) => {
        '0'..='9'
    };
}

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
macro_rules! expect_pat {
    ($pat:pat in $iter:expr) => {
        #[allow(irrefutable_let_patterns)]
        let $pat = (if let Some(_) = $iter.peek() {
            $iter.next().unwrap()
        } else {
            // there is no token to return
            return Err(CompileError::UnexpectedToken(Token::EOF));
        }) else {
            // the token is not the expected one
            return Err(CompileError::UnexpectedToken($iter.next().unwrap()));
        };
    };
}

pub use {bin_op_pat, expect_pat, str_pat};
