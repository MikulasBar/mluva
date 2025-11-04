use std::iter::Peekable;
use std::str::CharIndices;

use super::data_type::DataType;
use super::token::{Token, TokenKind};
use crate::diagnostics::Span;
use crate::errors::CompileError;

/// Tokenize input and attach byte-span info to each token.
/// Returns Vec<Token> where each token carries a Span { file, lo, hi }.
/// `file_id` should come from your SimpleFiles / SourceMap (codespan-reporting).
pub fn tokenize(file_id: usize, input: &str) -> Result<Vec<Token>, CompileError> {
    let mut tokens: Vec<Token> = vec![];
    let mut chars = input.char_indices().peekable();

    while let Some((start_idx, ch)) = chars.peek().cloned() {
        if let Some(token) = get_single_char_token(&mut chars, file_id) {
            tokens.push(token);
            continue;
        }

        let token = match ch {
            '#' => {
                chars.next();
                while let Some(&(_, c)) = chars.peek() {
                    if c == '\n' {
                        break;
                    }
                    chars.next();
                }

                if let Some(&(_, '\n')) = chars.peek() {
                    chars.next();
                }

                continue;
            }

            '\'' => tokenize_string(&mut chars, file_id, input),

            ';' | '\n' => {
                // EOL token; avoid duplicate sequential EOLs
                chars.next();
                if let Some(last) = tokens.last() {
                    if let TokenKind::EOL = last.kind {
                        continue;
                    }
                }
                let end = start_idx + ch.len_utf8();
                Ok(Token::new(
                    TokenKind::EOL,
                    Span::new(file_id, start_idx, end),
                ))
            }

            // whitespaces -> skip
            ch if ch.is_whitespace() => {
                chars.next();
                continue;
            }

            '.' => consume_optional_double(
                &mut chars,
                file_id,
                start_idx,
                '.',
                TokenKind::EOF,
                TokenKind::DotDot,
                true,
            ),

            '!' => consume_optional_double(
                &mut chars,
                file_id,
                start_idx,
                '=',
                TokenKind::Not,
                TokenKind::NotEqual,
                false,
            ),

            '=' => consume_optional_double(
                &mut chars,
                file_id,
                start_idx,
                '=',
                TokenKind::Assign,
                TokenKind::Equal,
                false,
            ),

            '<' => consume_optional_double(
                &mut chars,
                file_id,
                start_idx,
                '=',
                TokenKind::Less,
                TokenKind::LessEqual,
                false,
            ),

            '>' => consume_optional_double(
                &mut chars,
                file_id,
                start_idx,
                '=',
                TokenKind::Greater,
                TokenKind::GreaterEqual,
                false,
            ),

            '&' => consume_optional_double(
                &mut chars,
                file_id,
                start_idx,
                '&',
                TokenKind::And,
                TokenKind::And,
                true,
            ),

            '|' => consume_optional_double(
                &mut chars,
                file_id,
                start_idx,
                '|',
                TokenKind::Or,
                TokenKind::Or,
                true,
            ),

            ch if ch.is_ascii_digit() => tokenize_number(&mut chars, file_id),
            ch if ch.is_ascii_alphabetic() || ch == '_' => tokenize_ident(&mut chars, file_id),

            _ => {
                return Err(CompileError::unexpected_char_at(
                    ch,
                    Span::new(file_id, start_idx, start_idx + ch.len_utf8()),
                ));
            }
        };

        match token {
            Ok(t) => tokens.push(t),
            Err(e) => return Err(e),
        }
    }

    // push EOL at EOF
    let eof = input.len();
    tokens.push(Token::new(TokenKind::EOL, Span::new(file_id, eof, eof)));
    Ok(tokens)
}

