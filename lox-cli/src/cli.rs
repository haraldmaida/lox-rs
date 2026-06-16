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
}
