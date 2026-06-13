use crate::source::{Location, SourceCode};
use crate::token::Token;
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

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
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
    Identifier,
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
    current_lexeme: String,
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
            current_lexeme: String::new(),
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
                        },
                        _ if chr.is_ascii_digit() => {
                            self.state = LexingState::NumberLiteral;
                            self.current_lexeme.push(chr);
                        },
                        _ if chr.is_ascii_alphabetic() || chr == '_' => {
                            self.state = LexingState::Identifier;
                            self.current_lexeme.push(chr);
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
                    Some('=') => {
                        self.state = LexingState::Initial;
                        return Some(Ok(Token::BangEqual));
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.open_chars.push_back(chr);
                        return Some(Ok(Token::Bang));
                    },
                },
                LexingState::MaybeEqualEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(Token::Equal));
                    },
                    Some('=') => {
                        self.state = LexingState::Initial;
                        return Some(Ok(Token::EqualEqual));
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.open_chars.push_back(chr);
                        return Some(Ok(Token::Equal));
                    },
                },
                LexingState::MaybeGreaterEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(Token::Greater));
                    },
                    Some('=') => {
                        self.state = LexingState::Initial;
                        return Some(Ok(Token::GreaterEqual));
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.open_chars.push_back(chr);
                        return Some(Ok(Token::Greater));
                    },
                },
                LexingState::MaybeLessEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(Token::Less));
                    },
                    Some('=') => {
                        self.state = LexingState::Initial;
                        return Some(Ok(Token::LessEqual));
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.open_chars.push_back(chr);
                        return Some(Ok(Token::Less));
                    },
                },
                LexingState::MaybeLineComment => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(Token::Slash));
                    },
                    Some('/') => {
                        self.state = LexingState::LineComment;
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.open_chars.push_back(chr);
                        return Some(Ok(Token::Slash));
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
                        let lexeme = mem::take(&mut self.current_lexeme);
                        //TODO: should the location in the lexing error point to
                        // the beginning of the lexeme or the end?
                        return Some(Err(LexingError {
                            code: LexingErrorCode::UnterminatedStringLiteral(lexeme),
                            location: self.location,
                        }));
                    },
                    Some('"') => {
                        self.state = LexingState::Initial;
                        let value = mem::take(&mut self.current_lexeme);
                        return Some(Ok(Token::StringLiteral(value)));
                    },
                    Some(chr) => self.current_lexeme.push(chr),
                },
                LexingState::NumberLiteral => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(parse_number_token(lexeme, self.location));
                    },
                    Some('.') => {
                        self.state = LexingState::MaybeDecimalPoint;
                    },
                    Some(chr) if chr.is_ascii_digit() => self.current_lexeme.push(chr),
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.open_chars.push_back(chr);
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(parse_number_token(lexeme, self.location));
                    },
                },
                LexingState::MaybeDecimalPoint => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(parse_number_token(lexeme, self.location));
                    },
                    Some(chr) if chr.is_ascii_digit() => {
                        self.state = LexingState::NumberLiteral;
                        self.current_lexeme.push('.');
                        self.current_lexeme.push(chr);
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        if chr.is_ascii_alphabetic() || chr == '_' {
                            self.open_chars.push_back('.');
                        }
                        self.open_chars.push_back(chr);
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(parse_number_token(lexeme, self.location));
                    },
                },
                LexingState::Identifier => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        let identifier = mem::take(&mut self.current_lexeme);
                        return Some(Ok(Token::Identifier(identifier)));
                    },
                    Some(chr) if chr.is_ascii_alphanumeric() || chr == '_' => {
                        self.current_lexeme.push(chr);
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.open_chars.push_back(chr);
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(keyword_or_identifier(lexeme)));
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

fn parse_number_token(lexeme: String, location: Location) -> Result<Token, LexingError> {
    match lexeme.parse::<f64>() {
        Ok(value) => Ok(Token::NumberLiteral(value)),
        Err(_) => Err(LexingError {
            code: LexingErrorCode::InvalidNumberLiteral(lexeme),
            location,
        }),
    }
}

fn keyword_or_identifier(lexeme: String) -> Token {
    match &lexeme[..] {
        "and" => Token::And,
        "class" => Token::Class,
        "else" => Token::Else,
        "false" => Token::False,
        "for" => Token::For,
        "fun" => Token::Fun,
        "if" => Token::If,
        "nil" => Token::Nil,
        "or" => Token::Or,
        "print" => Token::Print,
        "return" => Token::Return,
        "super" => Token::Super,
        "this" => Token::This,
        "true" => Token::True,
        "var" => Token::Var,
        "while" => Token::While,
        _ => Token::Identifier(lexeme),
    }
}

#[cfg(test)]
mod tests;
