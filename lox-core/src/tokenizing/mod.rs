use crate::source::{Location, SourceCode};
use std::fmt::{self, Debug, Display};
use std::io;

pub trait Tokenize<'a>
where
    Self: SourceCode<'a>,
{
    fn tokenize(&'a mut self) -> Tokens<<Self as SourceCode<'a>>::Chars>;
}

impl<'a, S> Tokenize<'a> for S
where
    S: 'a + SourceCode<'a>,
{
    fn tokenize(&'a mut self) -> Tokens<<Self as SourceCode<'a>>::Chars> {
        Tokens::new(self.chars())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq)]
pub enum LexingErrorCode {
    IoError(String),
    CharacterAfterEndOfFile(char),
    UnexpectedCharacter(char),
}

impl Display for LexingErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(message) => write!(f, "{message}"),
            Self::CharacterAfterEndOfFile(chr) => write!(f, "character '{chr}' after end of file"),
            Self::UnexpectedCharacter(chr) => write!(f, "unexpected character '{chr}'"),
        }
    }
}

impl From<io::Error> for LexingErrorCode {
    fn from(value: io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
#[error("{code} at {location}")]
pub struct LexingError {
    code: LexingErrorCode,
    location: Location,
}

impl LexingError {
    pub const fn code(&self) -> &LexingErrorCode {
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
    Minus,
    Plus,
    Star,
    Slash,
    Bang,
    Equal,
    Less,
    Greater,
    BangEqual,
    EqualEqual,
    GreaterEqual,
    LessEqual,
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
            Self::Minus => write!(f, "MINUS - null"),
            Self::Plus => write!(f, "PLUS + null"),
            Self::Star => write!(f, "STAR * null"),
            Self::Slash => write!(f, "SLASH / null"),
            Self::Bang => write!(f, "BANG ! null"),
            Self::Equal => write!(f, "EQUAL = null"),
            Self::Less => write!(f, "LESS < null"),
            Self::Greater => write!(f, "GREATER > null"),
            Self::BangEqual => write!(f, "BANG_EQUAL != null"),
            Self::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            Self::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            Self::LessEqual => write!(f, "LESS_EQUAL <= null"),
        }
    }
}

pub struct LexingResult {
    tokens: Vec<Token>,
    errors: Vec<LexingError>,
}

impl LexingResult {
    pub const fn new(tokens: Vec<Token>, errors: Vec<LexingError>) -> Self {
        Self { tokens, errors }
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn errors(&self) -> &[LexingError] {
        &self.errors
    }

    pub fn into_result(self) -> Result<Vec<Token>, Vec<LexingError>> {
        if self.errors.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.errors)
        }
    }
}

