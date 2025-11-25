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
