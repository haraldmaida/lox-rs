use crate::source::Location;
use std::fmt::{self, Debug, Display};
use std::io;
use std::str::Chars;

pub trait Tokenize<'a> {
    fn tokenize(self) -> Tokens<'a>;
}

impl<'a> Tokenize<'a> for &'a str {
    fn tokenize(self) -> Tokens<'a> {
        Tokens::new(self)
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenizeErrorCode {
    IoError(String),
    UnexpectedCharacter(char),
}

impl Display for TokenizeErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(message) => write!(f, "{message}"),
            Self::UnexpectedCharacter(chr) => write!(f, "unexpected character '{chr}'"),
        }
    }
}

impl From<io::Error> for TokenizeErrorCode {
    fn from(value: io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
#[error("{code} at {location}")]
pub struct TokenizeError {
    code: TokenizeErrorCode,
    location: Location,
}

impl TokenizeError {
    pub const fn code(&self) -> &TokenizeErrorCode {
        &self.code
    }

    pub const fn location(&self) -> Location {
        self.location
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Token {
    EndOfFile,
    Comma,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Dot,
    Semicolon,
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EndOfFile => write!(f, "EOF  null"),
            Self::Comma => write!(f, "COMMA , null"),
            Self::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Self::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Self::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Self::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Self::Dot => write!(f, "DOT . null"),
            Self::Semicolon => write!(f, "SEMICOLON ; null"),
        }
    }
}

pub struct TokenizeResult {
    tokens: Vec<Token>,
    errors: Vec<TokenizeError>,
}

impl TokenizeResult {
    pub const fn new(tokens: Vec<Token>, errors: Vec<TokenizeError>) -> Self {
        Self { tokens, errors }
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn errors(&self) -> &[TokenizeError] {
        &self.errors
    }

    pub fn into_result(self) -> Result<Vec<Token>, Vec<TokenizeError>> {
        if self.errors.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.errors)
        }
    }
}

impl FromIterator<Result<Token, TokenizeError>> for TokenizeResult {
    fn from_iter<I: IntoIterator<Item = Result<Token, TokenizeError>>>(iter: I) -> Self {
        let iterator = iter.into_iter();
        let (lower_bound, _) = iterator.size_hint();

        iterator.fold(
            Self {
                tokens: Vec::with_capacity(lower_bound),
                errors: Vec::new(),
            },
            |mut acc, item| {
                match item {
                    Ok(token) => acc.tokens.push(token),
                    Err(error) => acc.errors.push(error),
                }
                acc
            },
        )
    }
}

pub struct Tokens<'a> {
    source: Chars<'a>,
    location: Location,
    end_of_file: bool,
}

impl<'a> Tokens<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars(),
            location: Location::default(),
            end_of_file: false,
        }
    }
}

impl Iterator for Tokens<'_> {
    type Item = Result<Token, TokenizeError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next() {
            None => {
                if self.end_of_file {
                    None
                } else {
                    self.end_of_file = true;
                    Some(Ok(Token::EndOfFile))
                }
            },
            Some(chr) => {
                self.location.advance_char();
                match chr {
                    ',' => Some(Ok(Token::Comma)),
                    '(' => Some(Ok(Token::LeftParen)),
                    ')' => Some(Ok(Token::RightParen)),
                    '{' => Some(Ok(Token::LeftBrace)),
                    '}' => Some(Ok(Token::RightBrace)),
                    '.' => Some(Ok(Token::Dot)),
                    ';' => Some(Ok(Token::Semicolon)),
                    _ => Some(Err(TokenizeError {
                        code: TokenizeErrorCode::UnexpectedCharacter(chr),
                        location: self.location,
                    })),
                }
            },
        }
    }
}

#[cfg(test)]
mod tests;
