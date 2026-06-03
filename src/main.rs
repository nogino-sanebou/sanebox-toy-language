struct Expr {
    token: Box<Token>,
}

impl Expr {
    fn new(token: Token) -> Self {
        let box1 = Box::new(token);

        Self {
            token: box1,
        }
    }

    fn eval(&self) -> anyhow::Result<Token> {
        match &*self.token {
            Token::Expr(expr) => {
                expr.eval()
            },
            Token::StdFunc(func) => {
                match func {
                    StdFunc::Print(expr) => {
                        let r = expr.eval();
                        let r = print(r.expect("printのexprに失敗"));
                        Ok(Token::Value(r))
                    }
                }
            },
            Token::Value(val) => {
                match val {
                    Value::Number(num) => {
                        Ok(Token::Value(Value::Number(*num)))
                    },
                    Value::Boolean(_) => {
                        panic!("想定外のvalue-Expression")
                    }
                }

            },
        }
    }
}

enum Token {
    Expr(Expr),
    StdFunc(StdFunc),
    Value(Value),
}

enum StdFunc {
    Print(Expr),
}

fn print(token: Token) -> Value {
    match token {
        Token::Value(val) => {
            match val {
                Value::Number(num) => {
                    print!("{}", num);
                    Value::Boolean(true)
                },
                Value::Boolean(_) => {
                    panic!("想定外のvalue-print")
                }
            }
        },
        _ => panic!("想定外のtoken-print")
    }
}

enum Value {
    Number(u64),
    Boolean(bool),
}

fn main() {

}

fn lexer<'a>(code: Vec<char>) -> Expr {
    let mut token = String::new();
    let mut code = code;

    while let Some((&c, rest)) = code.split_first() {
        if c == '(' {
            if token == "print" {
                let expr = lexer(rest.to_vec());
                let func = StdFunc::Print(expr);
                let token = Token::StdFunc(func);

                return Expr::new(token);
            } else {
                panic!("(の前がprintではない")
            }
        } else if c == ')' {
            if let Ok(num) = token.parse::<u64>() {
                let value = Value::Number(num);
                let token = Token::Value(value);
                return Expr::new(token);
            } else {
                panic!(")の前が数字ではない")
            }
        }
        token.push(c);
        code = rest.to_vec();
    }
    panic!("構文が想定外です");
}

fn eval(expr: Expr) -> anyhow::Result<Token> {
    expr.eval()
}

#[cfg(test)]
mod tests {
    use crate::{eval, lexer, Token, Value};

    #[test]
    fn test1() {
        let expr = lexer("print(123);".chars().collect());
        if let Ok(result) = eval(expr) {
            match result {
                Token::Value(val) => {
                    match val {
                        Value::Boolean(bool) => {
                            assert!(bool)
                        },
                        _ => unreachable!()
                    }
                },
                _ => unreachable!(),
            }
        }
    }
}