use lox_core::interpreter::Interpreter;
use lox_core::parse::Parse;
use lox_core::program::IntoProgram;
use lox_core::resolver::Resolve;
use lox_core::runtime::RuntimeContext;
use lox_core::tokenize::Tokenize;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};

pub fn run(
    stdin: impl Read,
    mut stdout: impl Write,
    mut stderr: impl Write,
) -> Result<(), io::Error> {
    let mut stdin = BufReader::new(stdin);

    writeln!(stderr, "Welcome to the Lox REPL Version 1.0.0!")?;
    writeln!(stderr, "Enter :quit to exit.")?;
    writeln!(stderr, "Enter :clear to reset the interpreter.")?;
    stdout.flush()?;

    let mut interpreter = Interpreter::default();
    let mut line = String::new();
    loop {
        write!(stderr, ">> ")?;
        stderr.flush()?;
        line.clear();
        stdin.read_line(&mut line)?;
        match line.trim() {
            ":quit" => {
                writeln!(stderr, ":: Quitting the REPL. Goodbye!")?;
                stderr.flush()?;
                break Ok(());
            },
            ":clear" => {
                interpreter = Interpreter::default();
                writeln!(stderr, ":: the state of the interpreter has been cleared.")?;
                stderr.flush()?;
                continue;
            },
            _ if line.is_empty() => continue,
            _ => {},
        }
        match line.tokenize().parse() {
            Ok(statements) => match statements.resolve().into_program() {
                Ok(program) => {
                    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
                    interpreter.interpret_chunk(&mut rtc, &program);
                },
                Err(resolution_errors) => {
                    for error in resolution_errors {
                        writeln!(stderr, "{error}")?;
                    }
                },
            },
            Err(syntax_errors) => {
                for error in syntax_errors {
                    writeln!(stderr, "{error}")?;
                }
            },
        }
        stdout.flush()?;
        stderr.flush()?;
    }
}

#[cfg(test)]
mod tests;
