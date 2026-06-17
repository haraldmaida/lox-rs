use crate::data::Symbol;
use miette::SourceSpan;
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
        let formatted = if f.alternate() {
            match self {
                Self::EndOfFile => "EOF",
                Self::LeftParen => "'('",
                Self::RightParen => "')'",
                Self::LeftBrace => "'{'",
                Self::RightBrace => "'}'",
                Self::Comma => "','",
                Self::Dot => "'.'",
                Self::Semicolon => "';'",
                Self::Minus => "'-'",
                Self::Plus => "'+'",
                Self::Star => "'*'",
                Self::Slash => "'/'",
                Self::Bang => "'!'",
                Self::Equal => "'='",
                Self::Greater => "'>'",
                Self::Less => "'<'",
                Self::BangEqual => "'!=",
                Self::EqualEqual => "'=='",
                Self::GreaterEqual => "'>='",
                Self::LessEqual => "'<='",
                Self::StringLiteral => "string literal",
                Self::NumberLiteral => "number literal",
                Self::Identifier => "identifier",
                Self::And => "and",
                Self::Class => "class",
                Self::Else => "else",
                Self::False => "false",
                Self::Fun => "fun",
                Self::For => "for",
                Self::If => "if",
                Self::Nil => "nil",
                Self::Or => "or",
                Self::Print => "print",
                Self::Return => "return",
                Self::Super => "super",
                Self::This => "this",
                Self::True => "true",
                Self::Var => "var",
                Self::While => "while",
            }
        } else {
            match self {
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
            }
        };
        f.write_str(formatted)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal {
    Number(f64),
    String(Symbol),
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => {
                if (value.trunc() - *value).abs() < 5. * f64::EPSILON {
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

impl<'a> From<&'a str> for Literal {
    fn from(value: &'a str) -> Self {
        Self::String(Symbol::intern(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub literal: Option<Literal>,
    pub lexeme: &'a str,
    pub location: SourceSpan,
}

impl<'a> Token<'a> {
    pub const fn new(
        kind: TokenKind,
        literal: Option<Literal>,
        lexeme: &'a str,
        location: SourceSpan,
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
        lexeme: &'a str,
        location: impl Into<SourceSpan>,
    ) -> Self {
        Self {
            kind,
            literal: None,
            lexeme,
            location: location.into(),
        }
    }

    pub fn new_literal(
        literal: impl Into<Literal>,
        lexeme: &'a str,
        location: impl Into<SourceSpan>,
    ) -> Self {
        let literal = literal.into();
        let kind = match literal {
            Literal::Number(_) => TokenKind::NumberLiteral,
            Literal::String(_) => TokenKind::StringLiteral,
        };
        Self {
            kind,
            literal: Some(literal),
            lexeme,
            location: location.into(),
        }
    }

    pub fn new_identifier(lexeme: &'a str, location: impl Into<SourceSpan>) -> Self {
        Self {
            kind: TokenKind::Identifier,
            literal: None,
            lexeme,
            location: location.into(),
        }
    }

    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    pub const fn literal(&self) -> Option<&Literal> {
        self.literal.as_ref()
    }

    pub const fn lexeme(&self) -> &str {
        self.lexeme
    }

    pub const fn location(&self) -> SourceSpan {
        self.location
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str(self.lexeme)
        } else {
            let literal = self
                .literal
                .as_ref()
                .map_or_else(|| "null".to_string(), ToString::to_string);
            write!(f, "{} {} {literal}", self.kind, self.lexeme)
        }
    }
}

pub fn token(kind: TokenKind, lexeme: &str, location: impl Into<SourceSpan>) -> Token<'_> {
    Token::new(kind, None, lexeme, location.into())
}

#[cfg(test)]
mod tests;
