use crate::data::{Symbol, Value};
use hashbrown::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum EnvironmentError {
    #[error("variable '{0}' is not defined")]
    UndefinedVariable(Symbol),
}

#[derive(Debug, Clone)]
pub struct Environment {
    node: Rc<RefCell<EnvironmentNode>>,
}

#[derive(Debug, Clone)]
struct EnvironmentNode {
    /// the environment enclosing this environment. only the root environment
    /// (global scope) has no parent (enclosing = None)
    enclosing: Option<Rc<RefCell<Self>>>,
    /// the definitions in this environment
    values: HashMap<Symbol, Value>,
}

impl Environment {
    pub fn new_global() -> Self {
        Self {
            node: Rc::new(RefCell::new(EnvironmentNode {
                enclosing: None,
                values: HashMap::default(),
            })),
        }
    }

    #[must_use]
    pub fn new_local(&self) -> Self {
        Self {
            node: Rc::new(RefCell::new(EnvironmentNode {
                enclosing: Some(self.node.clone()),
                values: HashMap::default(),
            })),
        }
    }

    #[must_use]
    pub fn global(&self) -> Self {
        Self {
            node: self.root_node(),
        }
    }

    fn root_node(&self) -> Rc<RefCell<EnvironmentNode>> {
        let mut current = self.node.clone();
        while let Some(parent) = {
            let node = current.borrow();
            node.enclosing.clone()
        } {
            current = parent;
        }
        current
    }

    #[must_use]
    pub fn enclosing(&self) -> Self {
        let enclosing_node = self
            .node
            .borrow()
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
    ) -> Result<Rc<RefCell<EnvironmentNode>>, EnvironmentError> {
        let mut current = Some(self.node.clone());
        while let Some(node) = current.take() {
            if node.borrow().values.contains_key(&name) {
                return Ok(node);
            }
            current.clone_from(&node.borrow().enclosing);
        }
        Err(EnvironmentError::UndefinedVariable(name))
    }

    pub fn define(&self, name: impl Into<Symbol>, value: impl Into<Value>) {
        let name = name.into();
        self.node.borrow_mut().values.insert(name, value.into());
    }

    pub fn assign(
        &self,
        name: impl Into<Symbol>,
        value: impl Into<Value>,
    ) -> Result<(), EnvironmentError> {
        let name = name.into();
        let node = self.find_first_node_with_symbol_in_scope(name)?;
        node.borrow_mut().values.insert(name, value.into());
        Ok(())
    }

    pub fn lookup(&self, name: impl Into<Symbol>) -> Result<Value, EnvironmentError> {
        let name = name.into();
        let node = self.find_first_node_with_symbol_in_scope(name)?;
        node.borrow()
            .values
            .get(&name)
            .cloned()
            .ok_or(EnvironmentError::UndefinedVariable(name))
    }
}

#[cfg(test)]
mod tests;
