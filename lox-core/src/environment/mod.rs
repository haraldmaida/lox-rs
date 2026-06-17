use crate::data::{Symbol, Value};
use hashbrown::HashMap;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum EnvironmentError {
    #[error("variable '{0}' is not defined")]
    UndefinedVariable(Symbol),
}

#[derive(Default, Debug)]
pub struct Environment {
    values: HashMap<Symbol, Value>,
}

impl Environment {
    pub fn define(&mut self, symbol: Symbol, value: Value) {
        self.values.insert(symbol, value);
    }

    pub fn get(&self, symbol: Symbol) -> Result<&Value, EnvironmentError> {
        self.values
            .get(&symbol)
            .ok_or(EnvironmentError::UndefinedVariable(symbol))
    }

    pub fn assign(&mut self, symbol: Symbol, value: Value) -> Result<(), EnvironmentError> {
        if let Some(entry) = self.values.get_mut(&symbol) {
            *entry = value;
            Ok(())
        } else {
            Err(EnvironmentError::UndefinedVariable(symbol))
        }
    }
}

#[cfg(test)]
mod tests;
