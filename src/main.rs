enum Expr {
    Value(Value),
    Func(StdFunc),
    Expr(Box<Expr>),
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
                    Value::Boolean(_) => {
                        panic!("想定外のvalue-Expression")
                    },
                    Value::Formula(formula) => {
                        Ok(formula.calc())
                    },
                }
            },
            Expr::Func(func) => {
                match func {
                    StdFunc::Print(expr) => {
                        let r = expr.eval();
                        let r = print(r.expect("printのexprに失敗"));
                        Ok(r)
                    },
                    StdFunc::Println(expr) => {
                        let r = expr.eval();
                        let r = println(r.expect("printlnのexprに失敗"));
                        Ok(r)
                    },
                }
            },
        }
    }
}

enum StdFunc {
    Print(Box<Expr>),
    Println(Box<Expr>),
}

#[derive(Clone)]
struct Formula {
    lhs: Box<Value>,
    rhs: Box<Value>,
    op: Op,
}

impl Formula {
    fn new(lhs: Box<Value>, rhs: Box<Value>, op: Op) -> Self {
        Self {
            lhs,
            rhs,
            op,
        }
    }

    fn calc(&self) -> Value {
        let lhs = match &*self.lhs {
            Value::Formula(f) => {
                f.calc()
            },
            Value::Number(_) => {
                *self.lhs.clone()
            },
            Value::Boolean(_) => {
                panic!("無効なValueです。-calc lhs")
            },
        };

        let rhs = match &*self.rhs {
            Value::Formula(f) => {
                f.calc()
            },
            Value::Number(_) => {
                *self.rhs.clone()
            },
            Value::Boolean(_) => {
                panic!("無効なValueです。-calc rhs")
            },
        };

        match &self.op {
            Op::Add => {
                Formula::add(lhs, rhs)
            },
        }
    }

    fn add(lhs: Value, rhs: Value) -> Value {
        let lhs = match lhs {
            Value::Number(num) => {
                num
            },
            _ => panic!("想定外のvalue -lhs")
        };

        let rhs = match rhs {
            Value::Number(num) => {
                num
            },
            _ => panic!("想定外のvalue -rhs")
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
    Number(u64),
    Boolean(bool),
    Formula(Formula),
}

fn print(value: Value) -> Value {
    match value {
        Value::Number(num) => {
            print!("{}", num);
            Value::Boolean(true)
        },
        _ => {
            panic!("想定外のvalue-print")
        },
    }
}

fn println(value: Value) -> Value {
    let p = print(value);
    match p {
        Value::Boolean(bool) => {
            if !bool {
                panic!("printに失敗しています。")
            }
        },
        _ => {
            panic!("想定外のvalue-println")
        }
    }

    println!();

    Value::Boolean(true)
}

fn main() {

}

fn lexer(code: &str) -> Vec<String> {
    let mut token = String::new();
    let mut tokens = Vec::new();

    for c in code.chars() {
        match c {
            '(' | ')' => {
                tokens.push(token.clone());
                token.clear();
            },
            '+' => {
                tokens.push(token.clone());
                tokens.push(String::from("+"));
                token.clear();
            }
            ' ' | ';' => {
                continue;
            }
            _ => {
                token.push(c);
            }
        }
    }

    tokens
}

fn parse(tokens: Vec<String>) -> Expr {
    let (token, tokens) = tokens.split_first().expect("split_firstに失敗");

    match token.as_str() {
        "print" => {
            let expr = parse(tokens.to_vec());
            let func = StdFunc::Print(Box::new(expr));

            return Expr::Func(func);
        },
        "println" => {
            let expr = parse(tokens.to_vec());
            let func = StdFunc::Println(Box::new(expr));

            return Expr::Func(func);
        },
        _ => {
            if let Ok(num) = token.parse::<u64>() {
                let Some(next) = tokens.get(0) else {
                    return Expr::Value(Value::Number(num));
                };

                if next.as_str() == "+" {
                    let lhs = Box::new(Value::Number(num));

                    let next = tokens.get(1).expect("想定外:parse:tokens.get(0)");
                    let Ok(num) = next.parse::<u64>() else {
                        let x = format!("想定外の右辺です。{}", next);
                        panic!("{}", x)
                    };
                    let rhs = Box::new(Value::Number(num));

                    let f = Formula::new(lhs, rhs, Op::Add);

                    return Expr::Value(Value::Formula(f));
                } else {
                    panic!("構文が想定外です-parse:tokens.get(0)")
                }
            }
        }
    }

    panic!("構文が想定外です");
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
                Value::Boolean(bool) => {
                    assert!(bool)
                },
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test2() {
        let tokens = lexer("print(1 + 2);");
        let expr = parse(tokens);

        if let Ok(result) = eval(expr) {
            match result {
                Value::Boolean(bool) => {
                    assert!(bool)
                },
                _ => unreachable!(),
            }
        }
    }
}