


#[macro_export]
macro_rules! pat {
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
macro_rules! expect_pat {
    ($pat:pat in $iter:expr) => {
        let $pat = $iter.next().unwrap()
            else {panic!()};
    };
}