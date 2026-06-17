use super::{Symbol, Value};

pub fn symbol(value: impl Into<Symbol>) -> Symbol {
    value.into()
}

pub fn value(value: impl Into<Value>) -> Value {
    value.into()
}

pub const fn nil() -> Value {
    Value::Nil
}

pub const fn true_() -> Value {
    Value::Bool(true)
}

pub const fn false_() -> Value {
    Value::Bool(false)
}

pub fn number(value: impl Into<f64>) -> Value {
    Value::Number(value.into())
}

pub fn string(value: impl Into<String>) -> Value {
    Value::String(value.into())
}
