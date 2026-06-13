use crate::source::{Location, SourceCode};
use crate::token::{Literal, Token, TokenKind};
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
    StringStarted,
    NumberStarted,
    MaybeDecimalPoint,
    FractionDigitsStarted,
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
    state: LexingState,
    start_location: Location,
    current_lexeme: String,
    current_location: Location,
    open_chars: VecDeque<char>,
}

impl<I> Tokens<I>
where
    I: Iterator<Item = Result<char, io::Error>>,
{
    pub fn new(source: I) -> Self {
        Self {
            source,
            state: LexingState::default(),
            start_location: Location::default(),
            current_lexeme: String::new(),
            current_location: Location::default(),
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
                self.current_location.advance_line();
                Some(Ok('\n'))
            },
            Some(Ok(chr)) => {
                self.current_location.advance_char();
                Some(Ok(chr))
            },
            Some(Err(error)) => Some(Err(LexingError {
                code: error.into(),
                location: self.current_location,
            })),
        }
    }

    fn revert_char(&mut self, chr: char) {
        self.open_chars.push_back(chr);
        self.current_location.revert(1);
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
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::EndOfFile,
                            lexeme,
                            self.current_location,
                        )));
                    },
                    Some(chr) => {
                        self.current_lexeme.push(chr);
                        match chr {
                            ',' => {
                                let lexeme = mem::take(&mut self.current_lexeme);
                                return Some(Ok(nonliteral_token(
                                    TokenKind::Comma,
                                    lexeme,
                                    self.current_location,
                                )));
                            },
                            '(' => {
                                let lexeme = mem::take(&mut self.current_lexeme);
                                return Some(Ok(nonliteral_token(
                                    TokenKind::LeftParen,
                                    lexeme,
                                    self.current_location,
                                )));
                            },
                            ')' => {
                                let lexeme = mem::take(&mut self.current_lexeme);
                                return Some(Ok(nonliteral_token(
                                    TokenKind::RightParen,
                                    lexeme,
                                    self.current_location,
                                )));
                            },
                            '{' => {
                                let lexeme = mem::take(&mut self.current_lexeme);
                                return Some(Ok(nonliteral_token(
                                    TokenKind::LeftBrace,
                                    lexeme,
                                    self.current_location,
                                )));
                            },
                            '}' => {
                                let lexeme = mem::take(&mut self.current_lexeme);
                                return Some(Ok(nonliteral_token(
                                    TokenKind::RightBrace,
                                    lexeme,
                                    self.current_location,
                                )));
                            },
                            '.' => {
                                let lexeme = mem::take(&mut self.current_lexeme);
                                return Some(Ok(nonliteral_token(
                                    TokenKind::Dot,
                                    lexeme,
                                    self.current_location,
                                )));
                            },
                            ';' => {
                                let lexeme = mem::take(&mut self.current_lexeme);
                                return Some(Ok(nonliteral_token(
                                    TokenKind::Semicolon,
                                    lexeme,
                                    self.current_location,
                                )));
                            },
                            '-' => {
                                let lexeme = mem::take(&mut self.current_lexeme);
                                return Some(Ok(nonliteral_token(
                                    TokenKind::Minus,
                                    lexeme,
                                    self.current_location,
                                )));
                            },
                            '+' => {
                                let lexeme = mem::take(&mut self.current_lexeme);
                                return Some(Ok(nonliteral_token(
                                    TokenKind::Plus,
                                    lexeme,
                                    self.current_location,
                                )));
                            },
                            '*' => {
                                let lexeme = mem::take(&mut self.current_lexeme);
                                return Some(Ok(nonliteral_token(
                                    TokenKind::Star,
                                    lexeme,
                                    self.current_location,
                                )));
                            },
                            '/' => {
                                self.state = LexingState::MaybeLineComment;
                                self.start_location = self.current_location;
                            },
                            '!' => {
                                self.state = LexingState::MaybeBangEqual;
                                self.start_location = self.current_location;
                            },
                            '=' => {
                                self.state = LexingState::MaybeEqualEqual;
                                self.start_location = self.current_location;
                            },
                            '<' => {
                                self.state = LexingState::MaybeLessEqual;
                                self.start_location = self.current_location;
                            },
                            '>' => {
                                self.state = LexingState::MaybeGreaterEqual;
                                self.start_location = self.current_location;
                            },
                            '"' => {
                                self.state = LexingState::StringStarted;
                                self.start_location = self.current_location;
                            },
                            _ if chr.is_ascii_digit() => {
                                self.state = LexingState::NumberStarted;
                                self.start_location = self.current_location;
                            },
                            _ if chr.is_ascii_alphabetic() || chr == '_' => {
                                self.state = LexingState::Identifier;
                                self.start_location = self.current_location;
                            },
                            _ if chr.is_whitespace() => {
                                // ignore whitespace
                                self.current_lexeme.pop();
                            },
                            _ => {
                                self.current_lexeme.clear();
                                return Some(Err(LexingError {
                                    code: LexingErrorCode::UnexpectedCharacter(chr),
                                    location: self.current_location,
                                }));
                            },
                        }
                    },
                },
                LexingState::MaybeBangEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::Bang,
                            lexeme,
                            self.start_location,
                        )));
                    },
                    Some('=') => {
                        self.state = LexingState::Initial;
                        self.current_lexeme.push('=');
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::BangEqual,
                            lexeme,
                            self.start_location,
                        )));
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::Bang,
                            lexeme,
                            self.start_location,
                        )));
                    },
                },
                LexingState::MaybeEqualEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::Equal,
                            lexeme,
                            self.start_location,
                        )));
                    },
                    Some('=') => {
                        self.state = LexingState::Initial;
                        self.current_lexeme.push('=');
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::EqualEqual,
                            lexeme,
                            self.start_location,
                        )));
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::Equal,
                            lexeme,
                            self.start_location,
                        )));
                    },
                },
                LexingState::MaybeGreaterEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::Greater,
                            lexeme,
                            self.start_location,
                        )));
                    },
                    Some('=') => {
                        self.state = LexingState::Initial;
                        self.current_lexeme.push('=');
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::GreaterEqual,
                            lexeme,
                            self.start_location,
                        )));
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::Greater,
                            lexeme,
                            self.start_location,
                        )));
                    },
                },
                LexingState::MaybeLessEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::Less,
                            lexeme,
                            self.start_location,
                        )));
                    },
                    Some('=') => {
                        self.state = LexingState::Initial;
                        self.current_lexeme.push('=');
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::LessEqual,
                            lexeme,
                            self.start_location,
                        )));
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::Less,
                            lexeme,
                            self.start_location,
                        )));
                    },
                },
                LexingState::MaybeLineComment => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::Slash,
                            lexeme,
                            self.start_location,
                        )));
                    },
                    Some('/') => {
                        self.state = LexingState::LineComment;
                        self.current_lexeme.clear();
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(nonliteral_token(
                            TokenKind::Slash,
                            lexeme,
                            self.start_location,
                        )));
                    },
                },
                LexingState::LineComment => match next_chr {
                    None | Some('\n') => {
                        self.state = LexingState::Initial;
                    },
                    Some(_) => {
                        // ignore characters in line comment
                    },
                },
                LexingState::StringStarted => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Err(LexingError {
                            code: LexingErrorCode::UnterminatedStringLiteral(lexeme),
                            location: self.start_location,
                        }));
                    },
                    Some('"') => {
                        self.state = LexingState::Initial;
                        self.current_lexeme.push('"');
                        let lexeme = mem::take(&mut self.current_lexeme);
                        let value = lexeme.trim_matches('"').to_string();
                        return Some(Ok(string_literal(lexeme, value, self.start_location)));
                    },
                    Some(chr) => self.current_lexeme.push(chr),
                },
                LexingState::NumberStarted => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(parse_number_token(lexeme, self.start_location));
                    },
                    Some('.') => {
                        self.state = LexingState::MaybeDecimalPoint;
                    },
                    Some(chr) if chr.is_ascii_digit() => self.current_lexeme.push(chr),
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(parse_number_token(lexeme, self.start_location));
                    },
                },
                LexingState::MaybeDecimalPoint => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        self.current_lexeme.push('.');
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(parse_number_token(lexeme, self.start_location));
                    },
                    Some(chr) if chr.is_ascii_digit() => {
                        self.state = LexingState::FractionDigitsStarted;
                        self.current_lexeme.push('.');
                        self.current_lexeme.push(chr);
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        if chr.is_ascii_alphabetic() || chr == '_' {
                            self.revert_char('.');
                        } else {
                            self.current_lexeme.push('.');
                        }
                        self.revert_char(chr);
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(parse_number_token(lexeme, self.start_location));
                    },
                },
                LexingState::FractionDigitsStarted => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(parse_number_token(lexeme, self.start_location));
                    },
                    Some(chr) if chr.is_ascii_digit() => self.current_lexeme.push(chr),
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(parse_number_token(lexeme, self.start_location));
                    },
                },
                LexingState::Identifier => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(keyword_or_identifier(lexeme, self.start_location)));
                    },
                    Some(chr) if chr.is_ascii_alphanumeric() || chr == '_' => {
                        self.current_lexeme.push(chr);
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        let lexeme = mem::take(&mut self.current_lexeme);
                        return Some(Ok(keyword_or_identifier(lexeme, self.start_location)));
                    },
                },
                LexingState::EndOfFile => match next_chr {
                    None => return None,
                    Some(chr) => {
                        // this should be unreachable, but just in case
                        return Some(Err(LexingError {
                            code: LexingErrorCode::CharacterAfterEndOfFile(chr),
                            location: self.current_location,
                        }));
                    },
                },
            }
        }
    }
}

