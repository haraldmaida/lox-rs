use crate::expr::{Binary, Expr, Grouping, Literal, Unary};
use crate::token;
use crate::token::{Token, TokenKind};
use crate::tokenize::{LexingError, LexingErrorCode};
use miette::{Diagnostic, SourceSpan};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxErrorCode {
    CharacterAfterEndOfFile(char),
    ExpectedExpression(String),
    InvalidNumberLiteral(String),
    IoError(String),
    MissingToken(TokenKind),
    UnexpectedEndOfInput,
    UnexpectedCharacter(char),
    UnexpectedToken(TokenKind),
    UnterminatedStringLiteral(String),
}

impl Display for SyntaxErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::CharacterAfterEndOfFile(chr) => {
                write!(f, "character '{chr}' after end of file")
            },
            Self::ExpectedExpression(lexeme) => {
                write!(f, "expected expression at {lexeme}")
            },
            Self::InvalidNumberLiteral(value) => {
                write!(f, "invalid number literal at {value}")
            },
            Self::IoError(message) => {
                write!(f, "{message}")
            },
            Self::MissingToken(kind) => {
                write!(f, "missing {kind:#}")
            },
            Self::UnexpectedEndOfInput => {
                write!(f, "unexpected end of input")
            },
            Self::UnexpectedCharacter(chr) => {
                write!(f, "unexpected character '{chr}'")
            },
            Self::UnexpectedToken(token) => {
                write!(f, "unexpected token '{token:#}'")
            },
            Self::UnterminatedStringLiteral(lexeme) => {
                write!(f, "unterminated string literal {lexeme}")
            },
        }
    }
}

impl From<LexingErrorCode> for SyntaxErrorCode {
    fn from(value: LexingErrorCode) -> Self {
        match value {
            LexingErrorCode::IoError(cause) => Self::IoError(cause),
            LexingErrorCode::CharacterAfterEndOfFile(chr) => Self::CharacterAfterEndOfFile(chr),
            LexingErrorCode::InvalidNumberLiteral(lexeme) => Self::InvalidNumberLiteral(lexeme),
            LexingErrorCode::UnexpectedCharacter(chr) => Self::UnexpectedCharacter(chr),
            LexingErrorCode::UnterminatedStringLiteral(lexeme) => {
                Self::UnterminatedStringLiteral(lexeme)
            },
        }
    }
}

#[derive(thiserror::Error, Diagnostic, Debug, Clone, PartialEq, Eq)]
#[error("{code}")]
pub struct SyntaxError {
    code: SyntaxErrorCode,
    #[help]
    help: Option<String>,
    #[label]
    location: SourceSpan,
}

impl SyntaxError {
    pub const fn code(&self) -> &SyntaxErrorCode {
        &self.code
    }

    pub fn help(&self) -> Option<&str> {
        self.help.as_deref()
    }

    pub const fn location(&self) -> SourceSpan {
        self.location
    }
}

impl From<LexingError> for SyntaxError {
    fn from(
        LexingError {
            code,
            help,
            location,
        }: LexingError,
    ) -> Self {
        Self {
            code: code.into(),
            help,
            location,
        }
    }
}

pub trait Parse<'a> {
    fn parse(self) -> Result<Expr<'a>, SyntaxError>;
}

impl<'a, T> Parse<'a> for T
where
    T: 'a + IntoIterator<Item = Result<Token<'a>, LexingError>>,
{
    fn parse(self) -> Result<Expr<'a>, SyntaxError> {
        Parser::from(self).expression()
    }
}

struct Parser<'a, T>
where
    T: Iterator<Item = Result<Token<'a>, LexingError>>,
{
    tokens: T,
    peeked: Option<T::Item>,
    last_location: SourceSpan,
}

impl<'a, I, T> From<I> for Parser<'a, T>
where
    I: IntoIterator<IntoIter = T>,
    T: Iterator<Item = Result<Token<'a>, LexingError>>,
{
    fn from(tokens: I) -> Self {
        Self {
            tokens: tokens.into_iter(),
            peeked: None,
            last_location: (0, 0).into(),
        }
    }
}

