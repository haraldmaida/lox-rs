use crate::data::{Symbol, Value};
use hashbrown::HashMap;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum EnvironmentError {
    #[error("variable '{0}' is not defined")]
    UndefinedVariable(Symbol),
}

#[derive(Default, Debug)]
struct Scope {
    parent: Option<usize>,
    values: HashMap<Symbol, Value>,
}

impl Scope {
    fn new(parent_scope_index: usize) -> Self {
        Self {
            parent: Some(parent_scope_index),
            values: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Environment {
    scopes: Vec<Option<Scope>>,
    active_index: usize,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            scopes: vec![Some(Scope::default())],
            active_index: 0,
        }
    }
}

impl Environment {
    pub fn create_new_scope(&mut self) {
        let new_parent_index = self.active_index;
        let new_scope = Scope::new(new_parent_index);
        self.scopes.push(Some(new_scope));
        self.active_index = self.scopes.len() - 1;
    }

    pub fn destroy_current_scope(&mut self) {
        if self.active_index == 0 {
            return;
        }
        let current_scope = self.scopes[self.active_index].take();
        let parent_index = current_scope.and_then(|scope| scope.parent).unwrap_or(0);
        self.active_index = parent_index;
    }

    fn active_scope_mut(&mut self) -> &mut Scope {
        self.scopes[self.active_index].as_mut().unwrap_or_else(|| {
            unreachable!("current scope is not set correctly. please file a bug.")
        })
    }

    pub fn define(&mut self, symbol: impl Into<Symbol>, value: impl Into<Value>) {
        self.active_scope_mut()
            .values
            .insert(symbol.into(), value.into());
    }

    fn lookup_nearest_scope_containing_symbol(&self, symbol: Symbol) -> Option<usize> {
        let mut scope_index = Some(self.active_index);
        while let Some(index) = scope_index {
            if let Some(scope) = self.scopes[index].as_ref() {
                if scope.values.contains_key(&symbol) {
                    return Some(index);
                }
                scope_index = scope.parent;
            } else {
                unreachable!("current scope or parent scope has been destroyed. please file a bug.")
            }
        }
        None
    }

    fn lookup_symbol(&self, symbol: Symbol) -> Option<&Value> {
        if let Some(found_index) = self.lookup_nearest_scope_containing_symbol(symbol) {
            if let Some(scope) = self.scopes[found_index].as_ref() {
                return scope.values.get(&symbol);
            }
            unreachable!("current scope or parent scope has been destroyed. please file a bug.")
        } else {
            None
        }
    }

    fn lookup_symbol_mut(&mut self, symbol: Symbol) -> Option<&mut Value> {
        if let Some(found_index) = self.lookup_nearest_scope_containing_symbol(symbol) {
            if let Some(scope) = self.scopes[found_index].as_mut() {
                return scope.values.get_mut(&symbol);
            }
            unreachable!("current scope or parent scope has been destroyed. please file a bug.")
        } else {
            None
        }
    }

    pub fn get(&self, symbol: impl Into<Symbol>) -> Result<&Value, EnvironmentError> {
        let symbol = symbol.into();
        self.lookup_symbol(symbol)
            .ok_or(EnvironmentError::UndefinedVariable(symbol))
    }

    pub fn assign(
        &mut self,
        symbol: impl Into<Symbol>,
        value: impl Into<Value>,
    ) -> Result<(), EnvironmentError> {
        let symbol = symbol.into();
        if let Some(entry) = self.lookup_symbol_mut(symbol) {
            *entry = value.into();
            Ok(())
        } else {
            Err(EnvironmentError::UndefinedVariable(symbol))
        }
    }
}

#[cfg(test)]
mod tests;
