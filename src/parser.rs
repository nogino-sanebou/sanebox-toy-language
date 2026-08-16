use anyhow::Error;
use crate::lexer::Token;
use crate::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    // パース処理の入口
    pub fn parse(&mut self) -> anyhow::Result<Statements> {
        let mut stmts = Statements::new();

        while self.peek().is_some() {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        }

        Ok(stmts)
    }

    // ステートメント処理の入口
    fn parse_stmt(&mut self) -> anyhow::Result<Stmt> {
        let token = self.peek().unwrap();
        let stmt = match token {
            Token::Text(text) => {
                match text.as_str() {
                    // 変数の処理
                    "let" => {
                        self.parse_let_stmt()?
                    },
                    // 変数でなければ関数として処理
                    _ => {
                        self.parse_expr_stmt()?
                    }
                }
            }
            // 予約語でなければ計算式として処理する
            _ => {
                self.parse_expr_stmt()?
            }
        };

        Ok(stmt)
    }

    // 変数宣言のステートメント処理
    fn parse_let_stmt(&mut self) -> anyhow::Result<Stmt> {
        self.next();
        let name = match self.next() {
            // 予約語でなければこれを変数名とする
            Some(Token::Text(name)) => {
                if self.is_keyword(&name) {
                    let msg = format!("変数名に予約語は使用できません。name = {}", name);
                    return Err(Error::msg(msg));
                }

                name
            },
            // 文字列出なかった場合はエラー
            Some(token) => {
                let msg = format!("変数名が文字列ではありません。token = {:?}", token);
                return Err(Error::msg(msg));
            },
            // 次のトークンがそもそも文字列でない場合もエラー
            None => {
                return Err(Error::msg("letの後に変数名がありません。"));
            }
        };

        if !self.consume(Token::Equal) {
            let msg = format!("変数名の次は「=」でないといけません。token = {:?}", self.peek());
            return Err(Error::msg(msg));
        }

        // 右辺の解析処理
        let expr = self.expr_add()?;

        if !self.consume(Token::Semicolon) {
            return Err(Error::msg("式の末尾がセミコロンでありません。"));
        }

        Ok(Stmt::Let { name, expr })
    }

    // 計算式のステートメント処理
    fn parse_expr_stmt(&mut self) -> anyhow::Result<Stmt> {
        let expr = self.parse_expr()?;

        if !self.consume(Token::Semicolon) {
            return Err(Error::msg("式の末尾がセミコロンでありません。"));
        }

        Ok(Stmt::Expr(expr))
    }

    // 予約語であるか確認
    fn is_keyword(&self, text: &String) -> bool {
        match text.as_str() {
            "let" => {
                true
            }
            _ => {
                false
            }
        }
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

    // 関数、変数の解析処理
    fn expr_name(&mut self, name: String) -> anyhow::Result<Expr> {
        if self.peek() == Some(Token::LParen) {
            self.next();
            self.expr_func(name)
        } else {
            Ok(Expr::Variable(name))
        }
    }

    // 関数の解析処理
    fn expr_func(&mut self, name: String) -> anyhow::Result<Expr> {
        if !self.is_func(&name) {
            return Err(Error::msg(format!("存在しない関数です。name = {}", name)));
        }

        let arg = self.expr_add()?;

        if !self.consume(Token::RParen) {
            let msg = format!("関数名が)で閉じられていません。token = {:?}", self.peek());
            return Err(Error::msg(msg));
        }

        Ok(self.get_func(&name, arg))
    }


    // nameが関数であるか比較
    fn is_func(&self, name: &str) -> bool {
        match name {
            "print" | "println" | "abs" => {
                true
            },
            _ => false
        }
    }

    // 関数のEnumを獲得
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
