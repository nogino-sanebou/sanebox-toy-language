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

    pub fn iter(&self) -> Iter<Stmt>{
        self.stmts.iter()
    }

    pub fn len(&self) -> usize {
        self.stmts.len()
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

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Expr {
    // Expr(Box<Expr>),
    Value(Value),
    Func(BuiltinFunc),
    Binary(Binary),
    Unary(Unary),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum BuiltinFunc {
    Print(Box<Expr>),
    Println(Box<Expr>),
    Abs(Box<Expr>),
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

    pub fn calc(&self) -> anyhow::Result<Value> {
        let lhs = eval(*self.lhs.clone())?;
        let rhs = eval(*self.rhs.clone())?;

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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum UnaryOp {
    Neg,
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

    pub fn calc(&self) -> anyhow::Result<Value> {
        let value = eval(*self.expr.clone())?;

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
    // Boolean(bool),
}
