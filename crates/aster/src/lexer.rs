#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub offset: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Ident(String),
    Integer(String),
    Fn,
    Let,
    Var,
    Return,
    True,
    False,
    Int,
    I32,
    Bool,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Comma,
    Semicolon,
    Arrow,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bang,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Eof,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexError {
    pub offset: usize,
    pub message: &'static str,
}

pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let start = i;
        let b = bytes[i];
        if b.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if b.is_ascii_alphabetic() || b == b'_' {
            i += 1;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let word = &source[start..i];
            let kind = match word {
                "fn" => TokenKind::Fn,
                "let" => TokenKind::Let,
                "var" => TokenKind::Var,
                "return" => TokenKind::Return,
                "true" => TokenKind::True,
                "false" => TokenKind::False,
                "int" => TokenKind::Int,
                "i32" => TokenKind::I32,
                "bool" => TokenKind::Bool,
                _ => TokenKind::Ident(word.to_owned()),
            };
            tokens.push(Token {
                kind,
                offset: start,
            });
            continue;
        }
        if b.is_ascii_digit() {
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            tokens.push(Token {
                kind: TokenKind::Integer(source[start..i].to_owned()),
                offset: start,
            });
            continue;
        }
        let (kind, width) = match b {
            b'(' => (TokenKind::LParen, 1),
            b')' => (TokenKind::RParen, 1),
            b'{' => (TokenKind::LBrace, 1),
            b'}' => (TokenKind::RBrace, 1),
            b':' => (TokenKind::Colon, 1),
            b',' => (TokenKind::Comma, 1),
            b';' => (TokenKind::Semicolon, 1),
            b'+' => (TokenKind::Plus, 1),
            b'*' => (TokenKind::Star, 1),
            b'/' => (TokenKind::Slash, 1),
            b'%' => (TokenKind::Percent, 1),
            b'-' if bytes.get(i + 1) == Some(&b'>') => (TokenKind::Arrow, 2),
            b'-' => (TokenKind::Minus, 1),
            b'!' => (TokenKind::Bang, 1),
            b'=' if bytes.get(i + 1) == Some(&b'=') => (TokenKind::EqualEqual, 2),
            b'=' => (TokenKind::Equal, 1),
            b'<' if bytes.get(i + 1) == Some(&b'=') => (TokenKind::LessEqual, 2),
            b'<' => (TokenKind::Less, 1),
            _ => {
                return Err(LexError {
                    offset: i,
                    message: "unexpected character",
                });
            }
        };
        tokens.push(Token {
            kind,
            offset: start,
        });
        i += width;
    }
    tokens.push(Token {
        kind: TokenKind::Eof,
        offset: source.len(),
    });
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::{TokenKind, lex};

    #[test]
    fn lexes_modern_function_tokens() {
        let tokens = lex("fn f(x: i32) -> i32 { return x + 1; }").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Fn);
        assert_eq!(tokens[1].kind, TokenKind::Ident("f".into()));
        assert_eq!(
            tokens.iter().filter(|t| t.kind == TokenKind::Plus).count(),
            1
        );
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn rejects_unknown_characters_with_byte_offset() {
        let error = lex("@").unwrap_err();
        assert_eq!(error.offset, 0);
    }
}
