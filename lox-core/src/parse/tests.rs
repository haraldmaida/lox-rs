use super::*;
use crate::expr::{Binary, Literal, Unary};
use crate::token::{TokenKind, token};
use crate::tokenize::Tokenize;
use asserting::prelude::*;

#[ignore = "Not implemented yet"]
#[test]
fn parse_empty_source_code() {}

#[test]
fn parse_equality_expression_literal_equal_literal() {
    let mut source_code = "1 == 1";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Binary::new(
            Literal::Number(1.),
            token(TokenKind::EqualEqual, "==", (1, 3)),
            Literal::Number(1.),
        )));
}

#[test]
fn parse_equality_expression_literal_not_equal_literal() {
    let mut source_code = "10 != 1";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Binary::new(
            Literal::Number(10.),
            token(TokenKind::BangEqual, "!=", (1, 4)),
            Literal::Number(1.),
        )));
}

#[test]
fn parse_comparison_expression_literal_greater_literal() {
    let mut source_code = "22 > 1";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Binary::new(
            Literal::Number(22.),
            token(TokenKind::Greater, ">", (1, 4)),
            Literal::Number(1.),
        )));
}

#[test]
fn parse_comparison_expression_literal_greater_equal_literal() {
    let mut source_code = "2 >= 1";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Binary::new(
            Literal::Number(2.),
            token(TokenKind::GreaterEqual, ">=", (1, 3)),
            Literal::Number(1.),
        )));
}

#[test]
fn parse_comparison_expression_literal_less_literal() {
    let mut source_code = "2 < 11";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Binary::new(
            Literal::Number(2.),
            token(TokenKind::Less, "<", (1, 3)),
            Literal::Number(11.),
        )));
}

#[test]
fn parse_comparison_expression_literal_less_equal_literal() {
    let mut source_code = "22 <= 111";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Binary::new(
            Literal::Number(22.),
            token(TokenKind::LessEqual, "<=", (1, 4)),
            Literal::Number(111.),
        )));
}

#[test]
fn parse_term_expression_literal_minus_literal() {
    let mut source_code = "2 - 1";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Binary::new(
            Literal::Number(2.),
            token(TokenKind::Minus, "-", (1, 3)),
            Literal::Number(1.),
        )));
}

#[test]
fn parse_term_expression_literal_plus_literal() {
    let mut source_code = "22 + 11";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Binary::new(
            Literal::Number(22.),
            token(TokenKind::Plus, "+", (1, 4)),
            Literal::Number(11.),
        )));
}

#[test]
fn parse_term_expression_literal_multiplied_by_literal() {
    let mut source_code = "22 * 11";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Binary::new(
            Literal::Number(22.),
            token(TokenKind::Star, "*", (1, 4)),
            Literal::Number(11.),
        )));
}

#[test]
fn parse_term_expression_literal_divided_by_literal() {
    let mut source_code = "6 / 2";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Binary::new(
            Literal::Number(6.),
            token(TokenKind::Slash, "/", (1, 3)),
            Literal::Number(2.),
        )));
}

#[test]
fn parse_unary_expression_not_literal() {
    let mut source_code = "!true";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(Expr::from(Unary::new(
        token(TokenKind::Bang, "!", (1, 1)),
        Literal::Bool(true),
    )));
}

#[test]
fn parse_unary_expression_negate_literal() {
    let mut source_code = "-42";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(Expr::from(Unary::new(
        token(TokenKind::Minus, "-", (1, 1)),
        Literal::Number(42.),
    )));
}

#[test]
fn parse_primary_literal_nil() {
    let mut source_code = "nil";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Literal::Nil));
}

#[test]
fn parse_primary_literal_false() {
    let mut source_code = "false";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Literal::Bool(false)));
}

#[test]
fn parse_primary_literal_true() {
    let mut source_code = "true";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Literal::Bool(true)));
}

#[test]
fn parse_primary_literal_number() {
    let mut source_code = "123.456";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Literal::Number(123.456)));
}

#[test]
fn parse_primary_literal_string() {
    let mut source_code = "\"Hello, World!\"";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Literal::String("Hello, World!".into())));
}

#[test]
fn parse_primary_literal_parens() {
    let mut source_code = "(5 - (3 - 1)) + -1";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Binary::new(
            Grouping::new(Binary::new(
                Literal::Number(5.),
                token(TokenKind::Minus, "-", (1, 4)),
                Grouping::new(Binary::new(
                    Literal::Number(3.),
                    token(TokenKind::Minus, "-", (1, 9)),
                    Literal::Number(1.),
                )),
            )),
            token(TokenKind::Plus, "+", (1, 15)),
            Unary::new(token(TokenKind::Minus, "-", (1, 17)), Literal::Number(1.)),
        )));
}
