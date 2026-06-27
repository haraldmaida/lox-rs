use crate::data::{HashMap, Symbol, Value};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum EnvironmentError {
    #[error("definition for identifier '{0}' not found")]
    IdentifierNotFound(Symbol),
}

#[derive(Debug, Clone)]
pub struct Environment {
    node: Rc<EnvironmentNode>,
}

#[derive(Debug)]
struct EnvironmentNode {
    /// the environment enclosing this environment. only the root environment
    /// (global scope) has no parent (enclosing = None)
    enclosing: Option<Rc<Self>>,
    /// the definitions in this environment
    values: RefCell<HashMap<Symbol, Value>>,
}

impl Environment {
    pub fn new_root() -> Self {
        Self {
            node: Rc::new(EnvironmentNode {
                enclosing: None,
                values: RefCell::default(),
            }),
        }
    }

    #[must_use]
    pub fn new_local(&self) -> Self {
        Self {
            node: Rc::new(EnvironmentNode {
                enclosing: Some(self.node.clone()),
                values: RefCell::default(),
            }),
        }
    }

    fn root_node(&self) -> Rc<EnvironmentNode> {
        let mut current = self.node.clone();
        while let Some(parent) = { current.enclosing.clone() } {
            current = parent;
        }
        current
    }

    #[must_use]
    pub fn enclosing(&self) -> Self {
        let enclosing_node = self
            .node
            .enclosing
            .clone()
            .unwrap_or_else(|| self.root_node());
        Self {
            node: enclosing_node,
        }
    }

    fn find_first_node_with_symbol_in_scope(
        &self,
        name: Symbol,
    ) -> Result<Rc<EnvironmentNode>, EnvironmentError> {
        let mut current = Some(self.node.clone());
        while let Some(node) = current.take() {
            if node.values.borrow().contains_key(&name) {
                return Ok(node);
            }
            current.clone_from(&node.enclosing);
        }
        Err(EnvironmentError::IdentifierNotFound(name))
    }

    pub fn define(&self, name: impl Into<Symbol>, value: impl Into<Value>) {
        let name = name.into();
        self.node.values.borrow_mut().insert(name, value.into());
    }

    pub fn assign(
        &self,
        name: impl Into<Symbol>,
        value: impl Into<Value>,
    ) -> Result<(), EnvironmentError> {
        let name = name.into();
        let node = self.find_first_node_with_symbol_in_scope(name)?;
        node.values.borrow_mut().insert(name, value.into());
        Ok(())
    }

    pub fn lookup(&self, name: impl Into<Symbol>) -> Result<Value, EnvironmentError> {
        let name = name.into();
        let node = self.find_first_node_with_symbol_in_scope(name)?;
        node.values
            .borrow()
            .get(&name)
            .cloned()
            .ok_or(EnvironmentError::IdentifierNotFound(name))
    }

    pub fn assign_at(
        &self,
        distance: usize,
        name: impl Into<Symbol>,
        value: impl Into<Value>,
    ) -> Result<(), EnvironmentError> {
        let name = name.into();
        let target_node = self
            .ancestors()
            .take(distance)
            .last()
            .unwrap_or_else(|| self.node.clone());
        target_node.values.borrow_mut().insert(name, value.into());
        Ok(())
    }

    pub fn lookup_at(
        &self,
        distance: usize,
        name: impl Into<Symbol>,
    ) -> Result<Value, EnvironmentError> {
        let name = name.into();
        let target_node = self
            .ancestors()
            .take(distance)
            .last()
            .unwrap_or_else(|| self.node.clone());
        let target_environment = Self { node: target_node };
        target_environment.lookup(name)
    }

    fn ancestors(&self) -> Ancestors {
        Ancestors {
            current: self.node.clone(),
        }
    }
}

struct Ancestors {
    current: Rc<EnvironmentNode>,
}

impl Iterator for Ancestors {
    type Item = Rc<EnvironmentNode>;

    fn next(&mut self) -> Option<Self::Item> {
        // we need to allow blocks in conditions, due to lifetime extension of the borrowed
        // field `current`
        // see also Clippy issue [15112](https://github.com/rust-lang/rust-clippy/issues/15112)
        #[allow(clippy::blocks_in_conditions)]
        match { self.current.enclosing.clone() } {
            None => None,
            Some(enclosing) => {
                self.current = enclosing.clone();
                Some(enclosing)
            },
        }
    }
}

#[cfg(test)]
mod tests;
