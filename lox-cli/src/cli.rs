use std::process::{ExitCode, Termination};

#[derive(clap::Parser, Debug)]
#[command(name = "lox", version = "0.1.0", about = "A Lox interpreter")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// Tokenize a source code file and print the list of tokens
    Tokenize {
        /// Path to the source code file
        source: String,
    },
    /// Parse a source code file and print the AST
    Parse {
        /// Path to the source code file
        source: String,
    },
    /// Interpret the program in a source code file
    #[command(alias = "run")]
    Interpret {
        /// Path to the source code file
        source: String,
    },
    /// Start the REPL
    Repl,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliStatus {
    /// The CLI command completed successfully
    Success,
    /// The source file specified by the user cannot be read
    CannotReadSourceFile,
    /// Writing an output file failed.
    CannotWriteOutputFile,
    /// The CLI command was used incorrectly.
    /// (e.g., bad flag, missing argument, etc.)
    InvalidArgument,
    /// Failures that happen when running the user-provided program
    /// (either in the interpreter or the VM)
    RuntimeError,
    /// Syntax error in the source code
    SyntaxError,
    /// An unrecoverable error in the system, e.g., in the parser, interpreter,
    /// compiler, or VM.
    ///
    /// This is most likely a bug that needs to be fixed by the
    /// developers
    SystemError,
}

impl CliStatus {
    pub const fn code(self) -> u8 {
        match self {
            Self::Success => 0,
            Self::CannotReadSourceFile => 66,
            Self::CannotWriteOutputFile => 73,
            Self::InvalidArgument => 64,
            Self::RuntimeError | Self::SyntaxError => 65,
            Self::SystemError => 70,
        }
    }
}

impl Termination for CliStatus {
    fn report(self) -> ExitCode {
        ExitCode::from(self.code())
    }
}
