use crate::data::Value;
use crate::interpreter::RuntimeError;
use std::time::SystemTime;

pub fn clock(_params: &[Value]) -> Result<Value, RuntimeError> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
        .into())
}

#[cfg(test)]
mod tests;
