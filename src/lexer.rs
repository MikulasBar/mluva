use crate::data_type::DataType;
use crate::parse_error::ParseError;
use crate::token::Token;
use crate::str_pat;


pub fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
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
            
            ';' | '\n' => {
                chars.next();
                Token::EOL
            },

            '{' => {
                chars.next();
                Token::BraceL
            }
            
            '}' => {
                chars.next();
                Token::BraceR
            }

            '(' => {
                chars.next();
                Token::ParenL
            }
            
            ')' => {
                chars.next();
                Token::ParenR
            }

            '[' => {
                chars.next();
                Token::BracketL
            }
            
            ']' => {
                chars.next();
                Token::BracketR
            }

            '!' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    Token::Neq
                } else {
                    return Err(ParseError::UnexpectedChar(char));
                }
            }
            
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

            '%' => {
                chars.next();
                Token::Percentage
            },
            
            // number 
            str_pat!(NUM) => {
                let mut number = String::new();
                
                while let Some(&digit @ str_pat!(NUM)) = chars.peek() {
                    number.push(digit);
                    chars.next();
                }
                
                Token::Num(number.parse().unwrap())
            },
            
            // identifier
            str_pat!(IDENT) => {
                let mut ident = String::new();
                
                while let Some(&ch @ str_pat!(IDENT)) = chars.peek() {
                    ident.push(ch);
                    chars.next();
                }
                
                match_kw(ident)
            }
            
            // whitespaces -> skip
            str_pat!(WS) => {
                chars.next();
                continue;
            },

            _ => return Err(ParseError::UnexpectedChar(char)),
        };

        tokens.push(token);
    }

    Ok(tokens)
}


fn match_kw(ident: String) -> Token {
    match ident.as_str() {
        "Number"=> Token::DataType(DataType::Num),
        "Bool"  => Token::DataType(DataType::Bool),

        "true"  => Token::Bool(true),
        "false" => Token::Bool(false),
        
        "let"   => Token::Let,
        "print" => Token::Print,
        "if"    => Token::If,
        // "else"  => Token::Else, 
        "while" => Token::While,

        _       => Token::Ident(ident)
    }
}