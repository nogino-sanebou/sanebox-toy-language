use anyhow::Error;

#[derive(Clone)]
enum Expr {
    // Expr(Box<Expr>),
    Value(Value),
    Func(BuiltinFunc),
    Binary(Binary),
    Unary(Unary),
}

impl Expr {
    fn eval(&self) -> anyhow::Result<Value> {
        match &self {
            // Expr::Expr(expr) => {
            //     expr.eval()
            // },
            Expr::Value(value) => {
                match value {
                    Value::Number(num) => {
                        Ok(Value::Number(*num))
                    },
                    _ => {
                        Err(Error::msg("Expr::ValueはValue::Number以外を想定していません。"))
                    },
                }
            },
            Expr::Binary(bin) => {
                Ok(bin.calc()?)
            },
            Expr::Func(func) => {
                match func {
                    BuiltinFunc::Print(expr) => {
                        let r = expr.eval()?;
                        let r = print(r)?;
                        Ok(r)
                    },
                    BuiltinFunc::Println(expr) => {
                        let r = expr.eval()?;
                        let r = println(r)?;
                        Ok(r)
                    },
                }
            },
            Expr::Unary(unary) => {
                Ok(unary.calc()?)
            },
        }
    }
}

#[derive(Clone)]
enum BuiltinFunc {
    Print(Box<Expr>),
    Println(Box<Expr>),
}

#[derive(Clone)]
struct Binary {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    op: Op,
}

impl Binary {
    fn new(lhs: Box<Expr>, rhs: Box<Expr>, op: Op) -> Self {
        Self {
            lhs,
            rhs,
            op,
        }
    }

    fn calc(&self) -> anyhow::Result<Value> {
        let lhs = self.lhs.eval()?;
        let rhs = self.rhs.eval()?;

        match &self.op {
            Op::Add => {
                Binary::add(lhs, rhs)
            },
            Op::Sub => {
                Binary::sub(lhs, rhs)
            },
            Op::Mul => {
                Binary::mul(lhs, rhs)
            },
            Op::Div => {
                Binary::div(lhs, rhs)
            },
        }
    }

    fn add(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            return Err(Error::msg("想定外のvalue add-lhs"));
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            return Err(Error::msg("想定外のvalue add-rhs"));
        };

        Ok(Value::Number(lhs + rhs))
    }

    fn sub(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            return Err(Error::msg("想定外のvalue sub-lhs"));
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            return Err(Error::msg("想定外のvalue sub-rhs"));
        };

        Ok(Value::Number(lhs - rhs))
    }

    fn mul(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            return Err(Error::msg("想定外のvalue mul-lhs"));
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            return Err(Error::msg("想定外のvalue mul-rhs"));
        };

        Ok(Value::Number(lhs * rhs))
    }

    fn div(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            return Err(Error::msg("想定外のvalue div-lhs"));
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            return Err(Error::msg("想定外のvalue div-rhs"));
        };

        if rhs == 0 {
            return Err(Error::msg("0で除算できません。"));
        }

        Ok(Value::Number(lhs / rhs))
    }
}

#[derive(Copy, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Copy, Clone)]
enum UnaryOp {
    Neg,
}

#[derive(Clone)]
struct Unary {
    expr: Box<Expr>,
    op: UnaryOp,
}

impl Unary {
    fn new(expr: Box<Expr>, op: UnaryOp) -> Self {
        Self {
            expr,
            op,
        }
    }

    fn calc(&self) -> anyhow::Result<Value> {
        let value = self.expr.eval()?;

        match self.op {
            UnaryOp::Neg => {
                match value {
                    Value::Number(num) => {
                        Ok(Value::Number(-num))
                    },
                    _ => {
                        Err(Error::msg("数値以外が出現しました。 Unary.calc"))
                    },
                }
            }
        }
    }
}

