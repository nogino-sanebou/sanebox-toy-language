use crate::eval::eval_all;
use crate::lexer::lexer;
use crate::parser::Parser;

mod lexer;
mod parser;
mod eval;
mod ast;

fn main() -> anyhow::Result<()> {
    let tokens = lexer("println(123);");
    let mut parser = Parser::new(tokens);
    let exprs = parser.parse()?;
    let _ = eval_all(exprs)?;

    Ok(())
}