impl FromIterator<Result<Token, LexingError>> for LexingResult {
    fn from_iter<I: IntoIterator<Item = Result<Token, LexingError>>>(iter: I) -> Self {
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

enum LexingState {
    Initial,
    MaybeBangEqual,
    MaybeEqualEqual,
    MaybeGreaterEqual,
    MaybeLessEqual,
    MaybeLineComment,
    LineComment,
    EndOfFile,
}

#[allow(clippy::derivable_impls)]
impl Default for LexingState {
    fn default() -> Self {
        Self::Initial
    }
}

pub struct Tokens<I> {
    source: I,
    location: Location,
    state: LexingState,
}

impl<I> Tokens<I>
where
    I: Iterator<Item = Result<char, io::Error>>,
{
    pub fn new(source: I) -> Self {
        Self {
            source,
            location: Location::default(),
            state: LexingState::default(),
        }
    }

    fn advance_to_next_char(&mut self) -> Option<Result<char, LexingError>> {
        let next_chr = self.source.next();
        match next_chr {
            None => None,
            Some(Ok('\n')) => {
                self.location.advance_line();
                Some(Ok('\n'))
            },
            Some(Ok(chr)) => {
                self.location.advance_char();
                Some(Ok(chr))
            },
            Some(Err(error)) => Some(Err(LexingError {
                code: error.into(),
                location: self.location,
            })),
        }
    }
}

impl<I> Iterator for Tokens<I>
where
    I: Iterator<Item = Result<char, io::Error>>,
{
    type Item = Result<Token, LexingError>;

    #[allow(clippy::too_many_lines)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_chr = match self.advance_to_next_char() {
                None => None,
                Some(Ok(chr)) => Some(chr),
                Some(Err(error)) => {
                    return Some(Err(error));
                },
            };
            match self.state {
                LexingState::Initial => match next_chr {
                    None => {
                        self.state = LexingState::EndOfFile;
                        return Some(Ok(Token::EndOfFile));
                    },
                    Some(chr) => match chr {
                        ',' => return Some(Ok(Token::Comma)),
                        '(' => return Some(Ok(Token::LeftParen)),
                        ')' => return Some(Ok(Token::RightParen)),
                        '{' => return Some(Ok(Token::LeftBrace)),
                        '}' => return Some(Ok(Token::RightBrace)),
                        '.' => return Some(Ok(Token::Dot)),
                        ';' => return Some(Ok(Token::Semicolon)),
                        '-' => return Some(Ok(Token::Minus)),
                        '+' => return Some(Ok(Token::Plus)),
                        '*' => return Some(Ok(Token::Star)),
                        '/' => {
                            self.state = LexingState::MaybeLineComment;
                        },
                        '!' => {
                            self.state = LexingState::MaybeBangEqual;
                        },
                        '=' => {
                            self.state = LexingState::MaybeEqualEqual;
                        },
                        '<' => {
                            self.state = LexingState::MaybeLessEqual;
                        },
                        '>' => {
                            self.state = LexingState::MaybeGreaterEqual;
                        },
                        _ if chr.is_whitespace() => {
                            // ignore whitespace
                        },
                        _ => {
                            return Some(Err(LexingError {
                                code: LexingErrorCode::UnexpectedCharacter(chr),
                                location: self.location,
                            }));
                        },
                    },
                },
                LexingState::MaybeBangEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(Token::Bang));
                    },
                    Some(chr) => match chr {
                        '=' => {
                            self.state = LexingState::Initial;
                            return Some(Ok(Token::BangEqual));
                        },
                        _ if chr.is_whitespace() => {
                            self.state = LexingState::Initial;
                            return Some(Ok(Token::Bang));
                        },
                        _ => {
                            return Some(Err(LexingError {
                                code: LexingErrorCode::UnexpectedCharacter(chr),
                                location: self.location,
                            }));
                        },
                    },
                },
                LexingState::MaybeEqualEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(Token::Equal));
                    },
                    Some(chr) => match chr {
                        '=' => {
                            self.state = LexingState::Initial;
                            return Some(Ok(Token::EqualEqual));
                        },
                        _ if chr.is_whitespace() => {
                            self.state = LexingState::Initial;
                            return Some(Ok(Token::Equal));
                        },
                        _ => {
                            return Some(Err(LexingError {
                                code: LexingErrorCode::UnexpectedCharacter(chr),
                                location: self.location,
                            }));
                        },
                    },
                },
                LexingState::MaybeGreaterEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(Token::Greater));
                    },
                    Some(chr) => match chr {
                        '=' => {
                            self.state = LexingState::Initial;
                            return Some(Ok(Token::GreaterEqual));
                        },
                        _ if chr.is_whitespace() => {
                            self.state = LexingState::Initial;
                            return Some(Ok(Token::Greater));
                        },
                        _ => {
                            return Some(Err(LexingError {
                                code: LexingErrorCode::UnexpectedCharacter(chr),
                                location: self.location,
                            }));
                        },
                    },
                },
                LexingState::MaybeLessEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(Token::Less));
                    },
                    Some(chr) => match chr {
                        '=' => {
                            self.state = LexingState::Initial;
                            return Some(Ok(Token::LessEqual));
                        },
                        _ if chr.is_whitespace() => {
                            self.state = LexingState::Initial;
                            return Some(Ok(Token::Less));
                        },
                        _ => {
                            return Some(Err(LexingError {
                                code: LexingErrorCode::UnexpectedCharacter(chr),
                                location: self.location,
                            }));
                        },
                    },
                },
                LexingState::MaybeLineComment => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(Token::Slash));
                    },
                    Some(chr) => match chr {
                        '/' => {
                            self.state = LexingState::LineComment;
                        },
                        _ if chr.is_whitespace() => {
                            self.state = LexingState::Initial;
                            return Some(Ok(Token::Slash));
                        },
                        _ => {
                            return Some(Err(LexingError {
                                code: LexingErrorCode::UnexpectedCharacter(chr),
                                location: self.location,
                            }));
                        },
                    },
                },
                LexingState::LineComment => match next_chr {
                    None => {
                        self.state = LexingState::EndOfFile;
                        return Some(Ok(Token::EndOfFile));
                    },
                    Some('\n') => {
                        self.state = LexingState::Initial;
                    },
                    Some(_) => {
                        // ignore characters in line comment
                    },
                },
                LexingState::EndOfFile => match next_chr {
                    None => return None,
                    Some(chr) => {
                        // this should be unreachable, but just in case
                        return Some(Err(LexingError {
                            code: LexingErrorCode::CharacterAfterEndOfFile(chr),
                            location: self.location,
                        }));
                    },
                },
            }
        }
    }
}

#[cfg(test)]
mod tests;