#[derive(Clone)]
enum Value {
    Unit,
    Number(i64),
    // Boolean(bool),
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Token {
    Text(String),
    Number(i64),
    LParen,
    RParen,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Semicolon,
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn parse_expr(&mut self) -> anyhow::Result<Expr> {
        if let Some(token) = self.peek() {
            match token {
                // テキストは関数として処理をする
                Token::Text(text) => {
                    if !self.is_func(&text) {
                        return Err(Error::msg("現在は関数名以外認めていません"));
                    }
                    self.next();
                    if !self.consume(Token::LParen) {
                        return Err(Error::msg("現在は関数名の次は(である必要があります。"));
                    }

                    let arg = match self.expr_add() {
                        Ok(o) => {
                            o
                        },
                        Err(e) => {
                            return Err(e);
                        },
                    };

                    if !self.consume(Token::RParen) {
                        return Err(Error::msg("関数名が)で閉じられていません。"));
                    }

                    if !self.consume(Token::Semicolon) {
                        return Err(Error::msg("現在は関数のみが先頭の式として認められているので;が必要です。"));
                    }

                    Ok(Expr::Func(self.get_func(&text, arg)))
                },
                // 数値、開始括弧、単項マイナスが来たら式文として処理をする
                Token::Number(_) | Token::LParen | Token::Minus => {
                    Ok(self.expr_stmt()?)
                },
                _ => {
                    Err(Error::msg("構文が想定外です。:parse_expr"))
                },
            }
        } else {
            Err(Error::msg("構文が想定外です。:parse_expr"))
        }
    }

    fn is_func(&self, name: &str) -> bool {
        match name {
            "print" | "println" => {
                true
            },
            _ => false
        }
    }

    fn get_func(&self, name: &str, args: Expr) -> BuiltinFunc {
        match name {
            "print" => {
                BuiltinFunc::Print(Box::new(args))
            },
            "println" => {
                BuiltinFunc::Println(Box::new(args))
            },
            _ => {
                panic!("{}", format!("存在しない関数名です。{}", name))
            },
        }
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

    // 式文処理
    fn expr_stmt(&mut self) -> anyhow::Result<Expr> {
        let res = self.expr_add()?;

        if !self.consume(Token::Semicolon) {
            return Err(Error::msg("式文の末尾がセミコロンでありません。"));
        }

        Ok(res)
    }

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

    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
        }
    }
}

fn print(value: Value) -> anyhow::Result<Value> {
    match value {
        Value::Number(num) => {
            print!("{}", num);
            Ok(Value::Unit)
        },
        _ => {
            Err(Error::msg("想定外のvalue-print"))
        },
    }
}

fn println(value: Value) -> anyhow::Result<Value> {
    print(value)?;
    println!();

    Ok(Value::Unit)
}

fn main() {
    let tokens = lexer("println(123);");
    let mut parser = Parser::new(tokens);
    if let Ok(expr) = parser.parse_expr() {
        let _ = eval(expr);
    }
}

fn lexer(code: &str) -> Vec<Token> {
    let mut token = String::new();
    let mut tokens = Vec::new();
    let mut chars = code.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                push_literal(&mut tokens, &mut token);
                tokens.push(Token::LParen);
            },
            ')' => {
                push_literal(&mut tokens, &mut token);
                tokens.push(Token::RParen);
            },
            '+' => {
                push_literal(&mut tokens, &mut token);
                tokens.push(Token::Plus);
            },
            '-' => {
                push_literal(&mut tokens, &mut token);
                tokens.push(Token::Minus);
            },
            '*' => {
                push_literal(&mut tokens, &mut token);
                tokens.push(Token::Asterisk);
            },
            '/' => {
                push_literal(&mut tokens, &mut token);
                tokens.push(Token::Slash);
            },
            ';' => {
                push_literal(&mut tokens, &mut token);
                tokens.push(Token::Semicolon);
            },
            c if c.is_whitespace() => {
                push_literal(&mut tokens, &mut token);
            },
            _ => {
                token.push(c);
            },
        }
    }

    push_literal(&mut tokens, &mut token);

    tokens
}

fn convert_literal(token: &str) -> Token {
    if let Ok(num) = token.parse::<i64>() {
        Token::Number(num)
    } else {
        if token.is_empty() {
            panic!("想定外の空文字が出現しました。")
        }
        Token::Text(token.to_string())
    }
}

fn push_literal(tokens: &mut Vec<Token>, token: &mut String) {
    if !token.is_empty() {
        tokens.push(convert_literal(&token));
        token.clear();
    }
}

fn eval(expr: Expr) -> anyhow::Result<Value> {
    expr.eval()
}

#[cfg(test)]
mod tests {
    use crate::{eval, lexer, Parser, Value};

