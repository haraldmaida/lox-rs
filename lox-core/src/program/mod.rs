#[cfg(any(test, feature = "dsl"))]
pub use dsl::*;

use crate::parse::SyntaxError;
use crate::resolver::{ResolutionMap, ResolverError};
use crate::stmt::Stmt;
use crate::tokenize::LexingError;
use miette::Diagnostic;

#[derive(thiserror::Error, Diagnostic, Debug, Clone, PartialEq, Eq)]
pub enum ProgramError {
    #[error(transparent)]
    LexingError(#[from] LexingError),
    #[error(transparent)]
    SyntaxError(#[from] SyntaxError),
    #[error(transparent)]
    ResolverError(#[from] ResolverError),
}

pub trait IntoProgram {
    fn into_program(self) -> Result<Program, Vec<ProgramError>>;
}

impl IntoProgram for (Vec<Stmt>, ResolutionMap) {
    fn into_program(self) -> Result<Program, Vec<ProgramError>> {
        Ok(Program::new(self.0, self.1))
    }
}

impl<E> IntoProgram for Result<(Vec<Stmt>, ResolutionMap), Vec<E>>
where
    E: Into<ProgramError>,
{
    fn into_program(self) -> Result<Program, Vec<ProgramError>> {
        self.map_err(|errors| errors.into_iter().map(Into::into).collect())
            .map(|(stmts, res_map)| Program::new(stmts, res_map))
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
    pub resolution_map: ResolutionMap,
}

impl Program {
    pub const fn new(statements: Vec<Stmt>, resolution_map: ResolutionMap) -> Self {
        Self {
            statements,
            resolution_map,
        }
    }

    pub fn statements(&self) -> &[Stmt] {
        &self.statements
    }

    pub const fn resolution_map(&self) -> &ResolutionMap {
        &self.resolution_map
    }
}

#[cfg(any(test, feature = "dsl"))]
mod dsl;