fn get_single_char_token(chars: &mut Peekable<CharIndices<'_>>, file_id: usize) -> Option<Token> {
    // Peek the current char without consuming it.
    let (start_idx, ch) = match chars.peek().cloned() {
        Some(pair) => pair,
        None => return None,
    };

    let token = match ch {
        '+' => TokenKind::Plus,
        '-' => TokenKind::Minus,
        '*' => TokenKind::Asterisk,
        '/' => TokenKind::Slash,
        '%' => TokenKind::Modulo,
        '(' => TokenKind::ParenL,
        ')' => TokenKind::ParenR,
        '[' => TokenKind::BracketL,
        ']' => TokenKind::BracketR,
        '{' => TokenKind::BraceL,
        '}' => TokenKind::BraceR,
        ',' => TokenKind::Comma,
        ':' => TokenKind::Colon,
        _ => return None,
    };

    // consume the char we just peeked
    chars.next();
    let end = start_idx + ch.len_utf8();
    Some(Token::new(token, Span::new(file_id, start_idx, end)))
}

fn consume_optional_double(
    chars: &mut Peekable<CharIndices<'_>>,
    file_id: usize,
    start_idx: usize,
    expected_second: char,
    single_tok: TokenKind,
    double_tok: TokenKind,
    require_double: bool,
) -> Result<Token, CompileError> {
    let (_, first_ch) = chars.next().unwrap();
    match chars.peek() {
        None if require_double => {
            return Err(CompileError::unexpected_end_of_file_at(Span::new(
                file_id,
                start_idx + first_ch.len_utf8(),
                start_idx + first_ch.len_utf8(),
            )));
        }
        Some(&(_, ch2)) if ch2 != expected_second && require_double => {
            return Err(CompileError::unexpected_char_at(
                ch2,
                Span::new(
                    file_id,
                    start_idx + first_ch.len_utf8(),
                    start_idx + first_ch.len_utf8() + ch2.len_utf8(),
                ),
            ));
        }
        Some(&(_, ch2)) if ch2 == expected_second => {
            // double char token
            chars.next(); // consume second char
            let end = start_idx + first_ch.len_utf8() + ch2.len_utf8();
            Ok(Token::new(double_tok, Span::new(file_id, start_idx, end)))
        }
        None | Some(&(_, _)) => {
            // single char token
            let end = start_idx + first_ch.len_utf8();
            Ok(Token::new(single_tok, Span::new(file_id, start_idx, end)))
        }
    }
}

/// On EOF without closing quote we return a token spanning until EOF (to preserve behaviour).
fn tokenize_string(
    chars: &mut Peekable<CharIndices<'_>>,
    file_id: usize,
    input: &str,
) -> Result<Token, CompileError> {
    // consume opening quote
    let (start_idx, _quote) = chars.next().unwrap();
    let mut string = String::new();
    let mut last_hi = start_idx + 1;

    while let Some(&(idx, ch)) = chars.peek() {
        match ch {
            '\\' => {
                chars.next();
                if let Some(&(esc_idx, escaped)) = chars.peek() {
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        '\'' => string.push('\''),
                        '\\' => string.push('\\'),
                        other => string.push(other),
                    }
                    last_hi = esc_idx + escaped.len_utf8();
                    chars.next(); // consume escaped
                } else {
                    // escape at EOF: treat as unterminated -> span to EOF
                    let end = input.len();
                    return Ok(Token::new(
                        TokenKind::StringLiteral(string),
                        Span::new(file_id, start_idx, end),
                    ));
                }
            }

            '\'' => {
                // closing quote at idx
                let end = idx + 1;
                chars.next(); // consume closing
                last_hi = end;
                return Ok(Token::new(
                    TokenKind::StringLiteral(string),
                    Span::new(file_id, start_idx, last_hi),
                ));
            }

            _ => {
                string.push(ch);
                last_hi = idx + ch.len_utf8();
                chars.next();
            }
        }
    }

    // EOF reached without closing quote -> span to EOF (preserve previous behaviour)
    let end = input.len();
    Ok(Token::new(
        TokenKind::StringLiteral(string),
        Span::new(file_id, start_idx, end),
    ))
}

