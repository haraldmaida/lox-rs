use super::{Literal, Token, TokenKind};
use crate::data::Symbol;
use miette::SourceSpan;

pub fn token(kind: TokenKind, lexeme: impl Into<Symbol>, location: impl Into<SourceSpan>) -> Token {
    Token::new(kind, None, lexeme.into(), location.into())
}

pub fn literal_token(
    value: impl Into<Literal>,
    lexeme: &str,
    location: impl Into<SourceSpan>,
) -> Token {
    let value = value.into();
    let kind = match value {
        Literal::Number(_) => TokenKind::NumberLiteral,
        Literal::String(_) => TokenKind::StringLiteral,
    };
    Token::new(kind, Some(value), lexeme.into(), location.into())
}

pub fn eof(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::EndOfFile, None, "".into(), location.into())
}

pub fn left_paren(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::LeftParen, None, "(".into(), location.into())
}

pub fn right_paren(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::RightParen, None, ")".into(), location.into())
}

pub fn left_brace(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::LeftBrace, None, "{".into(), location.into())
}

pub fn right_brace(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::RightBrace, None, "}".into(), location.into())
}

pub fn comma(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Comma, None, ".into(),".into(), location.into())
}

pub fn dot(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Dot, None, ".".into(), location.into())
}

pub fn semicolon(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Semicolon, None, ";".into(), location.into())
}

pub fn minus(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Minus, None, "-".into(), location.into())
}

pub fn plus(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Plus, None, "+".into(), location.into())
}

pub fn star(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Star, None, "*".into(), location.into())
}

pub fn slash(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Slash, None, "/".into(), location.into())
}

pub fn bang(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Bang, None, "!".into(), location.into())
}

pub fn equal(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Equal, None, "=".into(), location.into())
}

pub fn greater(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Greater, None, ">".into(), location.into())
}

pub fn less(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Less, None, "<".into(), location.into())
}

pub fn bang_equal(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::BangEqual, None, "!=".into(), location.into())
}

pub fn equal_equal(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::EqualEqual, None, "==".into(), location.into())
}

pub fn greater_equal(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::GreaterEqual, None, ">=".into(), location.into())
}

pub fn less_equal(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::LessEqual, None, "<=".into(), location.into())
}

pub fn string_literal(lexeme: &str, location: impl Into<SourceSpan>) -> Token {
    let value = Token::unescape(lexeme);
    let symbol = Symbol::from(value);
    Token::new(
        TokenKind::StringLiteral,
        Some(Literal::String(symbol)),
        lexeme.into(),
        location.into(),
    )
}

pub fn number_literal(lexeme: &str, location: impl Into<SourceSpan>) -> Token {
    let value = lexeme
        .parse::<f64>()
        .expect("number literal must be a valid floating point number");
    Token::new(
        TokenKind::NumberLiteral,
        Some(Literal::Number(value)),
        lexeme.into(),
        location.into(),
    )
}

pub fn identifier(lexeme: &str, location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Identifier, None, lexeme.into(), location.into())
}

pub fn and(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::And, None, "and".into(), location.into())
}

pub fn class(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Class, None, "class".into(), location.into())
}

pub fn else_(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Else, None, "else".into(), location.into())
}

pub fn false_(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::False, None, "false".into(), location.into())
}

pub fn fun(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Fun, None, "fun".into(), location.into())
}

pub fn for_(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::For, None, "for".into(), location.into())
}

pub fn if_(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::If, None, "if".into(), location.into())
}

pub fn keyword(kind: TokenKind, lexeme: &str, location: impl Into<SourceSpan>) -> Token {
    Token::new(kind, None, lexeme.into(), location.into())
}

pub fn nil(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Nil, None, "nil".into(), location.into())
}

pub fn or(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Or, None, "or".into(), location.into())
}

pub fn print(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Print, None, "print".into(), location.into())
}

pub fn return_(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Return, None, "return".into(), location.into())
}

pub fn super_(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Super, None, "super".into(), location.into())
}

pub fn this(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::This, None, "this".into(), location.into())
}

pub fn true_(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::True, None, "true".into(), location.into())
}

pub fn var(lexeme: &str, location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::Var, None, lexeme.into(), location.into())
}

pub fn while_(location: impl Into<SourceSpan>) -> Token {
    Token::new(TokenKind::While, None, "while".into(), location.into())
}
