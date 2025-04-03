


#[macro_export]
macro_rules! str_pat {
    (WS) => {
        ' ' | '\n' | '\r' | '\t'
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
    (ANY -> BOOL) => {
        BinOp::Eq
        | BinOp::Neq
    };

    (NUM -> NUM) => {
        BinOp::Add
        | BinOp::Sub
        | BinOp::Mul
        | BinOp::Div
        | BinOp::Modulo
    };
}

#[macro_export]
macro_rules! expect_pat {
    ($pat:pat in ITER $iter:expr) => {
        #[allow(irrefutable_let_patterns)]
        let $pat = (if let Some(_) = $iter.peek() {
            $iter.next().unwrap()
        } else {
            // there is no token to return
            return Err(ParseError::UnexpectedToken(Token::EOF));
        }) else {
            // the token is not the expected one
            return Err(ParseError::UnexpectedToken($iter.next().unwrap()));
        };
    };

    ($pat:pat in VAL $val:expr) => {
        let $pat = $val
            else {panic!()};
    };
}