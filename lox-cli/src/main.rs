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
use lox_core::interpreter::Interpreter;
use lox_core::parse::Parse;
use lox_core::runtime::RuntimeContext;
use lox_core::tokenize::Tokenize;
use miette::{IntoDiagnostic, NamedSource, Report, WrapErr};
use std::fs;
use std::path::Path;

fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);

    match &cli.command {
        Command::Tokenize { source } => {
            let source_code = read_source_file(source)?;
            source_code.tokenize().for_each(|item| match item {
                Ok(token) => println!("{token:#}"),
                Err(error) => {
                    let error = Report::from(error)
                        .with_source_code(NamedSource::new(source, source_code.clone()));
                    eprintln!("\n{error}\n");
                },
            });
        },
        Command::Parse { source } => {
            let source_code = read_source_file(source)?;
            match source_code.tokenize().parse_expr() {
                Ok(ast) => {
                    let mut output = String::new();
                    AstPrinter::print(&mut rtc, &ast, &mut output)?;
                    println!("{output}");
                },
                Err(error) => {
                    let error =
                        Report::from(error).with_source_code(NamedSource::new(source, source_code));
                    eprintln!("\n{error}\n");
                },
            }
        },
        Command::Interpret { source } => {
            let source_code = read_source_file(source)?;
            match source_code.tokenize().parse() {
                Ok(program) => {
                    let mut interpreter = Interpreter::default();
                    interpreter.interpret(&mut rtc, &program);
                },
                Err(syntax_errors) => {
                    for error in syntax_errors {
                        let error = Report::from(error)
                            .with_source_code(NamedSource::new(source, source_code.clone()));
                        eprintln!("\n{error}\n");
                    }
                },
            }
        },
    }

    Ok(())
}

fn read_source_file(source_file: impl AsRef<Path>) -> Result<String, miette::Error> {
    let source_file = source_file.as_ref();
    fs::read_to_string(source_file)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read source file {}", source_file.display()))
}
