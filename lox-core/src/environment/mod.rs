use crate::data::{Symbol, Value};
use hashbrown::HashMap;
use std::cell::RefCell;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum EnvironmentError {
    #[error("variable '{0}' is not defined")]
    UndefinedVariable(Symbol),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Environment {
    index: usize,
}

struct EnvironmentData {
    /// the environment enclosing this environment. only the root environment
    /// (global scope) has no parent (enclosing = None)
    enclosing: Option<Environment>,
    /// number of enclosed environments that reference this environment
    ref_count: usize,
    /// the definitions in this environment
    values: HashMap<Symbol, Value>,
}

/// Arena holds all created environments in a `Vec` on the heap.
#[derive(Default)]
struct Arena {
    /// all environments in this arena
    slots: Vec<Option<EnvironmentData>>,
    /// the index of slots that have been freed and can be reused
    free_slots: Vec<usize>,
}

thread_local! {
    static ARENA: RefCell<Arena> =RefCell::new(Arena::default());
}

impl Environment {
    pub fn global() -> Self {
        ARENA.with(|arena| {
            let mut arena = arena.borrow_mut();
            match arena.slots.get_mut(0) {
                None => arena.slots.push(Some(EnvironmentData {
                    enclosing: None,
                    ref_count: 1,
                    values: HashMap::default(),
                })),
                Some(None) => {
                    arena.slots[0] = Some(EnvironmentData {
                        enclosing: None,
                        ref_count: 1,
                        values: HashMap::default(),
                    });
                },
                Some(Some(env_data)) => {
                    env_data.ref_count += 1;
                },
            }
            Self { index: 0 }
        })
    }

    #[must_use]
    pub fn new_local(&self) -> Self {
        // the Self::clone() increments the reference count in the parent environment
        let new_parent = self.clone();
        ARENA.with(|arena| {
            let mut arena = arena.borrow_mut();
            Self::create_child_environment(&mut arena, new_parent)
        })
    }

    #[must_use]
    pub fn enclosing(&self) -> Self {
        ARENA.with(|arena| {
            let mut arena = arena.borrow_mut();
            let parent_idx = match arena.slots.get(self.index) {
                None | Some(None) => Self::global().index,
                Some(Some(env_data)) => env_data
                    .enclosing
                    .as_ref()
                    .map_or_else(|| Self::global().index, |p| p.index),
            };
            if let Some(Some(parent_data)) = arena.slots.get_mut(parent_idx) {
                parent_data.ref_count += 1;
            }
            Self { index: parent_idx }
        })
    }

    fn create_child_environment(arena: &mut Arena, parent: Self) -> Self {
        let new_environment_data = EnvironmentData {
            enclosing: Some(parent),
            ref_count: 1, // the handle we are returning is the first reference
            values: HashMap::default(),
        };
        let new_index = if let Some(free_slot) = arena.free_slots.pop() {
            arena.slots[free_slot] = Some(new_environment_data);
            free_slot
        } else {
            arena.slots.push(Some(new_environment_data));
            arena.slots.len() - 1
        };
        Self { index: new_index }
    }

    fn parent_index(&self, arena: &Arena) -> Option<usize> {
        arena.slots.get(self.index).and_then(|slot| {
            slot.as_ref()
                .and_then(|s| s.enclosing.as_ref().map(|p| p.index))
        })
    }

    fn increment_ref_count_in_parent(&self, arena: &mut Arena) {
        if let Some(parent_idx) = self.parent_index(arena)
            && let Some(Some(parent_data)) = arena.slots.get_mut(parent_idx)
        {
            parent_data.ref_count += 1;
        }
    }

    fn decrement_ref_count_in_parent(&self, arena: &mut Arena) {
        if let Some(parent_idx) = self.parent_index(arena)
            && let Some(Some(parent_data)) = arena.slots.get_mut(parent_idx)
        {
            parent_data.ref_count -= 1;
        }
    }

    fn decrement_ref_count_and_free_up_unused_slots(&self, arena: &mut Arena) {
        if let Some(parent_idx) = self.parent_index(arena) {
            let mut slots_to_process = vec![parent_idx];
            while let Some(index) = slots_to_process.pop() {
                if let Some(Some(env_data)) = arena.slots.get_mut(index) {
                    if let Some(next_parent) = env_data.enclosing.as_ref() {
                        slots_to_process.push(next_parent.index);
                    }
                    if env_data.ref_count <= 1 {
                        // free up the slot
                        arena.slots[index] = None;
                        // add freed slot to unused slots for reuse
                        arena.free_slots.push(index);
                    } else {
                        env_data.ref_count -= 1;
                    }
                }
            }
        }
    }
}

// when the user clones the environment, we must increment the reference count
impl Clone for Environment {
    fn clone(&self) -> Self {
        ARENA.with(|arena| {
            let mut arena = arena.borrow_mut();
            self.increment_ref_count_in_parent(&mut arena);
        });
        Self { index: self.index }
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        ARENA.with(|arena| {
            let mut arena = arena.borrow_mut();
            self.decrement_ref_count_in_parent(&mut arena);
            // due to possibly recursive drops, just decrement the reference count
            // the method `Environment::new_local` should free up unused slots.
            // self.decrement_ref_count_and_free_up_unused_slots(&mut arena);
        });
    }
}

impl Environment {
    pub fn define(&self, name: impl Into<Symbol>, value: impl Into<Value>) {
        ARENA.with(|arena| {
            let mut arena = arena.borrow_mut();
            let env_data = arena.slots[self.index].as_mut()
                .expect("there should not be an environment that has no data in the arena. please file a bug.");
            env_data.values.insert(name.into(), value.into());
        });
    }

    pub fn assign(
        &self,
        name: impl Into<Symbol>,
        value: impl Into<Value>,
    ) -> Result<(), EnvironmentError> {
        let name = name.into();
        ARENA.with(|arena| {
            let mut arena = arena.borrow_mut();
            if let Some(found_at_idx) =
                lookup_first_environment_with_symbol_in_scope(&arena, self, name)
            {
                if let Some(Some(env_data)) = arena.slots.get_mut(found_at_idx) {
                    if let Some(variable) = env_data.values.get_mut(&name) {
                        *variable = value.into();
                        Ok(())
                    } else {
                        Err(EnvironmentError::UndefinedVariable(name))
                    }
                } else {
                    unreachable!("we have just before looked up the environment, containing the given symbol. please file a bug.")
                }
            } else {
                Err(EnvironmentError::UndefinedVariable(name))
            }
        })
    }

    pub fn lookup(&self, name: impl Into<Symbol>) -> Result<Value, EnvironmentError> {
        let name = name.into();
        ARENA.with(|arena| {
            let arena = arena.borrow();
            if let Some(Some(env_data)) =
                lookup_first_environment_with_symbol_in_scope(&arena, self, name)
                    .and_then(|found_at_idx| arena.slots.get(found_at_idx))
            {
                env_data
                    .values
                    .get(&name)
                    .cloned()
                    .ok_or(EnvironmentError::UndefinedVariable(name))
            } else {
                Err(EnvironmentError::UndefinedVariable(name))
            }
        })
    }
}

fn lookup_first_environment_with_symbol_in_scope(
    arena: &Arena,
    current: &Environment,
    symbol: Symbol,
) -> Option<usize> {
    let mut current_idx = Some(current.index);
    while let Some(index) = current_idx {
        if let Some(Some(env_data)) = arena.slots.get(index) {
            if env_data.values.contains_key(&symbol) {
                return Some(index);
            }
            current_idx = env_data.enclosing.as_ref().map(|p| p.index);
        } else {
            // fallback just in case the arena is corrupted - should not happen, though
            break;
        }
    }
    None
}

#[cfg(test)]
mod tests;
