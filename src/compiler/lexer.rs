use super::data_type::DataType;
use crate::errors::CompileError;
use super::token::Token;
use crate::str_pat;

type CharIter<'a> = std::iter::Peekable<std::str::Chars<'a>>;

pub fn tokenize(input: &str) -> Result<Vec<Token>, CompileError> {
    // println!("Tokenizing input: {:?}", input);
    let mut tokens = vec![];
    let mut chars = input.chars().peekable();
    
    while let Some(&char) = chars.peek() {
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

            '\'' => tokenize_string(&mut chars),
            
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

            ',' => {
                chars.next();
                Token::Comma
            }

            '!' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    Token::NotEqual
                } else {
                    return Err(CompileError::UnexpectedChar(char));
                }
            }
            
            // assign / eq
            '=' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    Token::Equal
                } else {
                    Token::Assign
                }
            },

            '>' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }

            '<' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            
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
                Token::Modulo
            },
            
            str_pat!(NUM) => tokenize_number(&mut chars),
            str_pat!(IDENT) => tokenize_ident(&mut chars),

            _ => return Err(CompileError::UnexpectedChar(char)),
        };

        tokens.push(token);
    }

    tokens.push(Token::EOL);

    Ok(tokens)
}


fn tokenize_string(chars: &mut CharIter) -> Token {
    chars.next(); // consume the opening quote
    let mut string = String::new();
    
    while let Some(&ch) = chars.peek() {
        if ch == '\'' {
            chars.next(); // consume the closing quote
            break;
        } else {
            string.push(ch);
            chars.next();
        }
    }

    Token::StringLiteral(string)
}

fn tokenize_number(chars: &mut CharIter) -> Token {
    let mut number = String::new();
    
    while let Some(&digit @ str_pat!(NUM)) = chars.peek() {
        number.push(digit);
        chars.next();
    }

    if let Some('.') = chars.peek() {
        chars.next();
        number.push('.');

        while let Some(&digit @ str_pat!(NUM)) = chars.peek() {
            number.push(digit);
            chars.next();
        }

        Token::Float(number.parse().unwrap())
    } else {
        Token::Int(number.parse().unwrap())
    }
}

fn tokenize_ident(chars: &mut CharIter) -> Token {
    let mut ident = String::new();
    
    while let Some(&ch @ str_pat!(IDENT)) = chars.peek() {
        ident.push(ch);
        chars.next();
    }

    match_kw(ident)
}


fn match_kw(ident: String) -> Token {
    match ident.as_str() {
        "Int" => Token::DataType(DataType::Int),
        "Float" => Token::DataType(DataType::Float),
        "Bool"  => Token::DataType(DataType::Bool),
        "String" => Token::DataType(DataType::String),

        "true"  => Token::Bool(true),
        "false" => Token::Bool(false),
        
        "let"   => Token::Let,
        "if"    => Token::If,
        "else"  => Token::Else,
        "while" => Token::While,

        _       => Token::Ident(ident)
    }
}