use crate::source::{Location, SourceCode};
use std::collections::VecDeque;
use std::fmt::{self, Debug, Display};
use std::{io, mem};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexingErrorCode {
    IoError(String),
    CharacterAfterEndOfFile(char),
    InvalidNumberLiteral(String),
    UnexpectedCharacter(char),
    UnterminatedStringLiteral(String),
}

impl Display for LexingErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(message) => write!(f, "{message}"),
            Self::CharacterAfterEndOfFile(chr) => write!(f, "character '{chr}' after end of file"),
            Self::InvalidNumberLiteral(value) => write!(f, "invalid number literal {value}"),
            Self::UnexpectedCharacter(chr) => write!(f, "unexpected character '{chr}'"),
            Self::UnterminatedStringLiteral(value) => {
                write!(f, "unterminated string literal \"{value}")
            },
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

#[derive(Clone, PartialEq)]
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
    StringLiteral(String),
    NumberLiteral(f64),
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
            Self::StringLiteral(value) => write!(f, "STRING_LITERAL \"{value}\" {value:?}"),
            Self::NumberLiteral(value) => write!(f, "NUMBER_LITERAL {value} {value:?}"),
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
    StringLiteral,
    NumberLiteral,
    MaybeDecimalPoint,
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
    literal: String,
    open_chars: VecDeque<char>,
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
            literal: String::new(),
            open_chars: VecDeque::with_capacity(2),
        }
    }

    fn advance_to_next_char(&mut self) -> Option<Result<char, LexingError>> {
        let next_chr = self
            .open_chars
            .pop_front()
            .map(Ok)
            .or_else(|| self.source.next());

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
                        '"' => {
                            self.state = LexingState::StringLiteral;
                            self.literal.clear();
                        },
                        _ if chr.is_ascii_digit() => {
                            self.state = LexingState::NumberLiteral;
                            self.literal.clear();
                            self.literal.push(chr);
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
                LexingState::StringLiteral => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        let lexeme = mem::take(&mut self.literal);
                        //TODO: should the location in the lexing error point to
                        // the beginning of the lexeme or the end?
                        return Some(Err(LexingError {
                            code: LexingErrorCode::UnterminatedStringLiteral(lexeme),
                            location: self.location,
                        }));
                    },
                    Some('"') => {
                        self.state = LexingState::Initial;
                        let value = mem::take(&mut self.literal);
                        return Some(Ok(Token::StringLiteral(value)));
                    },
                    Some(chr) => self.literal.push(chr),
                },
                LexingState::NumberLiteral => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        let str_value = mem::take(&mut self.literal);
                        match str_value.parse::<f64>() {
                            Ok(value) => {
                                return Some(Ok(Token::NumberLiteral(value)));
                            },
                            Err(_) => {
                                return Some(Err(LexingError {
                                    code: LexingErrorCode::InvalidNumberLiteral(str_value),
                                    location: self.location,
                                }));
                            },
                        }
                    },
                    Some('.') => {
                        self.state = LexingState::MaybeDecimalPoint;
                    },
                    Some(chr) if chr.is_ascii_digit() => self.literal.push(chr),
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.open_chars.push_back(chr);
                        let str_value = mem::take(&mut self.literal);
                        match str_value.parse::<f64>() {
                            Ok(value) => {
                                return Some(Ok(Token::NumberLiteral(value)));
                            },
                            Err(_) => {
                                return Some(Err(LexingError {
                                    code: LexingErrorCode::InvalidNumberLiteral(str_value),
                                    location: self.location,
                                }));
                            },
                        }
                    },
                },
                LexingState::MaybeDecimalPoint => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        self.literal.push('.');
                        let lexeme = mem::take(&mut self.literal);
                        return Some(Err(LexingError {
                            code: LexingErrorCode::InvalidNumberLiteral(lexeme),
                            location: self.location,
                        }));
                    },
                    Some(chr) if chr.is_ascii_digit() => {
                        self.state = LexingState::NumberLiteral;
                        self.literal.push('.');
                        self.literal.push(chr);
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.literal.push('.');
                        self.open_chars.push_back(chr);
                        //TODO: scan for function calls on number literal
                        // - returning lexing error for now
                        let lexeme = mem::take(&mut self.literal);
                        return Some(Err(LexingError {
                            code: LexingErrorCode::InvalidNumberLiteral(lexeme),
                            location: self.location,
                        }));
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
