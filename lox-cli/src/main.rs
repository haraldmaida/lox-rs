#![allow(clippy::print_stdout, clippy::print_stderr)]

// workaround for false positive 'unused crate dependencies' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
mod dummy_extern_uses {
    #[cfg(test)]
    use trycmd as _;
}

mod cli;
mod repl;

use crate::cli::{Cli, CliStatus, Command};
use clap::Parser;
use lox_core::ast_printer::AstPrinter;
use lox_core::interpreter::Interpreter;
use lox_core::parse::Parse;
use lox_core::program::{IntoProgram, Program, ProgramError};
use lox_core::resolver::Resolve;
use lox_core::runtime::RuntimeContext;
use lox_core::tokenize::Tokenize;
use miette::{NamedSource, miette};
use std::io::{Stderr, Stdout};
use std::ops::ControlFlow;
use std::ops::ControlFlow::{Break, Continue};
use std::path::Path;
use std::{fs, io};

fn main() -> CliStatus {
    let cli = match parse_cli_args() {
        Continue(args) => args,
        Break(exit_status) => return exit_status,
    };

    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    match &cli.command {
        Command::Tokenize { source } => {
            let source_code = match read_source_file(source) {
                Continue(code) => code,
                Break(exit_status) => {
                    return exit_status;
                },
            };
            tokenize_source_code(source, &source_code)
        },
        Command::Parse { source } => {
            let source_code = match read_source_file(source) {
                Continue(code) => code,
                Break(exit_status) => return exit_status,
            };
            parse_and_print_ast(&mut stdout, &mut stderr, source, &source_code)
        },
        Command::Interpret { source } => {
            let source_code = match read_source_file(source) {
                Continue(code) => code,
                Break(exit_status) => return exit_status,
            };
            let program = match parse_program(source, &source_code) {
                Continue(program) => program,
                Break(exit_status) => return exit_status,
            };
            interpret_program(&mut stdout, &mut stderr, source, &source_code, &program)
        },
        Command::Repl => match repl::run(io::stdin(), &mut stdout, &mut stderr) {
            Ok(()) => CliStatus::Success,
            Err(error) => {
                eprintln!("{error:?}");
                CliStatus::SystemError
            },
        },
    }
}

fn parse_cli_args() -> ControlFlow<CliStatus, Cli> {
    match Cli::try_parse() {
        Ok(args) => Continue(args),
        Err(clap_err) => {
            if clap_err.use_stderr() {
                clap_err
                    .print()
                    .expect("failed to print error message from parsing the CLI arguments");
                Break(CliStatus::InvalidArgument)
            } else {
                clap_err
                    .print()
                    .expect("failed to print help message for CLI usage");
                Break(CliStatus::Success)
            }
        },
    }
}

fn read_source_file(source_file: impl AsRef<Path>) -> ControlFlow<CliStatus, String> {
    let source_file = source_file.as_ref();
    match fs::read_to_string(source_file) {
        Ok(source_code) => Continue(source_code),
        Err(error) => {
            let report = miette!(error).wrap_err(format!(
                "failed to read source file {}",
                source_file.display()
            ));
            eprintln!("{report:?}");
            Break(CliStatus::CannotReadSourceFile)
        },
    }
}

fn tokenize_source_code(source: &str, source_code: &str) -> CliStatus {
    let mut cli_status = CliStatus::Success;
    for item in source_code.tokenize() {
        match item {
            Ok(token) => {
                println!("{token:#}");
            },
            Err(error) => {
                let error = miette!(error)
                    .with_source_code(NamedSource::new(source, source_code.to_owned()));
                eprintln!("{error:?}");
                cli_status = CliStatus::SyntaxError;
            },
        }
    }
    cli_status
}

fn parse_and_print_ast(
    mut stdout: &mut Stdout,
    mut stderr: &mut Stderr,
    source: &str,
    source_code: &str,
) -> CliStatus {
    match source_code.tokenize().parse_expr() {
        Ok(ast) => {
            let mut output = String::new();
            let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr)
                .with_fancy_error_messages(Some(source), source_code);
            match AstPrinter::print(&mut rtc, &ast, &mut output) {
                Ok(()) => {
                    println!("{output}");
                    CliStatus::Success
                },
                Err(error) => {
                    let error = miette!(error);
                    eprintln!("{error:?}");
                    CliStatus::SystemError
                },
            }
        },
        Err(error) => {
            let error =
                miette!(error).with_source_code(NamedSource::new(source, source_code.to_owned()));
            eprintln!("{error:?}");
            CliStatus::SyntaxError
        },
    }
}

fn parse_program(source: &str, source_code: &str) -> ControlFlow<CliStatus, Program> {
    match source_code
        .tokenize()
        .parse()
        .map_err(|errors| errors.into_iter().map(ProgramError::from).collect())
        .and_then(|statements| {
            statements
                .resolve()
                .map_err(|errors| errors.into_iter().map(ProgramError::from).collect())
        })
        .into_program()
    {
        Ok(program) => Continue(program),
        Err(syntax_errors) => {
            for error in syntax_errors {
                let error = miette!(error)
                    .with_source_code(NamedSource::new(source, source_code.to_owned()));
                eprintln!("{error:?}");
            }
            Break(CliStatus::SyntaxError)
        },
    }
}

fn interpret_program(
    mut stdout: &mut Stdout,
    mut stderr: &mut Stderr,
    source: &str,
    source_code: &str,
    program: &Program,
) -> CliStatus {
    let mut interpreter = Interpreter::default();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr)
        .with_fancy_error_messages(Some(source), source_code);
    interpreter.interpret(&mut rtc, program);
    CliStatus::Success
}
