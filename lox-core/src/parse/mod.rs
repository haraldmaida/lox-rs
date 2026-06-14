use crate::expr::{Binary, Expr, Grouping, Literal, Unary};
use crate::source::Location;
use crate::token;
use crate::token::{Token, TokenKind};
use crate::tokenize::{LexingError, LexingErrorCode};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxErrorCode {
    CharacterAfterEndOfFile(char),
    ExpectedExpression(String),
    InvalidNumberLiteral(String),
    IoError(String),
    MissingToken(TokenKind),
    UnexpectedEndOfInput,
    UnexpectedCharacter(char),
    UnexpectedToken(Token),
    UnterminatedStringLiteral(String),
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

#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxError {
    code: SyntaxErrorCode,
    location: Location,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let location = self.location;
        match &self.code {
            SyntaxErrorCode::CharacterAfterEndOfFile(chr) => {
                write!(f, "{location}: character '{chr}' after end of file")
            },
            SyntaxErrorCode::ExpectedExpression(lexeme) => {
                write!(f, "{location}: expected expression at {lexeme}")
            },
            SyntaxErrorCode::InvalidNumberLiteral(value) => {
                write!(f, "{location}: invalid number literal at {value}")
            },
            SyntaxErrorCode::IoError(message) => {
                write!(f, "{location}: {message}")
            },
            SyntaxErrorCode::MissingToken(kind) => {
                write!(f, "{location}: missing {kind:#}")
            },
            SyntaxErrorCode::UnexpectedEndOfInput => {
                write!(f, "{location}: unexpected end of input")
            },
            SyntaxErrorCode::UnexpectedCharacter(chr) => {
                write!(f, "{location}: unexpected character '{chr}'")
            },
            SyntaxErrorCode::UnexpectedToken(token) => {
                write!(f, "{location}: unexpected token '{token:#}'")
            },
            SyntaxErrorCode::UnterminatedStringLiteral(lexeme) => {
                write!(f, "{location}: unterminated string literal {lexeme}")
            },
        }
    }
}

impl SyntaxError {
    pub const fn code(&self) -> &SyntaxErrorCode {
        &self.code
    }

    pub const fn location(&self) -> Location {
        self.location
    }
}

impl From<LexingError> for SyntaxError {
    fn from(LexingError { code, location }: LexingError) -> Self {
        Self {
            code: code.into(),
            location,
        }
    }
}

pub trait Parse {
    fn parse(self) -> Result<Expr, SyntaxError>;
}

impl<T> Parse for T
where
    T: IntoIterator<Item = Result<Token, LexingError>>,
{
    fn parse(self) -> Result<Expr, SyntaxError> {
        Parser::from(self).expression()
    }
}

struct Parser<T>
where
    T: Iterator<Item = Result<Token, LexingError>>,
{
    tokens: T,
    peeked: Option<T::Item>,
    last_location: Location,
}

impl<I, T> From<I> for Parser<T>
where
    I: IntoIterator<IntoIter = T>,
    T: Iterator<Item = Result<Token, LexingError>>,
{
    fn from(tokens: I) -> Self {
        Self {
            tokens: tokens.into_iter(),
            peeked: None,
            last_location: Location::default(),
        }
    }
}

impl<T> Parser<T>
where
    T: Iterator<Item = Result<Token, LexingError>>,
{
    const fn error(&self, code: SyntaxErrorCode) -> SyntaxError {
        SyntaxError {
            code,
            location: self.last_location,
        }
    }

    fn advance(&mut self) -> Result<Option<Token>, SyntaxError> {
        self.peeked
            .take()
            .or_else(|| self.tokens.next())
            .transpose()
            .map_err(SyntaxError::from)
            .inspect(|tk| tk.iter().for_each(|tk| self.last_location = tk.location))
    }

    fn revert(&mut self, token: Token) {
        self.peeked = Some(Ok(token));
    }

    fn consume(&mut self, token_kind: TokenKind) -> Result<Token, SyntaxError> {
        match self.advance()? {
            Some(token) if token.kind == token_kind => Ok(token),
            Some(_token) => Err(self.error(SyntaxErrorCode::MissingToken(token_kind))),
            None => Err(self.error(SyntaxErrorCode::UnexpectedEndOfInput)),
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

    pub fn expression(&mut self) -> Result<Expr, SyntaxError> {
        self.equality()
    }

    pub fn equality(&mut self) -> Result<Expr, SyntaxError> {
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

    pub fn comparison(&mut self) -> Result<Expr, SyntaxError> {
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

    pub fn term(&mut self) -> Result<Expr, SyntaxError> {
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

    pub fn factor(&mut self) -> Result<Expr, SyntaxError> {
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

    pub fn unary(&mut self) -> Result<Expr, SyntaxError> {
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

    pub fn primary(&mut self) -> Result<Expr, SyntaxError> {
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
                        Ok(Literal::String(value).into())
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
                    let lexeme = token.lexeme.clone();
                    self.revert(token);
                    Err(self.error(SyntaxErrorCode::ExpectedExpression(lexeme)))
                },
            },
            None => Err(self.error(SyntaxErrorCode::UnexpectedEndOfInput)),
        }
    }
}

#[cfg(test)]
mod tests;
