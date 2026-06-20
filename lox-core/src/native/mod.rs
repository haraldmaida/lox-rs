use crate::data::Value;
use std::time::SystemTime;

pub fn clock() -> Value {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
        .into()
}

#[cfg(test)]
mod tests;
