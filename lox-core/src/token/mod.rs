use crate::source::Location;
use std::fmt;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    EndOfFile,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Minus,
    Plus,
    Star,
    Slash,
    Bang,
    Equal,
    Greater,
    Less,
    BangEqual,
    EqualEqual,
    GreaterEqual,
    LessEqual,
    StringLiteral,
    NumberLiteral,
    Identifier,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted = match self {
            Self::EndOfFile => "EOF",
            Self::LeftParen => "LEFT_PAREN",
            Self::RightParen => "RIGHT_PAREN",
            Self::LeftBrace => "LEFT_BRACE",
            Self::RightBrace => "RIGHT_BRACE",
            Self::Comma => "COMMA",
            Self::Dot => "DOT",
            Self::Semicolon => "SEMICOLON",
            Self::Minus => "MINUS",
            Self::Plus => "PLUS",
            Self::Star => "STAR",
            Self::Slash => "SLASH",
            Self::Bang => "BANG",
            Self::Equal => "EQUAL",
            Self::Greater => "GREATER",
            Self::Less => "LESS",
            Self::BangEqual => "BANG_EQUAL",
            Self::EqualEqual => "EQUAL_EQUAL",
            Self::GreaterEqual => "GREATER_EQUAL",
            Self::LessEqual => "LESS_EQUAL",
            Self::StringLiteral => "STRING_LITERAL",
            Self::NumberLiteral => "NUMBER_LITERAL",
            Self::Identifier => "IDENTIFIER",
            Self::And => "AND",
            Self::Class => "CLASS",
            Self::Else => "ELSE",
            Self::False => "FALSE",
            Self::Fun => "FUN",
            Self::For => "FOR",
            Self::If => "IF",
            Self::Nil => "NIL",
            Self::Or => "OR",
            Self::Print => "PRINT",
            Self::Return => "RETURN",
            Self::Super => "SUPER",
            Self::This => "THIS",
            Self::True => "TRUE",
            Self::Var => "VAR",
            Self::While => "WHILE",
        };
        f.write_str(formatted)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => {
                if value % 1.0 == 0.0 {
                    write!(f, "{value:.1}")
                } else {
                    write!(f, "{value}")
                }
            },
            Self::String(value) => write!(f, "{value}"),
        }
    }
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    literal: Option<Literal>,
    lexeme: String,
    location: Location,
}

impl Token {
    pub const fn new(
        kind: TokenKind,
        literal: Option<Literal>,
        lexeme: String,
        location: Location,
    ) -> Self {
        Self {
            kind,
            literal,
            lexeme,
            location,
        }
    }

    pub fn new_nonliteral(
        kind: TokenKind,
        lexeme: impl Into<String>,
        location: impl Into<Location>,
    ) -> Self {
        Self {
            kind,
            literal: None,
            lexeme: lexeme.into(),
            location: location.into(),
        }
    }

    pub fn new_literal(
        literal: impl Into<Literal>,
        lexeme: impl Into<String>,
        location: impl Into<Location>,
    ) -> Self {
        let literal = literal.into();
        let kind = match literal {
            Literal::Number(_) => TokenKind::NumberLiteral,
            Literal::String(_) => TokenKind::StringLiteral,
        };
        Self {
            kind,
            literal: Some(literal),
            lexeme: lexeme.into(),
            location: location.into(),
        }
    }

    pub fn new_identifier(lexeme: impl Into<String>, location: impl Into<Location>) -> Self {
        Self {
            kind: TokenKind::Identifier,
            literal: None,
            lexeme: lexeme.into(),
            location: location.into(),
        }
    }

    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    pub const fn literal(&self) -> Option<&Literal> {
        self.literal.as_ref()
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub const fn location(&self) -> Location {
        self.location
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal = self
            .literal
            .as_ref()
            .map_or_else(|| "null".to_string(), ToString::to_string);
        write!(f, "{} {} {literal}", self.kind, self.lexeme)
    }
}

#[cfg(test)]
mod tests;
