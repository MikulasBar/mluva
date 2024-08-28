use crate::data_type::DataType;
use crate::token::Token;
use crate::pat;


pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut chars = input.chars().peekable();

    while let Some(&char) = chars.peek() {
        let token = match char {
            // comment
            '#' => {
                chars.next();

                while chars.peek() != Some(&'\n') {
                    chars.next();
                }
                chars.next();
                
                continue;
            }
            
            '{' => {
                chars.next();
                Token::BraceL
            }
            
            
            '}' => {
                chars.next();
                Token::BraceR
            }
            
            ';' => {
                chars.next();
                Token::Semi
            },
            
            // assign / eq
            '=' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    Token::Eq
                } else {
                    Token::Assign
                }
            },
            
            '+' => {
                chars.next();
                Token::Plus
            },
            
            '-' => {
                chars.next();
                Token::Minus
            },
            
            '*' => {
                chars.next();
                Token::Asterisk
            },
            
            '/' => {
                chars.next();
                Token::Slash
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
            
            // whitespaces -> skip
            pat!(WS) => {
                chars.next();
                continue;
            },

            _ => panic!()
        };

        tokens.push(token);
    }

    tokens
}


fn match_kw(ident: String) -> Token {
    match ident.as_str() {
        "number"=> Token::DataType(DataType::Num),
        "bool"  => Token::DataType(DataType::Bool),

        "true"  => Token::Bool(true),
        "false" => Token::Bool(false),
        
        "print" => Token::Print,
        "if"    => Token::If,

        _       => Token::Ident(ident)
    }
}