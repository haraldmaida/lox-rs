use crate::expr::{Binary, Expr, Grouping, Literal, Unary, Variable};
use crate::program::Program;
use crate::stmt::{Expression, Print, Stmt, Var};
use crate::token;
use crate::token::{Token, TokenKind};
use crate::tokenize::{LexingError, LexingErrorCode};
use miette::{Diagnostic, SourceSpan};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxErrorCode {
    CharacterAfterEndOfFile(char),
    InvalidExpression(String),
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
            Self::InvalidExpression(lexeme) => {
                write!(f, "not a valid expression at {lexeme}")
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
#[diagnostic()]
pub struct SyntaxError {
    code: SyntaxErrorCode,
    #[label]
    location: SourceSpan,
}

impl SyntaxError {
    pub const fn code(&self) -> &SyntaxErrorCode {
        &self.code
    }

    pub const fn location(&self) -> SourceSpan {
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

pub trait Parse<'a> {
    fn parse(self) -> Result<Program<'a>, Vec<SyntaxError>>;

    fn parse_expr(self) -> Result<Expr<'a>, SyntaxError>;
}

impl<'a, T> Parse<'a> for T
where
    T: 'a + IntoIterator<Item = Result<Token<'a>, LexingError>>,
{
    fn parse(self) -> Result<Program<'a>, Vec<SyntaxError>> {
        Parser::from(self).program()
    }

    fn parse_expr(self) -> Result<Expr<'a>, SyntaxError> {
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

    fn consume(&mut self, token_kind: TokenKind) -> Result<Token<'a>, SyntaxError> {
        match self.advance()? {
            Some(token) if token.kind == token_kind => Ok(token),
            Some(token) => {
                self.revert(token);
                Err(SyntaxError {
                    code: SyntaxErrorCode::MissingToken(token_kind),
                    location: token.location,
                })
            },
            None => Err(SyntaxError {
                code: SyntaxErrorCode::MissingToken(token_kind),
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

    pub fn program(&mut self) -> Result<Program<'a>, Vec<SyntaxError>> {
        let mut errors = Vec::new();
        let mut statements = Vec::new();
        loop {
            match self.declaration() {
                None => break,
                Some(Ok(stmt)) => statements.push(stmt),
                Some(Err(err)) => {
                    errors.push(err);
                    if let Err(error) = self.synchronize() {
                        errors.push(error);
                        break;
                    }
                },
            }
        }
        if errors.is_empty() {
            Ok(Program::new(statements))
        } else {
            Err(errors)
        }
    }

    pub fn declaration(&mut self) -> Option<Result<Stmt<'a>, SyntaxError>> {
        match self.advance() {
            Ok(None) => None,
            Ok(Some(token)) => match token.kind {
                TokenKind::EndOfFile => None,
                TokenKind::Var => Some(self.var_declaration()),
                _ => {
                    self.revert(token);
                    self.statement()
                },
            },
            Err(err) => Some(Err(err)),
        }
    }

    pub fn var_declaration(&mut self) -> Result<Stmt<'a>, SyntaxError> {
        let name = self.consume(TokenKind::Identifier)?;
        let initializer = match self.advance()? {
            None => None,
            Some(token) => match token.kind {
                TokenKind::Equal => Some(self.expression()?),
                _ => {
                    self.revert(token);
                    None
                },
            },
        };
        self.consume(TokenKind::Semicolon)?;
        Ok(Var::new(name, initializer).into())
    }

    pub fn statement(&mut self) -> Option<Result<Stmt<'a>, SyntaxError>> {
        match self.advance() {
            Ok(None) => None,
            Ok(Some(token)) => match token.kind {
                TokenKind::Print => Some(self.print_statement()),
                _ => {
                    self.revert(token);
                    Some(self.expression_statement())
                },
            },
            Err(err) => Some(Err(err)),
        }
    }

    pub fn print_statement(&mut self) -> Result<Stmt<'a>, SyntaxError> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon)?;
        Ok(Print::new(expr).into())
    }

    pub fn expression_statement(&mut self) -> Result<Stmt<'a>, SyntaxError> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon)?;
        Ok(Expression::new(expr).into())
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
                        Ok(Literal::String(value).into())
                    } else {
                        unreachable!("invalid string token {token:?}! please file a bug report.")
                    }
                },
                TokenKind::Identifier => Ok(Variable::new(token).into()),
                TokenKind::LeftParen => {
                    let expr = self.expression()?;
                    self.consume(TokenKind::RightParen)?;
                    Ok(Grouping::new(expr).into())
                },
                _ => {
                    let lexeme = token.lexeme;
                    self.revert(token);
                    Err(self.error(SyntaxErrorCode::InvalidExpression(lexeme.into())))
                },
            },
            None => Err(self.error(SyntaxErrorCode::UnexpectedEndOfInput)),
        }
    }
}

#[cfg(test)]
mod tests;
