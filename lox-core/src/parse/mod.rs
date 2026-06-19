use crate::expr::{Assign, Binary, Call, Expr, Grouping, Literal, Logical, Unary, Variable};
use crate::program::Program;
use crate::stmt::{Block, Expression, If, Print, Stmt, Var, While};
use crate::token;
use crate::token::{Token, TokenKind};
use crate::tokenize::{LexingError, LexingErrorCode};
use miette::{Diagnostic, SourceSpan};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxErrorCode {
    CharacterAfterEndOfFile(char),
    InvalidAssignmentTarget,
    InvalidExpression(String),
    InvalidNumberLiteral(String),
    IoError(String),
    MissingForCondition,
    MissingForIncrement,
    MissingForInitializer,
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
            Self::InvalidAssignmentTarget => {
                write!(f, "invalid assignment target")
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
            Self::MissingForCondition => {
                write!(f, "missing for condition or semicolon")
            },
            Self::MissingForIncrement => {
                write!(f, "missing for increment or right paren")
            },
            Self::MissingForInitializer => {
                write!(f, "missing for initializer or semicolon")
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

    fn program(&mut self) -> Result<Program<'a>, Vec<SyntaxError>> {
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

    fn declaration(&mut self) -> Option<Result<Stmt<'a>, SyntaxError>> {
        match self.advance() {
            Ok(None) => None,
            Ok(Some(token)) => match token.kind {
                TokenKind::EndOfFile => None,
                TokenKind::Var => Some(self.var_declaration()),
                _ => {
                    self.revert(token);
                    Some(self.statement())
                },
            },
            Err(err) => Some(Err(err)),
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt<'a>, SyntaxError> {
        let name = self.consume(TokenKind::Identifier)?;
        let initializer = match self.advance()? {
            None => None,
            Some(token) => {
                if token.kind == TokenKind::Equal {
                    Some(self.expression()?)
                } else {
                    self.revert(token);
                    None
                }
            },
        };
        self.consume(TokenKind::Semicolon)?;
        Ok(Var::new(name, initializer).into())
    }

    fn declaration_inside_block(&mut self) -> Option<Result<Stmt<'a>, SyntaxError>> {
        match self.advance() {
            Ok(None) => None,
            Ok(Some(token)) => match token.kind {
                TokenKind::RightBrace => {
                    self.revert(token);
                    None
                },
                TokenKind::EndOfFile => None,
                TokenKind::Var => Some(self.var_declaration()),
                _ => {
                    self.revert(token);
                    Some(self.statement())
                },
            },
            Err(err) => Some(Err(err)),
        }
    }

    fn statement(&mut self) -> Result<Stmt<'a>, SyntaxError> {
        match self.advance() {
            Ok(None) => Err(self.error(SyntaxErrorCode::UnexpectedEndOfInput)),
            Ok(Some(token)) => match token.kind {
                TokenKind::LeftBrace => self.block(),
                TokenKind::For => self.for_statement(),
                TokenKind::If => self.if_statement(),
                TokenKind::Print => self.print_statement(),
                TokenKind::While => self.while_statement(),
                _ => {
                    self.revert(token);
                    self.expression_statement()
                },
            },
            Err(err) => Err(err),
        }
    }

    fn block(&mut self) -> Result<Stmt<'a>, SyntaxError> {
        let mut statements = Vec::new();
        while let Some(stmt) = self.declaration_inside_block() {
            statements.push(stmt?);
        }
        self.consume(TokenKind::RightBrace)?;
        Ok(Block::new(statements).into())
    }

    fn for_statement(&mut self) -> Result<Stmt<'a>, SyntaxError> {
        self.consume(TokenKind::LeftParen)?;
        let initializer = if let Some(token) = self.advance()? {
            match token.kind {
                TokenKind::Semicolon => None,
                TokenKind::Var => Some(self.var_declaration()?),
                _ => {
                    self.revert(token);
                    Some(self.expression_statement()?)
                },
            }
        } else {
            return Err(self.error(SyntaxErrorCode::MissingForInitializer));
        };
        let condition = if let Some(token) = self.advance()? {
            self.revert(token);
            if token.kind == TokenKind::Semicolon {
                Literal::Bool(true).into()
            } else {
                self.expression()?
            }
        } else {
            return Err(self.error(SyntaxErrorCode::MissingForCondition));
        };
        self.consume(TokenKind::Semicolon)?;
        let increment = if let Some(token) = self.advance()? {
            self.revert(token);
            if token.kind == TokenKind::RightParen {
                None
            } else {
                Some(Stmt::from(self.expression()?))
            }
        } else {
            return Err(self.error(SyntaxErrorCode::MissingForIncrement));
        };
        self.consume(TokenKind::RightParen)?;
        let body = self.statement()?;
        let while_stmt = match (initializer, increment) {
            (None, None) => Stmt::from(While::new(condition, body)),
            (None, Some(incr)) => {
                Stmt::from(While::new(condition, Block::new(vec![body, incr]).into()))
            },
            (Some(init), None) => {
                Stmt::from(Block::new(vec![init, While::new(condition, body).into()]))
            },
            (Some(init), Some(incr)) => Stmt::from(Block::new(vec![
                init,
                While::new(condition, Block::new(vec![body, incr]).into()).into(),
            ])),
        };
        Ok(while_stmt)
    }

    fn if_statement(&mut self) -> Result<Stmt<'a>, SyntaxError> {
        self.consume(TokenKind::LeftParen)?;
        let condition = self.expression()?;
        self.consume(TokenKind::RightParen)?;

        let then_branch = self.statement()?;
        let else_branch = if let Some(token) = self.advance()? {
            if token.kind == TokenKind::Else {
                Some(self.statement()?)
            } else {
                self.revert(token);
                None
            }
        } else {
            None
        };

        Ok(If::new(condition, then_branch, else_branch).into())
    }

    fn print_statement(&mut self) -> Result<Stmt<'a>, SyntaxError> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon)?;
        Ok(Print::new(expr).into())
    }

    fn while_statement(&mut self) -> Result<Stmt<'a>, SyntaxError> {
        self.consume(TokenKind::LeftParen)?;
        let condition = self.expression()?;
        self.consume(TokenKind::RightParen)?;
        let body = self.statement()?;
        Ok(While::new(condition, body).into())
    }

    fn expression_statement(&mut self) -> Result<Stmt<'a>, SyntaxError> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon)?;
        Ok(Expression::new(expr).into())
    }

    fn expression(&mut self) -> Result<Expr<'a>, SyntaxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr<'a>, SyntaxError> {
        let expr = self.logical_or()?;
        if let Some(token) = self.advance()? {
            if token.kind == TokenKind::Equal {
                let value = self.assignment()?;
                if let Expr::Variable(variable) = expr {
                    let name = variable.take_name();
                    Ok(Assign::new(name, value).into())
                } else {
                    self.revert(token);
                    Err(self.error(SyntaxErrorCode::InvalidAssignmentTarget))
                }
            } else {
                self.revert(token);
                Ok(expr)
            }
        } else {
            Ok(expr)
        }
    }

    fn logical_or(&mut self) -> Result<Expr<'a>, SyntaxError> {
        let mut expr = self.logical_and()?;
        while let Some(token) = self.advance()? {
            if token.kind == TokenKind::Or {
                let operator = token;
                let right = self.logical_and()?;
                expr = Logical::new(expr, operator, right).into();
            } else {
                self.revert(token);
                break;
            }
        }
        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expr<'a>, SyntaxError> {
        let mut expr = self.equality()?;
        while let Some(token) = self.advance()? {
            if token.kind == TokenKind::And {
                let operator = token;
                let right = self.equality()?;
                expr = Logical::new(expr, operator, right).into();
            } else {
                self.revert(token);
                break;
            }
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr<'a>, SyntaxError> {
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

    fn comparison(&mut self) -> Result<Expr<'a>, SyntaxError> {
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

    fn term(&mut self) -> Result<Expr<'a>, SyntaxError> {
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

    fn factor(&mut self) -> Result<Expr<'a>, SyntaxError> {
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

    fn unary(&mut self) -> Result<Expr<'a>, SyntaxError> {
        match self.advance()? {
            Some(token) => match token.kind {
                TokenKind::Bang | TokenKind::Minus => {
                    let operator = token;
                    let right = self.unary()?;
                    Ok(Unary::new(operator, right).into())
                },
                _ => {
                    self.revert(token);
                    self.call()
                },
            },
            None => Err(self.error(SyntaxErrorCode::UnexpectedEndOfInput)),
        }
    }

    fn call(&mut self) -> Result<Expr<'a>, SyntaxError> {
        let mut expr = self.primary()?;
        while let Some(token) = self.advance()? {
            if token.kind == TokenKind::LeftParen {
                expr = self.finish_call(expr)?;
            } else {
                self.revert(token);
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr<'a>) -> Result<Expr<'a>, SyntaxError> {
        let mut arguments = Vec::new();
        let paren = if let Some(token) = self.advance()? {
            if token.kind == TokenKind::RightParen {
                token
            } else {
                self.revert(token);
                loop {
                    arguments.push(self.expression()?);
                    if let Some(token) = self.advance()?
                        && token.kind != TokenKind::Comma
                    {
                        self.revert(token);
                        break;
                    }
                }
                self.consume(TokenKind::RightParen)?
            }
        } else {
            return Err(self.error(SyntaxErrorCode::MissingToken(TokenKind::RightParen)));
        };
        Ok(Call::new(callee, paren, arguments).into())
    }

    fn primary(&mut self) -> Result<Expr<'a>, SyntaxError> {
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
