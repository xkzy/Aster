use crate::lexer::{LexError, Token, TokenKind, lex};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub result: Type,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    I32,
    Bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt {
    Return(Expr),
    Bind {
        mutable: bool,
        name: String,
        ty: Option<Type>,
        value: Expr,
    },
    Assign {
        name: String,
        value: Expr,
    },
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Integer(String),
    Bool(bool),
    Name(String),
    Unary(UnOp, Box<Expr>),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnOp {
    Negate,
    Not,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Equal,
    Less,
    LessEqual,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    pub offset: usize,
    pub message: String,
}

pub fn parse(source: &str) -> Result<Vec<Function>, ParseError> {
    let tokens = lex(source).map_err(|e: LexError| ParseError {
        offset: e.offset,
        message: e.message.into(),
    })?;
    Parser { tokens, pos: 0 }.program()
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn program(mut self) -> Result<Vec<Function>, ParseError> {
        let mut functions = Vec::new();
        while !self.at(&TokenKind::Eof) {
            functions.push(self.function()?);
        }
        Ok(functions)
    }

    fn function(&mut self) -> Result<Function, ParseError> {
        if self.eat(&TokenKind::Fn) {
            let name = self.identifier()?;
            self.expect(&TokenKind::LParen, "expected `(` after function name")?;
            let mut params = Vec::new();
            if !self.at(&TokenKind::RParen) {
                loop {
                    let param = self.identifier()?;
                    self.expect(&TokenKind::Colon, "expected `:` after parameter name")?;
                    params.push((param, self.ty()?));
                    if !self.eat(&TokenKind::Comma) {
                        break;
                    }
                }
            }
            self.expect(&TokenKind::RParen, "expected `)`")?;
            self.expect(&TokenKind::Arrow, "expected `->` before return type")?;
            let result = self.ty()?;
            return self.finish_function(name, params, result);
        }
        let result = self.ty()?;
        let name = self.identifier()?;
        self.expect(&TokenKind::LParen, "expected `(` after function name")?;
        let mut params = Vec::new();
        if !self.at(&TokenKind::RParen) {
            loop {
                let ty = self.ty()?;
                let param = self.identifier()?;
                params.push((param, ty));
                if !self.eat(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(&TokenKind::RParen, "expected `)`")?;
        self.finish_function(name, params, result)
    }

    fn finish_function(
        &mut self,
        name: String,
        params: Vec<(String, Type)>,
        result: Type,
    ) -> Result<Function, ParseError> {
        self.expect(&TokenKind::LBrace, "expected function body")?;
        let mut body = Vec::new();
        while !self.at(&TokenKind::RBrace) {
            if self.at(&TokenKind::Eof) {
                return self.fail("unterminated function body");
            }
            body.push(self.statement()?);
        }
        self.bump();
        Ok(Function {
            name,
            params,
            result,
            body,
        })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.eat(&TokenKind::Return) {
            let value = self.expression(0)?;
            self.expect(&TokenKind::Semicolon, "expected `;` after return")?;
            return Ok(Stmt::Return(value));
        }
        if self.eat(&TokenKind::Let) || self.eat(&TokenKind::Var) {
            let mutable = self.previous().kind == TokenKind::Var;
            let name = self.identifier()?;
            let ty = if self.eat(&TokenKind::Colon) {
                Some(self.ty()?)
            } else {
                None
            };
            self.expect(&TokenKind::Equal, "expected `=` in binding")?;
            let value = self.expression(0)?;
            self.expect(&TokenKind::Semicolon, "expected `;` after binding")?;
            return Ok(Stmt::Bind {
                mutable,
                name,
                ty,
                value,
            });
        }
        if let TokenKind::Ident(name) = self.peek().kind.clone()
            && self
                .tokens
                .get(self.pos + 1)
                .is_some_and(|t| t.kind == TokenKind::Equal)
        {
            self.bump();
            self.bump();
            let value = self.expression(0)?;
            self.expect(&TokenKind::Semicolon, "expected `;` after assignment")?;
            return Ok(Stmt::Assign { name, value });
        }
        let expr = self.expression(0)?;
        self.expect(&TokenKind::Semicolon, "expected `;` after expression")?;
        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
        let token = self.bump().clone();
        let mut lhs = match token.kind {
            TokenKind::Integer(value) => Expr::Integer(value),
            TokenKind::True => Expr::Bool(true),
            TokenKind::False => Expr::Bool(false),
            TokenKind::Ident(name) if self.eat(&TokenKind::LParen) => {
                let mut args = Vec::new();
                if !self.at(&TokenKind::RParen) {
                    loop {
                        args.push(self.expression(0)?);
                        if !self.eat(&TokenKind::Comma) {
                            break;
                        }
                    }
                }
                self.expect(&TokenKind::RParen, "expected `)` after arguments")?;
                Expr::Call(name, args)
            }
            TokenKind::Ident(name) => Expr::Name(name),
            TokenKind::Minus => Expr::Unary(UnOp::Negate, Box::new(self.expression(7)?)),
            TokenKind::Bang => Expr::Unary(UnOp::Not, Box::new(self.expression(7)?)),
            TokenKind::LParen => {
                let e = self.expression(0)?;
                self.expect(&TokenKind::RParen, "expected `)`")?;
                e
            }
            _ => return self.fail("expected expression"),
        };
        loop {
            let (op, left_bp, right_bp) = match self.peek().kind {
                TokenKind::EqualEqual => (BinOp::Equal, 1, 2),
                TokenKind::Less => (BinOp::Less, 1, 2),
                TokenKind::LessEqual => (BinOp::LessEqual, 1, 2),
                TokenKind::Plus => (BinOp::Add, 3, 4),
                TokenKind::Minus => (BinOp::Subtract, 3, 4),
                TokenKind::Star => (BinOp::Multiply, 5, 6),
                TokenKind::Slash => (BinOp::Divide, 5, 6),
                TokenKind::Percent => (BinOp::Remainder, 5, 6),
                _ => break,
            };
            if left_bp < min_bp {
                break;
            }
            self.bump();
            let rhs = self.expression(right_bp)?;
            lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
        }
        Ok(lhs)
    }

    fn ty(&mut self) -> Result<Type, ParseError> {
        match self.bump().kind {
            TokenKind::Int | TokenKind::I32 => Ok(Type::I32),
            TokenKind::Bool => Ok(Type::Bool),
            _ => self.fail("expected type (`int`, `i32`, or `bool`"),
        }
    }
    fn identifier(&mut self) -> Result<String, ParseError> {
        match self.bump().kind.clone() {
            TokenKind::Ident(s) => Ok(s),
            _ => self.fail("expected identifier"),
        }
    }
    fn expect(&mut self, kind: &TokenKind, message: &'static str) -> Result<(), ParseError> {
        if self.eat(kind) {
            Ok(())
        } else {
            self.fail(message)
        }
    }
    fn eat(&mut self, kind: &TokenKind) -> bool {
        if self.at(kind) {
            self.bump();
            true
        } else {
            false
        }
    }
    fn at(&self, kind: &TokenKind) -> bool {
        self.peek().kind == *kind
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }
    fn bump(&mut self) -> &Token {
        let i = self.pos;
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        &self.tokens[i]
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.pos.saturating_sub(1)]
    }
    fn fail<T>(&self, message: impl Into<String>) -> Result<T, ParseError> {
        Err(ParseError {
            offset: self.peek().offset,
            message: message.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Function, Type, parse};

    #[test]
    fn both_function_styles_parse_to_equivalent_ast() {
        let modern = parse("fn add(a: i32, b: i32) -> i32 { return a + b; }").unwrap();
        let c_style = parse("int add(int a, int b) { return a + b; }").unwrap();
        assert_eq!(modern, c_style);
        assert_eq!(
            modern[0],
            Function {
                name: "add".into(),
                params: vec![("a".into(), Type::I32), ("b".into(), Type::I32)],
                result: Type::I32,
                body: vec![super::Stmt::Return(super::Expr::Binary(
                    Box::new(super::Expr::Name("a".into())),
                    super::BinOp::Add,
                    Box::new(super::Expr::Name("b".into())),
                ))],
            }
        );
    }

    #[test]
    fn malformed_and_unsupported_syntax_is_rejected() {
        assert!(parse("fn f( -> i32 {}").is_err());
        assert!(parse("fn f() -> i32 { while true {} }").is_err());
    }
}
