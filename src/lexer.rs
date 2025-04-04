use crate::data_type::DataType;
use crate::parse_error::ParseError;
use crate::token::Token;
use crate::str_pat;


pub fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    // println!("Tokenizing input: {:?}", input);
    let mut tokens = vec![];
    let mut chars = input.chars().peekable();
    
    while let Some(&char) = chars.peek() {
        // println!("Current char: {:?}", char);
        let token = match char {
            // comment
            '#' => {
                chars.next();
                
                while chars.peek().is_some() {
                    if let '\n' = chars.peek().unwrap() {
                        break;
                    } else {
                        chars.next();
                    }
                }

                chars.next();
                
                continue;
            }

            '\'' => {
                chars.next();
                let mut string = String::new();
                
                while let Some(&ch) = chars.peek() {
                    if ch == '\'' {
                        chars.next();
                        break;
                    } else {
                        string.push(ch);
                        chars.next();
                    }
                }

                Token::StringLiteral(string)
            } 
            
            ';' | '\n' => {
                chars.next();
                if let Some(&Token::EOL) | None = tokens.last() {
                    continue;
                }
                Token::EOL
            },

            // whitespaces -> skip
            str_pat!(WS) => {
                chars.next();
                continue;
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
                
                Token::Int(number.parse().unwrap())
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

            _ => return Err(ParseError::UnexpectedChar(char)),
        };

        tokens.push(token);
    }

    tokens.push(Token::EOL);

    Ok(tokens)
}


fn match_kw(ident: String) -> Token {
    match ident.as_str() {
        "Int"=> Token::DataType(DataType::Int),
        "Bool"  => Token::DataType(DataType::Bool),
        "String" => Token::DataType(DataType::String),

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