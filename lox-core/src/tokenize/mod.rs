use crate::token::{Literal, Token, TokenKind};
use miette::{Diagnostic, SourceSpan};
use std::fmt::{self, Debug, Display};
use std::io;

pub trait Tokenize<'a> {
    fn tokenize(&'a self) -> Tokens<'a>;
}

impl<'a, S> Tokenize<'a> for S
where
    S: AsRef<str>,
{
    fn tokenize(&'a self) -> Tokens<'a> {
        Tokens::new(self.as_ref())
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
            Self::CharacterAfterEndOfFile(chr) => {
                write!(f, "character '{chr}' after end of file detected")
            },
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

#[derive(thiserror::Error, Diagnostic, Debug, Clone, PartialEq, Eq)]
#[error("{code}")]
#[diagnostic()]
pub struct LexingError {
    pub(crate) code: LexingErrorCode,
    #[label]
    pub(crate) location: SourceSpan,
}

impl LexingError {
    pub const fn code(&self) -> &LexingErrorCode {
        &self.code
    }

    pub const fn location(&self) -> SourceSpan {
        self.location
    }
}

pub struct LexingResult<'a> {
    tokens: Vec<Token<'a>>,
    errors: Vec<LexingError>,
}

impl<'a> LexingResult<'a> {
    pub const fn new(tokens: Vec<Token<'a>>, errors: Vec<LexingError>) -> Self {
        Self { tokens, errors }
    }

    pub fn tokens(&self) -> &[Token<'a>] {
        &self.tokens
    }

    pub fn errors(&self) -> &[LexingError] {
        &self.errors
    }

    pub fn into_result(self) -> Result<Vec<Token<'a>>, Vec<LexingError>> {
        if self.errors.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.errors)
        }
    }
}

impl<'a> FromIterator<Result<Token<'a>, LexingError>> for LexingResult<'a> {
    fn from_iter<I: IntoIterator<Item = Result<Token<'a>, LexingError>>>(iter: I) -> Self {
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

/// A lexer for Lox.
///
/// The implementation uses a finite state machine.
pub struct Tokens<'a> {
    source: &'a str,
    state: LexingState,
    /// Byte offset of the start of the current lexeme.
    start_offset: usize,
    /// Byte offset of the next character.
    current_offset: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            state: LexingState::default(),
            start_offset: 0,
            current_offset: 0,
        }
    }
}

impl<'a> Tokens<'a> {
    fn advance_to_next_char(&mut self) -> Option<char> {
        self.source[self.current_offset..]
            .chars()
            .next()
            .inspect(|chr| {
                self.current_offset += chr.len_utf8();
            })
    }

    const fn revert_char(&mut self, chr: char) {
        self.current_offset -= chr.len_utf8();
    }

    fn start_location(&self) -> SourceSpan {
        (self.start_offset, self.current_offset - self.start_offset).into()
    }

    fn current_location(&self, length: usize) -> SourceSpan {
        (self.current_offset, length).into()
    }

