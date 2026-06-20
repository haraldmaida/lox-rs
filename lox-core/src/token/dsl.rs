use super::{Literal, Token, TokenKind};
use crate::data::Symbol;
use miette::SourceSpan;

pub fn token(kind: TokenKind, lexeme: &str, location: impl Into<SourceSpan>) -> Token<'_> {
    Token::new(kind, None, lexeme, location.into())
}

pub fn literal_token(
    value: impl Into<Literal>,
    lexeme: &str,
    location: impl Into<SourceSpan>,
) -> Token<'_> {
    let value = value.into();
    let kind = match value {
        Literal::Number(_) => TokenKind::NumberLiteral,
        Literal::String(_) => TokenKind::StringLiteral,
    };
    Token::new(kind, Some(value), lexeme, location.into())
}

pub fn eof<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::EndOfFile, None, "", location.into())
}

pub fn left_paren<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::LeftParen, None, "(", location.into())
}

pub fn right_paren<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::RightParen, None, ")", location.into())
}

pub fn left_brace<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::LeftBrace, None, "{", location.into())
}

pub fn right_brace<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::RightBrace, None, "}", location.into())
}

pub fn comma<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Comma, None, ",", location.into())
}

pub fn dot<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Dot, None, ".", location.into())
}

pub fn semicolon<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Semicolon, None, ";", location.into())
}

pub fn minus<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Minus, None, "-", location.into())
}

pub fn plus<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Plus, None, "+", location.into())
}

pub fn star<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Star, None, "*", location.into())
}

pub fn slash<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Slash, None, "/", location.into())
}

pub fn bang<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Bang, None, "!", location.into())
}

pub fn equal<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Equal, None, "=", location.into())
}

pub fn greater<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Greater, None, ">", location.into())
}

pub fn less<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Less, None, "<", location.into())
}

pub fn bang_equal<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::BangEqual, None, "!=", location.into())
}

pub fn equal_equal<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::EqualEqual, None, "==", location.into())
}

pub fn greater_equal<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::GreaterEqual, None, ">=", location.into())
}

pub fn less_equal<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::LessEqual, None, "<=", location.into())
}

pub fn string_literal(lexeme: &str, location: impl Into<SourceSpan>) -> Token<'_> {
    let value = Token::unescape(lexeme);
    let symbol = Symbol::from(value);
    Token::new(
        TokenKind::StringLiteral,
        Some(Literal::String(symbol)),
        lexeme,
        location.into(),
    )
}

pub fn number_literal(lexeme: &str, location: impl Into<SourceSpan>) -> Token<'_> {
    let value = lexeme
        .parse::<f64>()
        .expect("number literal must be a valid floating point number");
    Token::new(
        TokenKind::NumberLiteral,
        Some(Literal::Number(value)),
        lexeme,
        location.into(),
    )
}

pub fn identifier(lexeme: &str, location: impl Into<SourceSpan>) -> Token<'_> {
    Token::new(TokenKind::Identifier, None, lexeme, location.into())
}

pub fn and<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::And, None, "and", location.into())
}

pub fn class<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Class, None, "class", location.into())
}

pub fn else_<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Else, None, "else", location.into())
}

pub fn false_<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::False, None, "false", location.into())
}

pub fn fun<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Fun, None, "fun", location.into())
}

pub fn for_<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::For, None, "for", location.into())
}

pub fn if_<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::If, None, "if", location.into())
}

pub fn keyword(kind: TokenKind, lexeme: &str, location: impl Into<SourceSpan>) -> Token<'_> {
    Token::new(kind, None, lexeme, location.into())
}

pub fn nil<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Nil, None, "nil", location.into())
}

pub fn or<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Or, None, "or", location.into())
}

pub fn print<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Print, None, "print", location.into())
}

pub fn return_<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Return, None, "return", location.into())
}

pub fn super_<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::Super, None, "super", location.into())
}

pub fn this<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::This, None, "this", location.into())
}

pub fn true_<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::True, None, "true", location.into())
}

pub fn var(lexeme: &str, location: impl Into<SourceSpan>) -> Token<'_> {
    Token::new(TokenKind::Var, None, lexeme, location.into())
}

pub fn while_<'a>(location: impl Into<SourceSpan>) -> Token<'a> {
    Token::new(TokenKind::While, None, "while", location.into())
}