fn tokenize_number(
    chars: &mut Peekable<CharIndices<'_>>,
    file_id: usize,
) -> Result<Token, CompileError> {
    let (start_idx, _) = chars.peek().cloned().unwrap();
    let mut num = String::new();
    let mut span = Span::new(file_id, start_idx, start_idx);

    while let Some(&(i, c)) = chars.peek() {
        if c.is_ascii_digit() {
            num.push(c);
            span.hi = i + c.len_utf8();
            chars.next();
        } else {
            break;
        }
    }

    if let Some(&(_, '.')) = chars.peek() {
        num.push('.');
        if let Some(&(dot_idx, dot_ch)) = chars.peek() {
            span.hi = dot_idx + dot_ch.len_utf8();
        }
        chars.next();

        while let Some(&(i, c)) = chars.peek() {
            if c.is_ascii_digit() {
                num.push(c);
                span.hi = i + c.len_utf8();
                chars.next();
            } else {
                break;
            }
        }

        match num.parse::<f64>() {
            Ok(f) => Ok(Token::new(TokenKind::Float(f), span)),
            Err(_) => Err(CompileError::unexpected_char_at('.', span)),
        }
    } else {
        match num.parse::<i32>() {
            Ok(i) => Ok(Token::new(TokenKind::Int(i), span)),
            Err(_) => Err(CompileError::unexpected_char_at('0', span)),
        }
    }
}

fn tokenize_ident(
    chars: &mut Peekable<CharIndices<'_>>,
    file_id: usize,
) -> Result<Token, CompileError> {
    let (start_idx, _) = chars.peek().cloned().unwrap();
    let mut ident = String::new();
    let mut end = start_idx;

    while let Some(&(i, c)) = chars.peek() {
        if c.is_ascii_alphanumeric() || c == '_' {
            ident.push(c);
            end = i + c.len_utf8();
            chars.next();
        } else {
            break;
        }
    }

    let tok = match_kw(ident);
    Ok(Token::new(tok, Span::new(file_id, start_idx, end)))
}

fn match_kw(ident: String) -> TokenKind {
    match ident.as_str() {
        "Int" => TokenKind::DataType(DataType::Int),
        "Float" => TokenKind::DataType(DataType::Float),
        "Bool" => TokenKind::DataType(DataType::Bool),
        "String" => TokenKind::DataType(DataType::String),
        "Void" => TokenKind::DataType(DataType::Void),

        "true" => TokenKind::Bool(true),
        "false" => TokenKind::Bool(false),

        "let" => TokenKind::Let,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "while" => TokenKind::While,
        "return" => TokenKind::Return,
        // "external" => Token::External,
        "import" => TokenKind::Import,

        _ => TokenKind::Ident(ident),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "let x: Int = 42;\nif x > 10 {\nreturn 'Hello';\n}";
        let tokens = tokenize(0, input).unwrap();
        let expected_tokens = vec![
            TokenKind::Let,
            TokenKind::Ident("x".to_string()),
            TokenKind::Colon,
            TokenKind::DataType(DataType::Int),
            TokenKind::Assign,
            TokenKind::Int(42),
            TokenKind::EOL,
            TokenKind::If,
            TokenKind::Ident("x".to_string()),
            TokenKind::Greater,
            TokenKind::Int(10),
            TokenKind::BraceL,
            TokenKind::EOL,
            TokenKind::Return,
            TokenKind::StringLiteral("Hello".to_string()),
            TokenKind::EOL,
            TokenKind::BraceR,
            TokenKind::EOL,
        ];

        let got_kinds: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(got_kinds, expected_tokens);
    }

    #[test]
    fn test_tokenize_string() {
        let input = "'Hello, World!'";
        let mut chars = input.char_indices().peekable();
        // call helper directly to get a spanned token
        let token = tokenize_string(&mut chars, 0, input).unwrap();
        let expected_token = TokenKind::StringLiteral("Hello, World!".to_string());
        assert_eq!(token.kind, expected_token);
    }

    #[test]
    fn test_tokenize_number() {
        let input = "12345";
        let mut chars = input.char_indices().peekable();
        let token_int = tokenize_number(&mut chars, 0).unwrap();
        match token_int.kind {
            TokenKind::Int(v) => assert_eq!(v, 12345),
            _ => panic!("expected int"),
        }

        let input_float = "123.45";
        let mut chars_float = input_float.char_indices().peekable();
        let token_float = tokenize_number(&mut chars_float, 0).unwrap();
        match token_float.kind {
            TokenKind::Float(v) => assert!((v - 123.45).abs() < 1e-9),
            _ => panic!("expected float"),
        }
    }
}