    fn current_lexeme(&self) -> &'a str {
        &self.source[self.start_offset..self.current_offset]
    }

    fn nonliteral_token(&self, kind: TokenKind) -> Token<'a> {
        let location = self.start_location();
        let lexeme = self.current_lexeme();
        Token::new(kind, None, lexeme, location)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn unescape_string_token(&self) -> Result<Token<'a>, LexingError> {
        let location = self.start_location();
        let lexeme = self.current_lexeme();
        let value = Token::unescape(lexeme);
        Ok(Token::new(
            TokenKind::StringLiteral,
            Some(Literal::String(value.into())),
            lexeme,
            location,
        ))
    }

    fn parse_number_token(&self) -> Result<Token<'a>, LexingError> {
        let location = self.start_location();
        let lexeme = self.current_lexeme();
        match lexeme.parse::<f64>() {
            Ok(value) => Ok(Token::new(
                TokenKind::NumberLiteral,
                Some(Literal::Number(value)),
                lexeme,
                location,
            )),
            Err(_) => Err(LexingError {
                code: LexingErrorCode::InvalidNumberLiteral(lexeme.into()),
                location,
            }),
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Token<'a>, LexingError>;

    #[allow(clippy::too_many_lines)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_chr = self.advance_to_next_char();
            match self.state {
                LexingState::Initial => match next_chr {
                    None => {
                        self.state = LexingState::EndOfFile;
                        return Some(Ok(Token::new(
                            TokenKind::EndOfFile,
                            None,
                            "",
                            self.current_location(0),
                        )));
                    },
                    Some(chr) => {
                        self.start_offset = self.current_offset - chr.len_utf8();
                        match chr {
                            ',' => {
                                return Some(Ok(self.nonliteral_token(TokenKind::Comma)));
                            },
                            '(' => {
                                return Some(Ok(self.nonliteral_token(TokenKind::LeftParen)));
                            },
                            ')' => {
                                return Some(Ok(self.nonliteral_token(TokenKind::RightParen)));
                            },
                            '{' => {
                                return Some(Ok(self.nonliteral_token(TokenKind::LeftBrace)));
                            },
                            '}' => {
                                return Some(Ok(self.nonliteral_token(TokenKind::RightBrace)));
                            },
                            '.' => {
                                return Some(Ok(self.nonliteral_token(TokenKind::Dot)));
                            },
                            ';' => {
                                return Some(Ok(self.nonliteral_token(TokenKind::Semicolon)));
                            },
                            '-' => {
                                return Some(Ok(self.nonliteral_token(TokenKind::Minus)));
                            },
                            '+' => {
                                return Some(Ok(self.nonliteral_token(TokenKind::Plus)));
                            },
                            '*' => {
                                return Some(Ok(self.nonliteral_token(TokenKind::Star)));
                            },
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
                                self.state = LexingState::StringStarted;
                            },
                            _ if chr.is_ascii_digit() => {
                                self.state = LexingState::NumberStarted;
                            },
                            _ if chr.is_ascii_alphabetic() || chr == '_' => {
                                self.state = LexingState::Identifier;
                            },
                            _ if chr.is_whitespace() => {
                                // ignore whitespace
                            },
                            _ => {
                                return Some(Err(LexingError {
                                    code: LexingErrorCode::UnexpectedCharacter(chr),
                                    location: self.start_location(),
                                }));
                            },
                        }
                    },
                },
                LexingState::MaybeBangEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(self.nonliteral_token(TokenKind::Bang)));
                    },
                    Some('=') => {
                        self.state = LexingState::Initial;
                        return Some(Ok(self.nonliteral_token(TokenKind::BangEqual)));
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        return Some(Ok(self.nonliteral_token(TokenKind::Bang)));
                    },
                },
                LexingState::MaybeEqualEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(self.nonliteral_token(TokenKind::Equal)));
                    },
                    Some('=') => {
                        self.state = LexingState::Initial;
                        return Some(Ok(self.nonliteral_token(TokenKind::EqualEqual)));
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        return Some(Ok(self.nonliteral_token(TokenKind::Equal)));
                    },
                },
                LexingState::MaybeGreaterEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(self.nonliteral_token(TokenKind::Greater)));
                    },
                    Some('=') => {
                        self.state = LexingState::Initial;
                        return Some(Ok(self.nonliteral_token(TokenKind::GreaterEqual)));
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        return Some(Ok(self.nonliteral_token(TokenKind::Greater)));
                    },
                },
                LexingState::MaybeLessEqual => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(self.nonliteral_token(TokenKind::Less)));
                    },
                    Some('=') => {
                        self.state = LexingState::Initial;
                        return Some(Ok(self.nonliteral_token(TokenKind::LessEqual)));
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        return Some(Ok(self.nonliteral_token(TokenKind::Less)));
                    },
                },
                LexingState::MaybeLineComment => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(self.nonliteral_token(TokenKind::Slash)));
                    },
                    Some('/') => {
                        self.state = LexingState::LineComment;
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        return Some(Ok(self.nonliteral_token(TokenKind::Slash)));
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
                        return Some(Err(LexingError {
                            code: LexingErrorCode::UnterminatedStringLiteral(
                                self.current_lexeme().into(),
                            ),
                            location: self.start_location(),
                        }));
                    },
                    Some('"') => {
                        self.state = LexingState::Initial;
                        return Some(self.unescape_string_token());
                    },
                    Some(_chr) => {},
                },
                LexingState::NumberStarted => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(self.parse_number_token());
                    },
                    Some('.') => {
                        self.state = LexingState::MaybeDecimalPoint;
                    },
                    Some(chr) if chr.is_ascii_digit() => {},
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        return Some(self.parse_number_token());
                    },
                },
                LexingState::MaybeDecimalPoint => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(self.parse_number_token());
                    },
                    Some(chr) if chr.is_ascii_digit() => {
                        self.state = LexingState::FractionDigitsStarted;
                    },
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        if chr.is_ascii_alphabetic() || chr == '_' {
                            self.revert_char('.');
                        }
                        self.revert_char(chr);
                        return Some(self.parse_number_token());
                    },
                },
                LexingState::FractionDigitsStarted => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(self.parse_number_token());
                    },
                    Some(chr) if chr.is_ascii_digit() => {},
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        return Some(self.parse_number_token());
                    },
                },
                LexingState::Identifier => match next_chr {
                    None => {
                        self.state = LexingState::Initial;
                        return Some(Ok(keyword_or_identifier(
                            self.current_lexeme(),
                            self.start_location(),
                        )));
                    },
                    Some(chr) if chr.is_ascii_alphanumeric() || chr == '_' => {},
                    Some(chr) => {
                        self.state = LexingState::Initial;
                        self.revert_char(chr);
                        return Some(Ok(keyword_or_identifier(
                            self.current_lexeme(),
                            self.start_location(),
                        )));
                    },
                },
                LexingState::EndOfFile => match next_chr {
                    None => return None,
                    Some(chr) => {
                        // this should be unreachable, but just in case
                        return Some(Err(LexingError {
                            code: LexingErrorCode::CharacterAfterEndOfFile(chr),
                            location: self.start_location(),
                        }));
                    },
                },
            }
        }
    }
}

fn keyword_or_identifier(lexeme: &str, location: SourceSpan) -> Token<'_> {
    let token_kind = match lexeme {
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