fn parse_number_token(lexeme: String, location: Location) -> Result<Token, LexingError> {
    match lexeme.parse::<f64>() {
        Ok(value) => Ok(number_literal(lexeme, value, location)),
        Err(_) => Err(LexingError {
            code: LexingErrorCode::InvalidNumberLiteral(lexeme),
            location,
        }),
    }
}

const fn nonliteral_token(kind: TokenKind, lexeme: String, location: Location) -> Token {
    Token::new(kind, None, lexeme, location)
}

const fn number_literal(lexeme: String, value: f64, location: Location) -> Token {
    Token::new(
        TokenKind::NumberLiteral,
        Some(Literal::Number(value)),
        lexeme,
        location,
    )
}

const fn string_literal(lexeme: String, value: String, location: Location) -> Token {
    Token::new(
        TokenKind::StringLiteral,
        Some(Literal::String(value)),
        lexeme,
        location,
    )
}

fn keyword_or_identifier(lexeme: String, location: Location) -> Token {
    let token_kind = match &lexeme[..] {
        "and" => TokenKind::And,
        "class" => TokenKind::Class,
        "else" => TokenKind::Else,
        "false" => TokenKind::False,
        "for" => TokenKind::For,
        "fun" => TokenKind::Fun,
        "if" => TokenKind::If,
        "nil" => TokenKind::Nil,
        "or" => TokenKind::Or,
        "print" => TokenKind::Print,
        "return" => TokenKind::Return,
        "super" => TokenKind::Super,
        "this" => TokenKind::This,
        "true" => TokenKind::True,
        "var" => TokenKind::Var,
        "while" => TokenKind::While,
        _ => TokenKind::Identifier,
    };
    Token::new(token_kind, None, lexeme, location)
}

#[cfg(test)]
mod tests;
