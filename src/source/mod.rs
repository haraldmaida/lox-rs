use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub char: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self { line: 1, char: 0 }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.char)
    }
}

impl Location {
    pub const fn line(&self) -> usize {
        self.line
    }

    pub const fn char(&self) -> usize {
        self.char
    }

    pub const fn advance_char(&mut self) {
        self.char += 1;
    }
}

#[cfg(test)]
mod tests;
