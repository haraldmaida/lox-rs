#[derive(clap::Parser, Debug)]
#[command(name = "lox", version = "0.1.0", about = "A Lox interpreter")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// Tokenize a source code file
    Tokenize {
        /// Path to the source code file
        source: String,
    },
}
