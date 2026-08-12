use anyhow::Error;
use crate::lexer::Token;
use crate::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn parse(&mut self) -> anyhow::Result<Statements> {
        let mut stmts = Statements::new();

        while self.peek().is_some() {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        }

        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> anyhow::Result<Stmt> {
        let expr = self.parse_expr()?;

        if !self.consume(Token::Semicolon) {
            return Err(Error::msg("式の末尾がセミコロンでありません。"));
        }

        Ok(Stmt::Expr(expr))
    }

    fn parse_expr(&mut self) -> anyhow::Result<Expr> {
        self.expr_add()
    }

    // 加算・減算処理
    fn expr_add(&mut self) -> anyhow::Result<Expr> {
        let mut left = self.expr_mul()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Plus => {
                    self.next();

                    let right = self.expr_mul()?;

                    let binary = Binary::new(
                        Box::new(left),
                        Box::new(right),
                        Op::Add
                    );
                    left = Expr::Binary(binary);
                },
                Token::Minus => {
                    self.next();

                    let right = self.expr_mul()?;

                    let binary = Binary::new(
                        Box::new(left),
                        Box::new(right),
                        Op::Sub
                    );
                    left = Expr::Binary(binary);
                },
                _ => {
                    break;
                }
            }
        }

        Ok(left)
    }

    // 乗算・除算処理
    fn expr_mul(&mut self) -> anyhow::Result<Expr> {
        let mut left = self.expr_unary()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Asterisk => {
                    self.next();

                    let right = self.expr_unary()?;

                    let binary = Binary::new(
                        Box::new(left),
                        Box::new(right),
                        Op::Mul
                    );
                    left = Expr::Binary(binary);
                },
                Token::Slash => {
                    self.next();

                    let right = self.expr_unary()?;

                    let binary = Binary::new(
                        Box::new(left),
                        Box::new(right),
                        Op::Div
                    );
                    left = Expr::Binary(binary);
                },
                _ => {
                    break;
                }
            }
        }

        Ok(left)
    }

    // 単項式(-)の処理
    fn expr_unary(&mut self) -> anyhow::Result<Expr> {
        if let Some(token) = self.peek() {
            match token {
                Token::Minus => {
                    self.next();

                    let expr = self.expr_unary()?;
                    let unary = Unary::new(Box::new(expr), UnaryOp::Neg);
                    Ok(Expr::Unary(unary))
                },
                _ => {
                    self.expr_primary()
                },
            }
        } else {
            Err(Error::msg("トークンが見つかりませんでした。expr_unary"))
        }
    }

    // リテラル処理
    fn expr_primary(&mut self) -> anyhow::Result<Expr> {
        if let Some(token) = self.next() {
            match token {
                Token::Text(text) => {
                    Ok(self.expr_name(text)?)
                },
                Token::Number(num) => {
                    let num = Value::Number(num);
                    Ok(Expr::Value(num))
                },
                Token::LParen => {
                    let expr = self.expr_add()?;
                    if !self.consume(Token::RParen) {
                        return Err(Error::msg("対応する)が見つかりませんでした。"));
                    }
                    Ok(expr)
                },
                _ => {
                    Err(Error::msg(format!("予期せぬ値です。expr_primary = {:?}", token)))
                },
            }
        } else {
            Err(Error::msg("予期せぬ値です。expr_primary"))
        }
    }

    fn expr_name(&mut self, name: String) -> anyhow::Result<Expr> {
        if self.peek() == Some(Token::LParen) {
            self.expr_func(name)
        } else {
            Err(Error::msg(format!("現在は変数参照に対応していません。name = {}", name)))
        }
    }

    fn expr_func(&mut self, name: String) -> anyhow::Result<Expr> {
        if !self.is_func(&name) {
            return Err(Error::msg(format!("存在しない関数です。name = {}", name)));
        }
        if !self.consume(Token::LParen) {
            return Err(Error::msg("現在は関数名の次は(である必要があります。"));
        }

        let arg = self.expr_add()?;

        if !self.consume(Token::RParen) {
            return Err(Error::msg("関数名が)で閉じられていません。"));
        }

        Ok(self.get_func(&name, arg))
    }


    fn is_func(&self, name: &str) -> bool {
        match name {
            "print" | "println" | "abs" => {
                true
            },
            _ => false
        }
    }

    fn get_func(&self, name: &str, args: Expr) -> Expr {
        match name {
            "print" => {
                let func = BuiltinFunc::Print(Box::new(args));
                Expr::Func(func)
            },
            "println" => {
                let func = BuiltinFunc::Println(Box::new(args));
                Expr::Func(func)
            },
            "abs" => {
                let func = BuiltinFunc::Abs(Box::new(args));
                Expr::Func(func)
            }
            _ => {
                panic!("{}", format!("存在しない関数名です。{}", name))
            },
        }
    }

    // 式文処理
    fn next(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.pos).cloned()
    }

    fn consume(&mut self, expected: Token) -> bool {
        if self.peek() == Some(expected) {
            self.next();
            true
        } else {
            false
        }
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
        }
    }
}
