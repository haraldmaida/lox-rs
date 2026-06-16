#![allow(clippy::print_stdout, clippy::print_stderr)]

// workaround for false positive 'unused crate dependencies' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
mod dummy_extern_uses {
    #[cfg(test)]
    use trycmd as _;
}

mod cli;

use crate::cli::{Cli, Command};
use clap::Parser;
use lox_core::ast_printer::AstPrinter;
use lox_core::parse::Parse;
use lox_core::tokenize::Tokenize;
use std::path::Path;
use std::{fs, io};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Tokenize { source } => {
            let source_code = read_source_file(source)?;
            source_code.tokenize().for_each(|item| match item {
                Ok(token) => println!("{token}"),
                Err(error) => eprintln!("\n{error}\n"),
            });
        },
        Command::Parse { source } => {
            let source_code = read_source_file(source)?;
            match source_code.tokenize().parse() {
                Ok(ast) => {
                    let mut output = String::new();
                    AstPrinter::print(&ast, &mut output)?;
                    println!("{output}");
                },
                Err(error) => eprintln!("\n{error}\n"),
            }
        },
    }

    Ok(())
}

fn read_source_file(source_file: impl AsRef<Path>) -> Result<String, io::Error> {
    fs::read_to_string(source_file)
}
