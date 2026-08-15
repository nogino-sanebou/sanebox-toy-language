use sanebox::{eval_all, lexer, Parser};

fn main() -> anyhow::Result<()> {
    let tokens = lexer("println(123);");
    let mut parser = Parser::new(tokens);
    let exprs = parser.parse()?;
    let _ = eval_all(exprs)?;

    Ok(())
}