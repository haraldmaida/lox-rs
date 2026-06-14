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
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Tokenize { source } => {
            let mut source_code = open_source_reader(source)?;
            source_code.tokenize().for_each(|item| match item {
                Ok(token) => println!("{token}"),
                Err(error) => eprintln!("\n{error}\n"),
            });
        },
        Command::Parse { source } => {
            let mut source_code = open_source_reader(source)?;
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

fn open_source_reader(source_file: impl AsRef<Path>) -> Result<BufReader<impl Read>, io::Error> {
    let file = File::open(source_file)?;
    Ok(BufReader::new(file))
}
