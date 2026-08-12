use anyhow::Error;

use crate::ast::*;

pub fn print(value: Value) -> anyhow::Result<Value> {
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

pub fn println(value: Value) -> anyhow::Result<Value> {
    print(value)?;
    println!();

    Ok(Value::Unit)
}

pub fn abs(value: Value) -> anyhow::Result<Value> {
    match value {
        Value::Number(num) => {
            let num = num
                .checked_abs()
                .ok_or_else(|| Error::msg("absでオーバーフローしました。"))?;

            Ok(Value::Number(num))
        },
        _ => {
            Err(Error::msg("数値以外が出現しました。Abs"))
        },
    }
}

pub fn eval(expr: Expr) -> anyhow::Result<Value> {
    match expr {
        // Expr::Expr(expr) => {
        //     expr.eval()
        // },
        Expr::Value(value) => {
            match value {
                Value::Number(num) => {
                    Ok(Value::Number(num))
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
                    let r = eval(*expr)?;
                    let r = print(r)?;
                    Ok(r)
                },
                BuiltinFunc::Println(expr) => {
                    let r = eval(*expr)?;
                    let r = println(r)?;
                    Ok(r)
                },
                BuiltinFunc::Abs(expr) => {
                    let r = abs(eval(*expr)?)?;
                    Ok(r)
                },
            }
        },
        Expr::Unary(unary) => {
            Ok(unary.calc()?)
        },
    }
}

pub fn eval_all(stmts: Statements) -> anyhow::Result<Vec<Value>> {
    let mut values = Vec::new();

    for stmt in stmts.iter() {
        // 文の内容によって処理を変える
        match stmt {
            // 式、関数の処理
            Stmt::Expr(expr) => {
                let value = eval(expr.clone())?;
                values.push(value);
            },
            // let, if, forなどの予約語の処理
            _ => {
                return Err(Error::msg("未実装の文です。"));
            }
        }
    }

    Ok(values)
}