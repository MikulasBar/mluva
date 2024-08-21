use crate::token::Token;
use crate::pat;


pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut chars = input.chars().peekable();

    while let Some(&char) = chars.peek() {
        let token = match char {
            // whitespaces -> skip
            pat!(WS) => {
                chars.next();
                continue;
            },

            // comment
            '#' => {
                chars.next();

                while chars.peek() != Some(&'#') {
                    chars.next();
                }
                chars.next();
                
                continue;
            }

            // semicolon
            ';' => {
                chars.next();
                Token::Semi
            },

            // assign
            '=' => {
                chars.next();
                Token::Assign
            },

            // plus
            '+' => {
                chars.next();
                Token::Plus
            },

            // number 
            pat!(NUM) => {
                let mut number = String::new();

                while let Some(&digit @ pat!(NUM)) = chars.peek() {
                    number.push(digit);
                    chars.next();
                }

                Token::Num(number.parse().unwrap())
            },

            // identifier
            pat!(IDENT) => {
                let mut ident = String::new();

                while let Some(&ch @ pat!(IDENT)) = chars.peek() {
                    ident.push(ch);
                    chars.next();
                }

                match_kw(ident)
            }

            _ => panic!()
        };

        tokens.push(token);
    }

    tokens
}


fn match_kw(ident: String) -> Token {
    match ident.as_str() {
        "print" => Token::Print,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        _ => Token::Ident(ident)
    }
}