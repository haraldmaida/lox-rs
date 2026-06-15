use super::*;
use crate::expr::Expr;
use crate::token::{TokenKind, token};
use asserting::prelude::*;

mod value {
    use super::*;

    #[test]
    fn nil_is_not_truthy() {
        assert_that!(Value::Nil.is_truthy()).is_false();
    }

    #[test]
    fn boolean_false_is_not_truthy() {
        assert_that!(Value::Bool(false).is_truthy()).is_false();
    }

    #[test]
    fn boolean_true_is_truthy() {
        assert_that!(Value::Bool(true).is_truthy()).is_true();
    }

    #[test]
    fn number_0_is_truthy() {
        assert_that!(Value::Number(0.).is_truthy()).is_true();
    }

    #[test]
    fn number_1_is_truthy() {
        assert_that!(Value::Number(1.).is_truthy()).is_true();
    }

    #[test]
    fn number_minus_1_is_truthy() {
        assert_that!(Value::Number(-1.).is_truthy()).is_true();
    }

    #[test]
    fn empty_string_is_truthy() {
        assert_that!(Value::String(String::new()).is_truthy()).is_true();
    }

    #[test]
    fn string_of_char_0_is_truthy() {
        assert_that!(Value::String("0".into()).is_truthy()).is_true();
    }
}

#[test]
fn evaluate_literal_nil() {
    let expr = Expr::from(Literal::Nil);

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Nil);
}

#[test]
fn evaluate_literal_bool() {
    let expr = Expr::from(Literal::Bool(true));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_literal_number() {
    let expr = Expr::from(Literal::Number(123.456));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Number(123.456));
}

#[test]
fn evaluate_literal_string() {
    let expr = Expr::from(Literal::String("Hello, world!".to_string()));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::String("Hello, world!".to_string()));
}

#[test]
fn evaluate_grouping_expression() {
    let expr = Expr::from(Grouping::new(Literal::Number(123.456)));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Number(123.456));
}

#[test]
fn evaluate_unary_expr_minus() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Minus, "-", (1, 2)),
        Literal::Number(123.456),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Number(-123.456));
}

#[test]
fn evaluate_unary_expr_bang_for_true() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Bang, "!", (1, 2)),
        Literal::Bool(true),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_unary_expr_bang_for_false() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Bang, "!", (1, 2)),
        Literal::Bool(false),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_unary_expr_bang_for_number_0() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Bang, "!", (1, 2)),
        Literal::Number(0.),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_unary_expr_bang_for_string() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Bang, "!", (1, 2)),
        Literal::String("0".to_string()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(false));
}
