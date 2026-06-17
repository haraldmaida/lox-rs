use lasso::{Spur, ThreadedRodeo};
use std::fmt;
use std::fmt::Display;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Bool(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{value}"),
        }
    }
}

impl Value {
    /// Returns whether this value is true for all types.
    ///
    /// In dynamically typed languages, logical operations can be applied to any
    /// type of value. Each language defines what values are "truthy" and what
    /// are not.
    ///
    /// For Lox, we follow the simple rules of Lua and Ruby:
    ///
    /// * `false` and `nil` are falsy
    /// * everything else is truthy
    pub const fn is_truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Bool(value) => *value,
            _ => true,
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Number(f64::from(value))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

static SYMBOL_TABLE: LazyLock<ThreadedRodeo> = LazyLock::new(ThreadedRodeo::new);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol(Spur);

impl Symbol {
    pub fn intern(identifier: &str) -> Self {
        Self(SYMBOL_TABLE.get_or_intern(identifier))
    }

    pub fn as_str(&self) -> &'static str {
        SYMBOL_TABLE.resolve(&self.0)
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Self::intern(value)
    }
}

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Self::intern(&value)
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        SYMBOL_TABLE.resolve(&self.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(any(test, feature = "proptest_support"))]
mod proptest_support {
    use super::*;
    use proptest::arbitrary::{Arbitrary, any};
    use proptest::prop_oneof;
    use proptest::strategy::{BoxedStrategy, Just, Strategy};

    impl Arbitrary for Value {
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(Self::Nil),
                any::<bool>().prop_map(Self::Bool),
                any::<f64>().prop_map(Self::Number),
                any::<String>().prop_map(Self::String)
            ]
            .boxed()
        }

        type Strategy = BoxedStrategy<Self>;
    }
}

#[cfg(test)]
mod tests;
