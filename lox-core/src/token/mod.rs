#[cfg(any(test, feature = "dsl"))]
pub use dsl::*;

use crate::data::Symbol;
use miette::SourceSpan;
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
                Self::StringLiteral => "STRING",
                Self::NumberLiteral => "NUMBER",
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
        } else {
            match self {
                Self::EndOfFile => "EOF",
                Self::LeftParen => "(",
                Self::RightParen => ")",
                Self::LeftBrace => "{",
                Self::RightBrace => "}",
                Self::Comma => ",",
                Self::Dot => ".",
                Self::Semicolon => ";",
                Self::Minus => "-",
                Self::Plus => "+",
                Self::Star => "*",
                Self::Slash => "/",
                Self::Bang => "!",
                Self::Equal => "=",
                Self::Greater => ">",
                Self::Less => "<",
                Self::BangEqual => "!=",
                Self::EqualEqual => "==",
                Self::GreaterEqual => ">=",
                Self::LessEqual => "<=",
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

impl From<Symbol> for Literal {
    fn from(value: Symbol) -> Self {
        Self::String(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: Option<Literal>,
    pub lexeme: Symbol,
    pub location: SourceSpan,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.lexeme == other.lexeme && self.location == other.location && self.kind == other.kind
    }
}

impl Eq for Token {}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lexeme.hash(state);
        self.location.hash(state);
        self.kind.hash(state);
    }
}

impl Token {
    pub const fn new(
        kind: TokenKind,
        literal: Option<Literal>,
        lexeme: Symbol,
        location: SourceSpan,
    ) -> Self {
        Self {
            kind,
            literal,
            lexeme,
            location,
        }
    }

    /// Escapes a string literal in Lox.
    ///
    /// Lox has no escaping, so it just adds `"` around the string.
    pub fn escape(value: &str) -> String {
        format!("\"{value}\"")
    }

    /// Unescapes the lexeme for a string literal in Lox.
    ///
    /// Lox has no escaping, so it just removes the `"` around the string.
    pub fn unescape(lexeme: &str) -> &str {
        lexeme
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .unwrap_or(lexeme)
    }

    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    pub const fn literal(&self) -> Option<&Literal> {
        self.literal.as_ref()
    }

    pub const fn lexeme(&self) -> Symbol {
        self.lexeme
    }

    pub const fn location(&self) -> SourceSpan {
        self.location
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            let literal = self
                .literal
                .as_ref()
                .map_or_else(|| "null".to_string(), ToString::to_string);
            write!(f, "{:#} {} {literal}", self.kind, self.lexeme)
        } else {
            f.write_str(self.lexeme.as_str())
        }
    }
}

#[cfg(any(test, feature = "dsl"))]
mod dsl;

#[cfg(test)]
mod tests;
