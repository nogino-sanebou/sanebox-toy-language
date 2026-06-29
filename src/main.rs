use anyhow::Error;

#[derive(Clone)]
enum Expr {
    // Expr(Box<Expr>),
    Value(Value),
    Func(BuiltinFunc),
    Binary(Binary),
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
}

#[derive(Copy, Clone)]
enum Op {
    Add,
    Sub,
}

#[derive(Clone)]
enum Value {
    Unit,
    Number(i64),
    // Boolean(bool),
}

#[derive(Clone, Eq, PartialEq)]
enum Token {
    Text(String),
    Number(i64),
    LParen,
    RParen,
    Plus,
    Minus,
    Semicolon,
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn parse_expr(&mut self) -> anyhow::Result<Expr> {
        if let Some(token) = self.next() {
            match token {
                Token::Text(text) => {
                    if !self.is_func(&text) {
                        return Err(Error::msg("現在は関数名以外認めていません"));
                    }
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
                Token::Number(num) => {
                    Ok(Expr::Value(Value::Number(num)))
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

    fn expr_add(&mut self) -> anyhow::Result<Expr> {
        let mut left = self.expr_primary()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Plus => {
                    self.next();

                    let right = self.expr_primary()?;

                    let binary = Binary::new(
                        Box::new(left),
                        Box::new(right),
                        Op::Add
                    );
                    left = Expr::Binary(binary);
                },
                Token::Minus => {
                    self.next();

                    let right = self.expr_primary()?;

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
                    Err(Error::msg("予期せぬ値です。expr_primary"))
                },
            }
        } else {
            Err(Error::msg("予期せぬ値です。expr_primary"))
        }
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

        let mut msg = String::new();

        match expr {
            Ok(_) => {
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

        let mut msg = String::new();

        match expr {
            Ok(_) => {
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

        let mut msg = String::new();

        match expr {
            Ok(_) => {
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

        let mut msg = String::new();

        match expr {
            Ok(_) => {
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

        let mut msg = String::new();

        match expr {
            Ok(_) => {
            },
            Err(e) => {
                msg = e.to_string();
                assert_eq!("現在は関数のみが先頭の式として認められているので;が必要です。", msg);
            },
        }

        println!("{}", msg);
    }

    #[test]
    fn test21_err() {
        print!("test21 [println(5 - 7 - 4);] = ");

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
}