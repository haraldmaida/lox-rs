use crate::data::{Symbol, Value};
use hashbrown::HashMap;

#[derive(Default, Debug)]
pub struct Environment {
    values: HashMap<Symbol, Value>,
}

impl Environment {
    pub fn define(&mut self, symbol: Symbol, value: Value) {
        self.values.insert(symbol, value);
    }

    pub fn get(&self, symbol: Symbol) -> Option<&Value> {
        self.values.get(&symbol)
    }
}

#[cfg(test)]
mod tests;
