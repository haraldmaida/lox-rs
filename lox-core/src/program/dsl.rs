use crate::parse::Parse;
use crate::program::{IntoProgram, Program, ProgramError};
use crate::resolver::Resolve;
use crate::tokenize::Tokenize;

pub fn program(source_code: impl AsRef<str>) -> Result<Program, Vec<ProgramError>> {
    source_code
        .tokenize()
        .parse()
        .map_err(|errors| errors.into_iter().map(ProgramError::from).collect())
        .and_then(|statements| {
            statements
                .resolve()
                .map_err(|errors| errors.into_iter().map(ProgramError::from).collect())
        })
        .into_program()
}
