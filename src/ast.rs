use std::collections::HashMap;
use std::slice::Iter;
use anyhow::Error;

use crate::eval::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Statements {
    stmts: Vec<Stmt>,
}

impl Statements {
    pub fn push(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }

    pub fn iter(&self) -> Iter<Stmt> {
        self.stmts.iter()
    }

    pub fn len(&self) -> usize {
        self.stmts.len()
    }
    
    pub fn as_slice(&self) -> &[Stmt] {
        &self.stmts
    }

    pub fn new() -> Self {
        Self {
            stmts: Vec::new(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Stmt {
    Expr(Expr),
    Let {
        name: String,
        expr: Expr,
    },
    If {
        condition: Expr,
        then_branch: Statements,
        else_branch: Option<Statements>,
    },
    While {
        condition: Expr,
        body: Statements,
    },
    For {
        body: Statements,
        // TODO: for の構文を決めてから init/condition/update などを追加する
    }
}

pub struct Environment {
    list: HashMap<String, Value>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: Value) {
        self.list.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.list.get(name)
    }

    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Expr {
    Value(Value),
    Func(BuiltinFunc),
    Binary(Binary),
    Unary(Unary),
    Variable(String),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum BuiltinFunc {
    Print(Box<Expr>),
    Println(Box<Expr>),
    Abs(Box<Expr>),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum UnaryOp {
    Neg,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Binary {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    op: Op,
}

impl Binary {
    pub fn new(lhs: Box<Expr>, rhs: Box<Expr>, op: Op) -> Self {
        Self {
            lhs,
            rhs,
            op,
        }
    }

    pub fn calc(&self, env: &Environment) -> anyhow::Result<Value> {
        let lhs = eval(*self.lhs.clone(), env)?;
        let rhs = eval(*self.rhs.clone(), env)?;

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
            Op::Less => {
                Binary::less(lhs, rhs)
            },
            Op::LessEqual => {
                Binary::less_equal(lhs, rhs)
            },
            Op::Greater => {
                Binary::greater(lhs, rhs)
            },
            Op::GreaterEqual => {
                Binary::greater_equal(lhs, rhs)
            },
            Op::Equal => {
                Binary::equal(lhs, rhs)
            },
            Op::NotEqual => {
                Binary::not_equal(lhs, rhs)
            },
            Op::And => {
                Binary::and(lhs, rhs)
            },
            Op::Or => {
                Binary::or(lhs, rhs)
            },
        }
    }

    fn add(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            return Err(Error::msg("addの左辺に数値以外が出現しました。"));
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            return Err(Error::msg("addの右辺に数値以外が出現しました。"));
        };

        Ok(Value::Number(lhs + rhs))
    }

    fn sub(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            return Err(Error::msg("subの左辺に数値以外が出現しました。"));
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            return Err(Error::msg("subの右辺に数値以外が出現しました。"));
        };

        Ok(Value::Number(lhs - rhs))
    }

    fn mul(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            return Err(Error::msg("mulの左辺に数値以外が出現しました。"));
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            return Err(Error::msg("mulの右辺に数値以外が出現しました。"));
        };

        Ok(Value::Number(lhs * rhs))
    }

    fn div(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            return Err(Error::msg("divの左辺に数値以外が出現しました。"));
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            return Err(Error::msg("divの右辺に数値以外が出現しました。"));
        };

        if rhs == 0 {
            return Err(Error::msg("0で除算できません。"));
        }

        Ok(Value::Number(lhs / rhs))
    }

    fn less(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            return Err(Error::msg("lessの左辺に数値以外が出現しました。"));
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            return Err(Error::msg("lessの右辺に数値以外が出現しました。"));
        };

        Ok(Value::Boolean(lhs < rhs))
    }

    fn less_equal(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            return Err(Error::msg("less_equalの左辺に数値以外が出現しました。"));
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            return Err(Error::msg("less_equalの右辺に数値以外が出現しました。"));
        };

        Ok(Value::Boolean(lhs <= rhs))
    }

    fn greater(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            return Err(Error::msg("greaterの左辺に数値以外が出現しました。"));
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            return Err(Error::msg("greaterの右辺に数値以外が出現しました。"));
        };

        Ok(Value::Boolean(lhs > rhs))
    }

    fn greater_equal(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            return Err(Error::msg("greater_equalの左辺に数値以外が出現しました。"));
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            return Err(Error::msg("greater_equalの右辺に数値以外が出現しました。"));
        };

        Ok(Value::Boolean(lhs >= rhs))
    }

    fn equal(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            return Err(Error::msg("equalの左辺に数値以外が出現しました。"));
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            return Err(Error::msg("equalの右辺に数値以外が出現しました。"));
        };

        Ok(Value::Boolean(lhs == rhs))
    }

    fn not_equal(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Number(num) = lhs {
            num
        } else {
            return Err(Error::msg("not_equalの左辺に数値以外が出現しました。"));
        };

        let rhs = if let Value::Number(num) = rhs {
            num
        } else {
            return Err(Error::msg("not_equalの右辺に数値以外が出現しました。"));
        };

        Ok(Value::Boolean(lhs != rhs))
    }

    fn and(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Boolean(b) = lhs {
            b
        } else {
            return Err(Error::msg("andの左辺にboolean以外が出現しました。"));
        };

        let rhs = if let Value::Boolean(b) = rhs {
            b
        } else {
            return Err(Error::msg("andの右辺にboolean以外が出現しました。"));
        };

        Ok(Value::Boolean(lhs && rhs))
    }

    fn or(lhs: Value, rhs: Value) -> anyhow::Result<Value> {
        let lhs = if let Value::Boolean(b) = lhs {
            b
        } else {
            return Err(Error::msg("orの左辺にboolean以外が出現しました。"));
        };

        let rhs = if let Value::Boolean(b) = rhs {
            b
        } else {
            return Err(Error::msg("orの右辺にboolean以外が出現しました。"));
        };

        Ok(Value::Boolean(lhs || rhs))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Unary {
    expr: Box<Expr>,
    op: UnaryOp,
}

impl Unary {
    pub fn new(expr: Box<Expr>, op: UnaryOp) -> Self {
        Self {
            expr,
            op,
        }
    }

    pub fn calc(&self, env: &Environment) -> anyhow::Result<Value> {
        let value = eval(*self.expr.clone(), env)?;

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

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Value {
    Unit,
    Number(i64),
    Boolean(bool),
}
