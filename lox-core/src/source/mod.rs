use std::fmt::{self, Display};
use std::io;

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

    pub const fn advance_line(&mut self) {
        self.line += 1;
        self.char = 0;
    }
}

pub trait SourceCode<'a> {
    type Chars: Iterator<Item = Result<char, io::Error>>;

    fn chars(&'a mut self) -> Self::Chars;
}

mod buf_reader {
    use super::SourceCode;
    use std::io::{BufReader, Read};
    use utf8_chars::BufReadCharsExt;

    impl<'a, R> SourceCode<'a> for BufReader<R>
    where
        R: 'a + Read,
    {
        type Chars = utf8_chars::Chars<'a, Self>;

        fn chars(&'a mut self) -> Self::Chars {
            <Self as BufReadCharsExt>::chars(self)
        }
    }
}

mod str {
    use super::SourceCode;
    use std::io;

    impl<'a> SourceCode<'a> for &'a str {
        type Chars = StrChars<'a>;

        fn chars(&mut self) -> Self::Chars {
            StrChars {
                chars: str::chars(self),
            }
        }
    }

    pub struct StrChars<'a> {
        chars: std::str::Chars<'a>,
    }

    impl Iterator for StrChars<'_> {
        type Item = Result<char, io::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            self.chars.next().map(Ok)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.chars.size_hint()
        }
    }
}

#[cfg(test)]
mod tests;
