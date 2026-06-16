use anyhow::Error;

#[derive(Clone)]
enum Expr {
    Expr(Box<Expr>),
    Value(Value),
    Func(BuiltinFunc),
    Binary(Binary),
}

impl Expr {
    fn eval(&self) -> anyhow::Result<Value> {
        match &self {
            Expr::Expr(expr) => {
                expr.eval()
            },
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
}

#[derive(Copy, Clone)]
enum Op {
    Add,
}

#[derive(Clone)]
enum Value {
    Unit,
    Number(u64),
    Boolean(bool),
}

#[derive(Clone, Eq, PartialEq)]
enum Token {
    Text(String),
    Number(u64),
    LParen,
    RParen,
    Plus,
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

        while self.consume(Token::Plus) {
            let right = self.parse_expr()?;

            let binary = Binary::new(
                Box::new(left),
                Box::new(right),
                Op::Add
            );
            left = Expr::Binary(binary);
        }

        if !self.consume(Token::RParen) {
            return Err(Error::msg("閉じ括弧がありません。"));
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

}

fn lexer(code: &str) -> Vec<Token> {
    let mut token = String::new();
    let mut tokens = Vec::new();

    for c in code.chars() {
        match c {
            '(' => {
                tokens.push(convert_literal(&token));
                tokens.push(Token::LParen);
                token.clear();
            },
            ')' => {
                tokens.push(convert_literal(&token));
                tokens.push(Token::RParen);
                token.clear();
            },
            '+' => {
                tokens.push(convert_literal(&token));
                tokens.push(Token::Plus);
                token.clear();
            }
            ';' => {
                tokens.push(Token::Semicolon);
            }
            ' ' => {
                continue;
            }
            _ => {
                token.push(c);
            }
        }
    }

    tokens
}

fn convert_literal(token: &str) -> Token {
    if let Ok(num) = token.parse::<u64>() {
        Token::Number(num)
    } else {
        if token.is_empty() {
            panic!("想定外の空文字が出現しました。")
        }
        Token::Text(token.to_string())
    }
}

fn eval(expr: Expr) -> anyhow::Result<Value> {
    expr.eval()
}

#[cfg(test)]
mod tests {
    use crate::{eval, lexer, Parser, Value};

    #[test]
    fn test1() {
        let tokens = lexer("println(12345);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => unreachable!(),
        }
    }

    #[test]
    fn test2() {
        let tokens = lexer("println(3 + 2);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => unreachable!(),
        }
    }

    #[test]
    fn test3() {
        let tokens = lexer("println(1 + 2 + 5);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => unreachable!(),
        }
    }

    #[test]
    fn test4() {
        let tokens = lexer("println(3 + 12 + 7 + 10);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        let r = eval(expr).unwrap();

        match r {
            Value::Unit => {
            },
            _ => unreachable!(),
        }
    }
}