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

    #[test]
    fn nil_is_equal_to_nil() {
        assert_that!(Value::Nil == Value::Nil).is_true();
    }

    #[test]
    fn nil_is_not_equal_to_bool() {
        assert_that!(Value::Nil == Value::Bool(false)).is_false();
        assert_that!(Value::Nil == Value::Bool(true)).is_false();
    }

    proptest! {
        #[test]
        fn nil_is_not_equal_to_any_number(
            num in any::<f64>(),
        ) {
            prop_assert!(Value::Nil != Value::Number(num));
        }

        #[test]
        fn nil_is_not_equal_to_any_string(
            strg in any::<String>(),
        ) {
            prop_assert!(Value::Nil != Value::String(strg));
        }
    }

    #[test]
    fn boolean_false_is_equal_to_false() {
        assert_that!(Value::Bool(false) == Value::Bool(false)).is_true();
    }

    #[test]
    fn boolean_false_is_not_equal_to_true() {
        assert_that!(Value::Bool(false) == Value::Bool(true)).is_false();
    }

    #[test]
    fn boolean_true_is_equal_to_true() {
        assert_that!(Value::Bool(true) == Value::Bool(true)).is_true();
    }

    #[test]
    fn boolean_true_is_not_equal_to_false() {
        assert_that!(Value::Bool(true) == Value::Bool(false)).is_false();
    }

    proptest! {
        #[test]
        fn any_boolean_is_not_equal_to_any_number(
            boolean in any::<bool>(),
            num in any::<f64>(),
        ) {
            prop_assert!(Value::Bool(boolean) != Value::Number(num));
        }

        #[test]
        fn any_boolean_is_not_equal_to_any_string(
            boolean in any::<bool>(),
            strg in any::<String>(),
        ) {
            prop_assert!(Value::Bool(boolean) != Value::String(strg));
        }

        #[test]
        fn any_number_is_equal_to_the_same_number(
            num in any::<f64>(),
        ) {
            prop_assert!(Value::Number(num) == Value::Number(num));
        }

        #[allow(clippy::float_cmp)]
        #[test]
        fn any_number_is_not_equal_to_another_number(
            (a, b) in (any::<f64>(), any::<f64>()).prop_filter("a != b", |(a, b)| a != b),
        ) {
            prop_assert!(Value::Number(a) != Value::Number(b));
        }

        #[test]
        fn any_number_is_not_equal_to_any_string(
            num in any::<f64>(),
            strg in any::<String>(),
        ) {
            prop_assert!(Value::Number(num) != Value::String(strg));
        }

        #[test]
        fn any_string_is_equal_to_the_same_string(
            strg in any::<String>(),
        ) {
            prop_assert!(Value::String(strg.clone()) == Value::String(strg));
        }

        #[test]
        fn any_string_is_not_equal_to_another_string(
            (a, b) in (any::<String>(), any::<String>()).prop_filter("a != b", |(a, b)| a != b),
        ) {
            prop_assert!(Value::String(a) != Value::String(b));
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

#[test]
fn evaluate_binary_expr_bangequal_with_numbers() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::BangEqual, "!=", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_bangequal_with_booleans() {
    let expr = Expr::from(Binary::new(
        Literal::Bool(true),
        token(TokenKind::BangEqual, "!=", (1, 2)),
        Literal::Bool(false),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_bangequal_with_strings() {
    let expr = Expr::from(Binary::new(
        Literal::String("Anna".to_string()),
        token(TokenKind::BangEqual, "!=", (1, 2)),
        Literal::String("Anna".to_string()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_bangequal_with_nils() {
    let expr = Expr::from(Binary::new(
        Literal::Nil,
        token(TokenKind::BangEqual, "!=", (1, 2)),
        Literal::Nil,
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_equalequal_with_numbers() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::EqualEqual, "==", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_equalequal_with_booleans() {
    let expr = Expr::from(Binary::new(
        Literal::Bool(true),
        token(TokenKind::EqualEqual, "==", (1, 2)),
        Literal::Bool(false),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_equalequal_with_strings() {
    let expr = Expr::from(Binary::new(
        Literal::String("Anna".to_string()),
        token(TokenKind::EqualEqual, "==", (1, 2)),
        Literal::String("Anna".to_string()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_equalequal_with_nils() {
    let expr = Expr::from(Binary::new(
        Literal::Nil,
        token(TokenKind::EqualEqual, "==", (1, 2)),
        Literal::Nil,
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greater_with_numbers() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::Greater, ">", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_greater_with_booleans() {
    let expr = Expr::from(Binary::new(
        Literal::Bool(true),
        token(TokenKind::Greater, ">", (1, 2)),
        Literal::Bool(false),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greater_with_strings() {
    let expr = Expr::from(Binary::new(
        Literal::String("Billie".to_string()),
        token(TokenKind::Greater, ">", (1, 2)),
        Literal::String("Anna".to_string()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greater_with_nils() {
    let expr = Expr::from(Binary::new(
        Literal::Nil,
        token(TokenKind::Greater, ">", (1, 2)),
        Literal::Nil,
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_greaterequal_with_numbers() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::GreaterEqual, ">=", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_greaterequal_with_booleans() {
    let expr = Expr::from(Binary::new(
        Literal::Bool(true),
        token(TokenKind::GreaterEqual, ">=", (1, 2)),
        Literal::Bool(true),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greaterequal_with_strings() {
    let expr = Expr::from(Binary::new(
        Literal::String("Anna".to_string()),
        token(TokenKind::GreaterEqual, ">=", (1, 2)),
        Literal::String("Anna".to_string()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greaterequal_with_nils() {
    let expr = Expr::from(Binary::new(
        Literal::Nil,
        token(TokenKind::GreaterEqual, ">=", (1, 2)),
        Literal::Nil,
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_less_with_numbers() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::Less, "<", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_less_with_booleans() {
    let expr = Expr::from(Binary::new(
        Literal::Bool(true),
        token(TokenKind::Less, "<", (1, 2)),
        Literal::Bool(false),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_less_with_strings() {
    let expr = Expr::from(Binary::new(
        Literal::String("Anna".to_string()),
        token(TokenKind::Less, "<", (1, 2)),
        Literal::String("Billie".to_string()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_less_with_nils() {
    let expr = Expr::from(Binary::new(
        Literal::Nil,
        token(TokenKind::Less, "<", (1, 2)),
        Literal::Nil,
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_lessequal_with_numbers() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::LessEqual, "<=", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_lessequal_with_booleans() {
    let expr = Expr::from(Binary::new(
        Literal::Bool(false),
        token(TokenKind::LessEqual, "<=", (1, 2)),
        Literal::Bool(true),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_lessequal_with_strings() {
    let expr = Expr::from(Binary::new(
        Literal::String("Anna".to_string()),
        token(TokenKind::LessEqual, "<=", (1, 2)),
        Literal::String("Anna".to_string()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_lessequal_with_nils() {
    let expr = Expr::from(Binary::new(
        Literal::Nil,
        token(TokenKind::LessEqual, "<=", (1, 2)),
        Literal::Nil,
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).is_equal_to(Value::Bool(true));
}