    #[test]
    fn test01() {
        print!("test1 [println(12345);] = ");

        let tokens = lexer("println(12345);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test02() {
        print!("test2 [println(3 + 2);] = ");

        let tokens = lexer("println(3 + 2);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test03() {
        print!("test3 [println(1 + 2 + 5);] = ");

        let tokens = lexer("println(1 + 2 + 5);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test04() {
        print!("test4 [println(3 + 12 + 7 + 10);] = ");

        let tokens = lexer("println(3 + 12 + 7 + 10);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test05() {
        print!("test5 [println(10 - 7);] = ");

        let tokens = lexer("println(10 - 7);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test06() {
        print!("test6 [println(10 - 7 + 2);] = ");

        let tokens = lexer("println(10 - 7 + 2);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test07() {
        print!("test7 [println(10 - 7 + 2 - 4);] = ");

        let tokens = lexer("println(10 - 7 + 2 - 4);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test08() {
        print!("test8 [println(5 - 7 - 4);] = ");

        let tokens = lexer("println(5 - 7 - 4);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test09() {
        print!("test9 [println((1 + 2) + 3);] = ");

        let tokens = lexer("println((1 + 2) + 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test10() {
        print!("test10 [println(10 - (3 + 2));] = ");

        let tokens = lexer("println(10 - (3 + 2));");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test11() {
        print!("test11 [println((10 - 3) - 2);] = ");

        let tokens = lexer("println((10 - 3) - 2);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test12() {
        print!("test12 [println(5 + (10 - 3) - 2);] = ");

        let tokens = lexer("println(5 + (10 - 3) - 2);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test13() {
        print!("test13 [println(((1 + 2) - (3 + 4)) + 5);] = ");

        let tokens = lexer("println(((1 + 2) - (3 + 4)) + 5);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test14() {
        print!("test14 [println((1 + 2 - 3 + 4) + 5);] = ");

        let tokens = lexer("println((1 + 2 - 3 + 4) + 5);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test15() {
        print!("test15 [println(1 + (2 - 3 + 4 + 5));] = ");

        let tokens = lexer("println(1 + (2 - 3 + 4 + 5));");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test16_err() {
        print!("test16 [println((1 + 2 + 3);] = ");

        let tokens = lexer("println((1 + 2 + 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr();

        let msg ;

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                msg = e.to_string();
                assert_eq!("関数名が)で閉じられていません。", msg);
            },
        }

        println!("{}", msg);
    }

    #[test]
    fn test17_err() {
        print!("test17 [println(1 + (2 + 3);] = ");

        let tokens = lexer("println(1 + (2 + 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr();

        let msg;

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                msg = e.to_string();
                assert_eq!("関数名が)で閉じられていません。", msg);
            },
        }

        println!("{}", msg);
    }

    #[test]
    fn test18_err() {
        print!("test18 [println(1 + 2));] = ");

        let tokens = lexer("println(1 + 2));");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr();

        let msg;

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                msg = e.to_string();
                assert_eq!("現在は関数のみが先頭の式として認められているので;が必要です。", msg);
            },
        }

        println!("{}", msg);
    }

    #[test]
    fn test19_err() {
        print!("test19 [println((1 + 2)));] = ");

        let tokens = lexer("println((1 + 2)));");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr();

        let msg;

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                msg = e.to_string();
                assert_eq!("現在は関数のみが先頭の式として認められているので;が必要です。", msg);
            },
        }

        println!("{}", msg);
    }

    #[test]
    fn test20_err() {
        print!("test20 [println(1 + 2 + 3)] = ");

        let tokens = lexer("println(1 + 2 + 3)");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr();

        let msg;

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                msg = e.to_string();
                assert_eq!("現在は関数のみが先頭の式として認められているので;が必要です。", msg);
            },
        }

        println!("{}", msg);
    }

    #[test]
    fn test21() {
        print!("test21 [1 + 3;] = ");

        let tokens = lexer("1 + 3;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(4, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test22() {
        print!("test22 [3 - 1;] = ");

        let tokens = lexer("3 - 1;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(2, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test23() {
        print!("test23 [3 + 4 - 5;] = ");

        let tokens = lexer("3 + 4 - 5;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(2, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test24() {
        print!("test24 [(3 + 2) - (7 + 3);] = ");

        let tokens = lexer("(3 + 2) - (7 + 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(-5, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test25() {
        print!("test25 [((3 + 2) - (7 + 3)) + 20;] = ");

        let tokens = lexer("((3 + 2) - (7 + 3)) + 20;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(15, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test26() {
        print!("test26 [200 + ((100 - 20) + (15 + 20));] = ");

        let tokens = lexer("200 + ((100 - 20) + (15 + 20));");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(315, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test27() {
        print!("test27 [((10 + 20) - (30 + 70) - (200 - 150));] = ");

        let tokens = lexer("((10 + 20) - (30 + 70) - (200 - 150));");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(-120, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test28_err() {
        print!("test28 [1 + 2 - 3] = ");

        let tokens = lexer("1 + 2 - 3");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr();

        let msg;

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                msg = e.to_string();
                assert_eq!("式文の末尾がセミコロンでありません。", msg);
            },
        }

        println!("{}", msg);
    }

    #[test]
    fn test29() {
        print!("test29 [3 * 3;] = ");

        let tokens = lexer("3 * 3;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(9, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test30() {
        print!("test30 [120 / 4;] = ");

        let tokens = lexer("120 / 4;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(30, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test31() {
        print!("test31 [10 * 20 / 2;] = ");

        let tokens = lexer("10 * 20 / 2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(100, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test32() {
        print!("test32 [3 + 5 * 5;] = ");

        let tokens = lexer("3 + 5 * 5;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(28, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test33() {
        print!("test33 [3 + 5 * 5 + 10 / 5;] = ");

        let tokens = lexer("3 + 5 * 5 + 10 / 5;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(30, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test34() {
        print!("test34 [(10 + 1) * (3 + 8);] = ");

        let tokens = lexer("(10 + 1) * (3 + 8);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(121, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test35() {
        print!("test35 [print(12 * 20);] = ");

        let tokens = lexer("print(12 * 10);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
                println!();
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test36() {
        print!("test36 [println((5 - 10) / (1 + 1));] = ");

        let tokens = lexer("println((5 - 10) / (1 + 1));");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test37_err() {
        print!("test37 [10 / 0;] = ");

        let tokens = lexer("10 / 0;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr();

        let eval = eval(expr.unwrap());

        let msg;

        match eval {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                msg = e.to_string();
                assert_eq!("0で除算できません。", msg);
            },
        }

        println!("{}", msg);
    }

    #[test]
    fn test38() {
        print!("test38 [20 / 5 / 2;] = ");

        let tokens = lexer("20 / 5 / 2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(2, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test39() {
        print!("test39 [2 * 3 + 4 * 5;] = ");

        let tokens = lexer("2 * 3 + 4 * 5;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(26, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test40() {
        print!("test40 [20 / (5 * 2);] = ");

        let tokens = lexer("20 / (5 * 2);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(2, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test41() {
        print!("test41 [(20 / 5) * 2;] = ");

        let tokens = lexer("(20 / 5) * 2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(8, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test42_err() {
        print!("test42 [10 / ;] = ");

        let tokens = lexer("10 / ;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr();

        let msg;

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                msg = e.to_string();
                assert_eq!("予期せぬ値です。expr_primary = Semicolon", msg);
            },
        }

        println!("{}", msg);
    }

    #[test]
    fn test43() {
        print!("test43 [-10 + 15;] = ");

        let tokens = lexer("-10 + 15;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(5, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test44() {
        print!("test44 [6 + -2;] = ");

        let tokens = lexer("6 + -2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(4, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test45() {
        print!("test45 [6 - -2;] = ");

        let tokens = lexer("6 - -2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(8, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test46() {
        print!("test46 [-102;] = ");

        let tokens = lexer("-102;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(-102, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test47() {
        print!("test47 [-(1 + 3);] = ");

        let tokens = lexer("-(1 + 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(-4, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test48() {
        print!("test48 [10 * -3;] = ");

        let tokens = lexer("10 * -3;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(-30, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test49() {
        print!("test49 [-10 * 4;] = ");

        let tokens = lexer("-10 * 4;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(-40, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test50() {
        print!("test50 [10 / -2;] = ");

        let tokens = lexer("10 / -2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(-5, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test51() {
        print!("test51 [-10 / 2;] = ");

        let tokens = lexer("-10 / 2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(-5, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test52() {
        print!("test52 [-(2 * 3);] = ");

        let tokens = lexer("-(2 * 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(-6, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test53() {
        print!("test53 [(-1 + 6) * (2 -- 3) + -(10 / 2);] = ");

        let tokens = lexer("(-1 + 6) * (2 -- 3) + -(10 / 2);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(20, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test54() {
        print!("test54 [println(-20 + 5);] = ");

        let tokens = lexer("println(-20 + 5);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn test55() {
        print!("test55 [10 - - -1;] = ");

        let tokens = lexer("10 - - -1;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Number(num) => {
                assert_eq!(9, num);
                println!("{}", num);
            },
            _ => {
                unreachable!()
            },
        }
    }
}