impl<'a, T> Parser<'a, T>
where
    T: 'a + Iterator<Item = Result<Token<'a>, LexingError>>,
{
    const fn error(&self, code: SyntaxErrorCode) -> SyntaxError {
        SyntaxError {
            code,
            help: None,
            location: self.last_location,
        }
    }

    fn advance(&mut self) -> Result<Option<Token<'a>>, SyntaxError> {
        self.peeked
            .take()
            .or_else(|| self.tokens.next())
            .transpose()
            .map_err(SyntaxError::from)
            .inspect(|tk| tk.iter().for_each(|tk| self.last_location = tk.location))
    }

    fn revert(&mut self, token: Token<'a>) {
        self.peeked = Some(Ok(token));
    }

    fn consume(&mut self, token_kind: TokenKind) -> Result<Token<'_>, SyntaxError> {
        match self.advance()? {
            Some(token) if token.kind == token_kind => Ok(token),
            Some(token) => Err(SyntaxError {
                code: SyntaxErrorCode::UnexpectedToken(token.kind),
                help: None,
                location: token.location,
            }),
            None => Err(SyntaxError {
                code: SyntaxErrorCode::MissingToken(token_kind),
                help: None,
                location: self.last_location,
            }),
        }
    }

    fn synchronize(&mut self) -> Result<(), SyntaxError> {
        while let Some(token) = self.advance()? {
            match token.kind {
                TokenKind::Semicolon => return Ok(()),
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => {
                    self.revert(token);
                    return Ok(());
                },
                _ => {
                    // consume and ignore
                },
            }
        }
        Ok(())
    }

    pub fn expression(&mut self) -> Result<Expr<'a>, SyntaxError> {
        self.equality()
    }

    pub fn equality(&mut self) -> Result<Expr<'a>, SyntaxError> {
        let mut expr = self.comparison()?;
        while let Some(token) = self.advance()? {
            match token.kind {
                TokenKind::BangEqual | TokenKind::EqualEqual => {
                    let operator = token;
                    let right = self.comparison()?;
                    expr = Binary::new(expr, operator, right).into();
                },
                _ => {
                    self.revert(token);
                    break;
                },
            }
        }
        Ok(expr)
    }

    pub fn comparison(&mut self) -> Result<Expr<'a>, SyntaxError> {
        let mut expr = self.term()?;
        while let Some(token) = self.advance()? {
            match token.kind {
                TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual => {
                    let operator = token;
                    let right = self.term()?;
                    expr = Binary::new(expr, operator, right).into();
                },
                _ => {
                    self.revert(token);
                    break;
                },
            }
        }
        Ok(expr)
    }

    pub fn term(&mut self) -> Result<Expr<'a>, SyntaxError> {
        let mut expr = self.factor()?;
        while let Some(token) = self.advance()? {
            match token.kind {
                TokenKind::Minus | TokenKind::Plus => {
                    let operator = token;
                    let right = self.factor()?;
                    expr = Binary::new(expr, operator, right).into();
                },
                _ => {
                    self.revert(token);
                    break;
                },
            }
        }
        Ok(expr)
    }

    pub fn factor(&mut self) -> Result<Expr<'a>, SyntaxError> {
        let mut expr = self.unary()?;
        while let Some(token) = self.advance()? {
            match token.kind {
                TokenKind::Slash | TokenKind::Star => {
                    let operator = token;
                    let right = self.unary()?;
                    expr = Binary::new(expr, operator, right).into();
                },
                _ => {
                    self.revert(token);
                    break;
                },
            }
        }
        Ok(expr)
    }

    pub fn unary(&mut self) -> Result<Expr<'a>, SyntaxError> {
        match self.advance()? {
            Some(token) => match token.kind {
                TokenKind::Bang | TokenKind::Minus => {
                    let operator = token;
                    let right = self.unary()?;
                    Ok(Unary::new(operator, right).into())
                },
                _ => {
                    self.revert(token);
                    self.primary()
                },
            },
            None => Err(self.error(SyntaxErrorCode::UnexpectedEndOfInput)),
        }
    }

    pub fn primary(&mut self) -> Result<Expr<'a>, SyntaxError> {
        match self.advance()? {
            Some(token) => match token.kind {
                TokenKind::Nil => Ok(Literal::Nil.into()),
                TokenKind::False => Ok(Literal::Bool(false).into()),
                TokenKind::True => Ok(Literal::Bool(true).into()),
                TokenKind::NumberLiteral => {
                    if let Some(token::Literal::Number(value)) = token.literal {
                        Ok(Literal::Number(value).into())
                    } else {
                        unreachable!("invalid number token {token:?}! please file a bug report.")
                    }
                },
                TokenKind::StringLiteral => {
                    if let Some(token::Literal::String(value)) = token.literal {
                        Ok(Literal::String(value.to_string()).into())
                    } else {
                        unreachable!("invalid string token {token:?}! please file a bug report.")
                    }
                },
                TokenKind::LeftParen => {
                    let expr = self.expression()?;
                    self.consume(TokenKind::RightParen)?;
                    Ok(Grouping::new(expr).into())
                },
                _ => {
                    let lexeme = token.lexeme;
                    self.revert(token);
                    Err(self.error(SyntaxErrorCode::ExpectedExpression(lexeme.into())))
                },
            },
            None => Err(self.error(SyntaxErrorCode::UnexpectedEndOfInput)),
        }
    }
}

#[cfg(test)]
mod tests;
