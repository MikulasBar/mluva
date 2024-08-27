


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
    ($pat:pat in ITER $iter:expr) => {
        let $pat = $iter.next().unwrap()
            else {panic!()};
    };

    ($pat:pat in MAP $map:expr; $key:expr) => {
        let $pat = $map.get($key).unwrap()
            else {panic!()};
    };

    ($pat:pat in VAL $val:expr) => {
        let $pat = $val
            else {panic!()};
    };
}