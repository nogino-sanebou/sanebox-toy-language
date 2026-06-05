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
                        panic!("想定外のvalue-Expression")
                    },
                }
            },
            Expr::Binary(bin) => {
                Ok(bin.calc())
            },
            Expr::Func(func) => {
                match func {
                    BuiltinFunc::Print(expr) => {
                        let r = expr.eval();
                        let r = print(r.expect("printのexprに失敗"));
                        Ok(r)
                    },
                    BuiltinFunc::Println(expr) => {
                        let r = expr.eval();
                        let r = println(r.expect("printlnのexprに失敗"));
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

    fn calc(&self) -> Value {
        let lhs = self.lhs.eval().expect("calcでlhsの展開に失敗");
        let rhs = self.rhs.eval().expect("calcでrhsの展開に失敗");

        match &self.op {
            Op::Add => {
                Binary::add(lhs, rhs)
            },
        }
    }

    fn add(lhs: Value, rhs: Value) -> Value {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            panic!("想定外のvalue -lhs")
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            panic!("想定外のvalue -rhs")
        };

        Value::Number(lhs + rhs)
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

#[derive(Clone)]
enum Token {
    Text(String),
    Number(u64),
    LParen,
    RParen,
    Plus,
    Semicolon,
}

fn print(value: Value) -> Value {
    match value {
        Value::Number(num) => {
            print!("{}", num);
            Value::Unit
        },
        _ => {
            panic!("想定外のvalue-print")
        },
    }
}

fn println(value: Value) -> Value {
    print(value);
    println!();

    Value::Unit
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

fn convert_literal(token: &String) -> Token {
    if let Ok(num) = token.parse::<u64>() {
        Token::Number(num)
    } else {
        Token::Text(token.clone())
    }
}

fn parse(tokens: Vec<Token>) -> Expr {
    let mut left= None;
    let mut tokens = tokens.to_vec();

    while let (token, mut tokens2) = tokens.split_first().expect("split_firstに失敗") {
        match token {
            Token::Text(text) => {
                if !is_func(text) {
                    panic!("現在は関数名以外認めていません")
                }

                let expr = parse(tokens2.to_vec());
                let func = Expr::Func(parse_func(text, expr));
                return func;
            }
            Token::Number(num) => {
                left = Some(Expr::Value(Value::Number(*num)));
            }
            Token::Plus => {
                let left2 = if let Some(expr) = left {
                    expr
                } else {
                    panic!("+の前が存在しなかった")
                };

                let right = if let Some(num) = tokens2.get(0) {
                    let num = if let Token::Number(num) = num {
                        Value::Number(*num)
                    } else {
                        panic!("+の次が数字でなかった")
                    };
                    Expr::Value(num)
                } else {
                    panic!("+の次が数字でなかった2")
                };

                let bin = Binary::new(Box::new(left2), Box::new(right), Op::Add);
                let expr = Expr::Binary(bin);

                left = Some(expr);

                tokens2 = &tokens2[1..];
            }
            _ => {
            }
        }

        if tokens2.is_empty() {
            break;
        }

        tokens = tokens2.to_vec();
    }

    if let Some(expr) = left {
        expr
    } else {
        panic!("構文が想定外です。")
    }
}

fn is_func(name: &str) -> bool {
    match name {
        "print" | "println" => {
            true
        },
        _ => false
    }
}

fn parse_func(name: &str, args: Expr) -> BuiltinFunc {
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

fn eval(expr: Expr) -> anyhow::Result<Value> {
    expr.eval()
}

#[cfg(test)]
mod tests {
    use crate::{eval, lexer, parse, Value};

    #[test]
    fn test1() {
        let tokens = lexer("println(12345);");
        let expr = parse(tokens);

        if let Ok(result) = eval(expr) {
            match result {
                Value::Unit => {
                },
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test2() {
        let tokens = lexer("println(3 + 2);");
        let expr = parse(tokens);

        if let Ok(result) = eval(expr) {
            match result {
                Value::Unit => {
                },
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test3() {
        let tokens = lexer("print(1 + 2 + 5);");
        let expr = parse(tokens);

        if let Ok(result) = eval(expr) {
            match result {
                Value::Unit => {
                },
                _ => unreachable!(),
            }
        }
    }
}