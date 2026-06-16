pub mod ast_printer;
pub mod expr;
pub mod interpreter;
pub mod parse;
pub mod token;
pub mod tokenize;

pub type Result<T> = miette::Result<T>;
