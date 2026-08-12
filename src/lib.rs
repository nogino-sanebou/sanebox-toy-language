mod lexer;
mod parser;
mod ast;
mod eval;

pub use lexer::lexer;
pub use parser::Parser;
pub use eval::eval_all;
pub use ast::{Value, Stmt, Expr, Statements};