use super::data_type::DataType;
use super::token::Token;
use crate::errors::CompileError;
use crate::str_pat;

type CharIter<'a> = std::iter::Peekable<std::str::Chars<'a>>;

pub fn tokenize(input: &str) -> Result<Vec<Token>, CompileError> {
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
            }

            // whitespaces -> skip
            str_pat!(WS) => {
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

            '.' => {
                chars.next();

                if let Some('.') = chars.peek() {
                    chars.next();
                    Token::DotDot
                } else {
                    return Err(CompileError::UnexpectedChar(char));
                }
            }

            '!' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    Token::NotEqual
                } else {
                    Token::Not
                }
            }

            ':' => {
                chars.next();
                Token::Colon
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
            }

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

            '&' => {
                chars.next();
                if let Some('&') = chars.peek() {
                    chars.next();
                    Token::And
                } else {
                    return Err(CompileError::UnexpectedChar(char));
                }
            }

            '|' => {
                chars.next();
                if let Some('|') = chars.peek() {
                    chars.next();
                    Token::Or
                } else {
                    return Err(CompileError::UnexpectedChar(char));
                }
            }

            '+' => {
                chars.next();
                Token::Plus
            }

            '-' => {
                chars.next();
                Token::Minus
            }

            '*' => {
                chars.next();
                Token::Asterisk
            }

            '/' => {
                chars.next();
                Token::Slash
            }

            '%' => {
                chars.next();
                Token::Modulo
            }

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
        match ch {
            '\\' => {
                chars.next(); // consume the backslash
                if let Some(&escaped_char) = chars.peek() {
                    match escaped_char {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        '\'' => string.push('\''),
                        '\\' => string.push('\\'),
                        other => string.push(other),
                    }
                    chars.next(); // consume the escaped character
                }
            }

            '\'' => {
                chars.next(); // consume the closing quote
                break;
            }

            _ => {
                string.push(ch);
                chars.next();
            }
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

    while let Some(&ch @ (str_pat!(IDENT) | str_pat!(NUM))) = chars.peek() {
        ident.push(ch);
        chars.next();
    }

    match_kw(ident)
}

fn match_kw(ident: String) -> Token {
    match ident.as_str() {
        "Int" => Token::DataType(DataType::Int),
        "Float" => Token::DataType(DataType::Float),
        "Bool" => Token::DataType(DataType::Bool),
        "String" => Token::DataType(DataType::String),
        "Void" => Token::DataType(DataType::Void),

        "true" => Token::Bool(true),
        "false" => Token::Bool(false),

        "let" => Token::Let,
        "if" => Token::If,
        "else" => Token::Else,
        "while" => Token::While,
        "return" => Token::Return,
        // "external" => Token::External,
        "import" => Token::Import,

        _ => Token::Ident(ident),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "let x: Int = 42;\nif x > 10 {\nreturn 'Hello';\n}";
        let tokens = tokenize(input).unwrap();
        let expected_tokens = vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Colon,
            Token::DataType(DataType::Int),
            Token::Assign,
            Token::Int(42),
            Token::EOL,
            Token::If,
            Token::Ident("x".to_string()),
            Token::Greater,
            Token::Int(10),
            Token::BraceL,
            Token::EOL,
            Token::Return,
            Token::StringLiteral("Hello".to_string()),
            Token::EOL,
            Token::BraceR,
            Token::EOL,
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_tokenize_string() {
        let input = "'Hello, World!'";
        let mut chars = input.chars().peekable();
        let token = tokenize_string(&mut chars);
        let expected_token = Token::StringLiteral("Hello, World!".to_string());
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_tokenize_number() {
        let input = "12345";
        let mut chars = input.chars().peekable();
        let token_int = tokenize_number(&mut chars);
        let expected_token_int = Token::Int(12345);
        assert_eq!(token_int, expected_token_int);

        let input_float = "123.45";
        let mut chars_float = input_float.chars().peekable();
        let token_float = tokenize_number(&mut chars_float);
        let expected_token_float = Token::Float(123.45);
        assert_eq!(token_float, expected_token_float);
    }
}
