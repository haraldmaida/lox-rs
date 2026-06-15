use super::*;
use crate::expr::Expr;
use crate::token::{TokenKind, token};
use asserting::prelude::*;
use proptest::prelude::*;

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

    proptest! {
        #[test]
        fn any_number_including_0_and_negative_numbers_is_truthy(
            num in any::<f64>(),
        ) {
            prop_assert!(Value::Number(num).is_truthy());
        }

        #[test]
        fn any_string_including_empty_strings_and_string_of_char_0_is_truthy(
            string in any::<String>(),
        ) {
            prop_assert!(Value::String(string).is_truthy());
        }
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
    let Value::Number(value) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(123.456);
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
    let Value::Number(value) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(123.456);
}

#[test]
fn evaluate_unary_expr_minus() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Minus, "-", (1, 2)),
        Literal::Number(123.456),
    ));

    let mut interpreter = Interpreter::default();
    let Value::Number(value) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(-123.456);
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

#[test]
fn evaluate_binary_expr_minus_with_numbers() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::Minus, "-", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let Value::Number(value) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(-665.556);
}

#[test]
fn evaluate_binary_expr_plus_with_numbers() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::Plus, "+", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let Value::Number(value) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(912.468);
}

#[test]
fn evaluate_binary_expr_plus_with_strings() {
    let expr = Expr::from(Binary::new(
        Literal::String("Hello, ".to_string()),
        token(TokenKind::Plus, "+", (1, 2)),
        Literal::String("world!".to_string()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::String("Hello, world!".into()));
}

#[test]
fn evaluate_binary_expr_star_with_numbers() {
    let expr = Expr::from(Binary::new(
        Literal::Number(-123.456),
        token(TokenKind::Star, "*", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let Value::Number(value) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(-97_408.265_472);
}

#[test]
fn evaluate_binary_expr_slash_with_numbers() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::Slash, "/", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let Value::Number(value) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(0.156_469_103_131_511_3);
}
