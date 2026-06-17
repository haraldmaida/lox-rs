use super::*;
use crate::expr::Expr;
use crate::token::{TokenKind, token};
use asserting::prelude::*;

#[test]
fn evaluate_literal_nil() {
    let expr = Expr::from(Literal::Nil);

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Nil);
}

#[test]
fn evaluate_literal_bool() {
    let expr = Expr::from(Literal::Bool(true));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_literal_number() {
    let expr = Expr::from(Literal::Number(123.456));

    let mut interpreter = Interpreter::default();
    let Ok(Value::Number(value)) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(123.456);
}

#[test]
fn evaluate_literal_string() {
    let expr = Expr::from(Literal::String("Hello, world!".into()));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value)
        .ok()
        .is_equal_to(Value::String("Hello, world!".to_string()));
}

#[test]
fn evaluate_grouping_expression() {
    let expr = Expr::from(Grouping::new(Literal::Number(123.456)));

    let mut interpreter = Interpreter::default();
    let Ok(Value::Number(value)) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(123.456);
}

#[test]
fn evaluate_unary_expr_bang_for_true() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Bang, "!", (1, 2)),
        Literal::Bool(true),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_unary_expr_bang_for_false() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Bang, "!", (1, 2)),
        Literal::Bool(false),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_unary_expr_bang_for_number_0() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Bang, "!", (1, 2)),
        Literal::Number(0.),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_unary_expr_bang_for_string() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Bang, "!", (1, 2)),
        Literal::String("0".into()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_unary_expr_minus_with_number() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Minus, "-", (1, 2)),
        Literal::Number(123.456),
    ));

    let mut interpreter = Interpreter::default();
    let Ok(Value::Number(value)) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(-123.456);
}

#[test]
fn evaluate_unary_expr_minus_with_boolean_returns_runtime_error() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Minus, "-", (1, 2)),
        Literal::Bool(true),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        token(TokenKind::Minus, "-", (1, 2)),
    ));
}

#[test]
fn evaluate_unary_expr_minus_with_string_returns_runtime_error() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Minus, "-", (1, 2)),
        Literal::String("Hello, world!".into()),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        token(TokenKind::Minus, "-", (1, 2)),
    ));
}

#[test]
fn evaluate_unary_expr_minus_with_nil_returns_runtime_error() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Minus, "-", (1, 2)),
        Literal::Nil,
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        token(TokenKind::Minus, "-", (1, 2)),
    ));
}

#[test]
fn evaluate_unary_expr_with_illegal_operator() {
    let expr = Expr::from(Unary::new(
        token(TokenKind::Plus, "+", (1, 2)),
        Literal::Number(123.456),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::NotAnUnaryOperator,
        token(TokenKind::Plus, "+", (1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_minus_with_numbers() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::Minus, "-", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let Ok(Value::Number(value)) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(-665.556);
}

#[test]
fn evaluate_binary_expr_minus_with_booleans_returns_runtime_error() {
    let expr = Expr::from(Binary::new(
        Literal::Bool(true),
        token(TokenKind::Minus, "-", (1, 2)),
        Literal::Bool(false),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        token(TokenKind::Minus, "-", (1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_minus_with_strings_returns_runtime_error() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::Minus, "-", (1, 2)),
        Literal::String("Hello, world!".into()),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        token(TokenKind::Minus, "-", (1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_minus_with_nil_returns_runtime_error() {
    let expr = Expr::from(Binary::new(
        Literal::Nil,
        token(TokenKind::Minus, "-", (1, 2)),
        Literal::Number(123.456),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        token(TokenKind::Minus, "-", (1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_plus_with_numbers() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::Plus, "+", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let Ok(Value::Number(value)) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(912.468);
}

#[test]
fn evaluate_binary_expr_plus_with_strings() {
    let expr = Expr::from(Binary::new(
        Literal::String("Hello, ".into()),
        token(TokenKind::Plus, "+", (1, 2)),
        Literal::String("world!".into()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value)
        .ok()
        .is_equal_to(Value::String("Hello, world!".into()));
}

#[test]
fn evaluate_binary_expr_plus_with_string_and_number_returns_runtime_error() {
    let expr = Expr::from(Binary::new(
        Literal::String("Anna".into()),
        token(TokenKind::Plus, "+", (1, 2)),
        Literal::Number(123.456),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandsOfDifferentType,
        token(TokenKind::Plus, "+", (1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_plus_with_number_and_string_returns_runtime_error() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::Plus, "+", (1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandsOfDifferentType,
        token(TokenKind::Plus, "+", (1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_plus_with_booleans_returns_runtime_error() {
    let expr = Expr::from(Binary::new(
        Literal::Bool(true),
        token(TokenKind::Plus, "+", (1, 2)),
        Literal::Bool(false),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumberOrString,
        token(TokenKind::Plus, "+", (1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_plus_with_nil_returns_runtime_error() {
    let expr = Expr::from(Binary::new(
        Literal::Nil,
        token(TokenKind::Plus, "+", (1, 2)),
        Literal::Number(123.456),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumberOrString,
        token(TokenKind::Plus, "+", (1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_star_with_numbers() {
    let expr = Expr::from(Binary::new(
        Literal::Number(-123.456),
        token(TokenKind::Star, "*", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let Ok(Value::Number(value)) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(-97_408.265_472);
}

#[test]
fn evaluate_binary_expr_star_with_booleans_returns_runtime_error() {
    let expr = Expr::from(Binary::new(
        Literal::Bool(true),
        token(TokenKind::Star, "*", (1, 2)),
        Literal::Bool(false),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        token(TokenKind::Star, "*", (1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_star_with_strings_returns_runtime_error() {
    let expr = Expr::from(Binary::new(
        Literal::String("Hello, world!".into()),
        token(TokenKind::Star, "*", (1, 2)),
        Literal::Number(123.456),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        token(TokenKind::Star, "*", (1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_star_with_nil_returns_runtime_error() {
    let expr = Expr::from(Binary::new(
        Literal::Nil,
        token(TokenKind::Star, "*", (1, 2)),
        Literal::Number(123.456),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        token(TokenKind::Star, "*", (1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_slash_with_numbers() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::Slash, "/", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let Ok(Value::Number(value)) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(0.156_469_103_131_511_3);
}

#[test]
fn evaluate_binary_expr_slash_with_booleans_returns_runtime_error() {
    let expr = Expr::from(Binary::new(
        Literal::Bool(true),
        token(TokenKind::Slash, "/", (1, 2)),
        Literal::Bool(false),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        token(TokenKind::Slash, "/", (1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_slash_with_strings_returns_runtime_error() {
    let expr = Expr::from(Binary::new(
        Literal::String("Hello, world!".into()),
        token(TokenKind::Slash, "/", (1, 2)),
        Literal::Number(123.456),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        token(TokenKind::Slash, "/", (1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_slash_with_nil_returns_runtime_error() {
    let expr = Expr::from(Binary::new(
        Literal::Nil,
        token(TokenKind::Slash, "/", (1, 2)),
        Literal::Number(123.456),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        token(TokenKind::Slash, "/", (1, 2)),
    ));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_bangequal_with_strings() {
    let expr = Expr::from(Binary::new(
        Literal::String("Anna".into()),
        token(TokenKind::BangEqual, "!=", (1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_equalequal_with_strings() {
    let expr = Expr::from(Binary::new(
        Literal::String("Anna".into()),
        token(TokenKind::EqualEqual, "==", (1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greater_with_strings() {
    let expr = Expr::from(Binary::new(
        Literal::String("Billie".into()),
        token(TokenKind::Greater, ">", (1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greaterequal_with_strings() {
    let expr = Expr::from(Binary::new(
        Literal::String("Anna".into()),
        token(TokenKind::GreaterEqual, ">=", (1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_less_with_strings() {
    let expr = Expr::from(Binary::new(
        Literal::String("Anna".into()),
        token(TokenKind::Less, "<", (1, 2)),
        Literal::String("Billie".into()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_lessequal_with_strings() {
    let expr = Expr::from(Binary::new(
        Literal::String("Anna".into()),
        token(TokenKind::LessEqual, "<=", (1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
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

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_with_illegal_operator() {
    let expr = Expr::from(Binary::new(
        Literal::Number(123.456),
        token(TokenKind::Bang, "!", (1, 2)),
        Literal::Number(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::NotABinaryOperator,
        token(TokenKind::Bang, "!", (1, 2)),
    ));
}

#[test]
fn execute_print_stmt_with_expression() {
    let stmt = Stmt::from(Print::new(Binary::new(
        Literal::Number(84.),
        token(TokenKind::Plus, "/", (1, 2)),
        Literal::Number(2.),
    )));

    let mut interpreter = Interpreter::default();
    let result = interpreter.execute(&stmt);

    assert_that!(result).is_ok();
}

#[test]
fn execute_var_stmt_with_initializer() {
    let stmt = Stmt::from(Var::new(
        token(TokenKind::Identifier, "my_var", (4, 6)),
        Expr::from(Binary::new(
            Literal::Number(40.),
            token(TokenKind::Plus, "+", (17, 1)),
            Literal::Number(2.),
        )),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.execute(&stmt);

    assert_that!(result).is_ok();
    assert_that!(interpreter.environment().get("my_var")).is_equal_to(Ok(&Value::Number(42.)));
}

#[test]
fn execute_var_stmt_without_initializer() {
    let stmt = Stmt::from(Var::new(token(TokenKind::Identifier, "foo", (4, 3)), None));

    let mut interpreter = Interpreter::default();
    let result = interpreter.execute(&stmt);

    assert_that!(result).is_ok();
    assert_that!(interpreter.environment().get("foo")).is_equal_to(Ok(&Value::Nil));
}

#[test]
fn execute_var_stmt_with_variable_expression() {
    let declare_a = Stmt::from(Var::new(
        token(TokenKind::Identifier, "a", (4, 1)),
        Expr::from(Literal::Number(3.)),
    ));
    let declare_b = Stmt::from(Var::new(
        token(TokenKind::Identifier, "b", (14, 1)),
        Expr::from(Literal::Number(2.)),
    ));
    let var_stmt = Stmt::from(Var::new(
        token(TokenKind::Identifier, "foo", (24, 3)),
        Expr::from(Binary::new(
            Variable::new(token(TokenKind::Identifier, "a", (30, 1))),
            token(TokenKind::Plus, "+", (32, 1)),
            Variable::new(token(TokenKind::Identifier, "b", (34, 1))),
        )),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.execute(&declare_a);
    assert_that!(result).is_ok();
    let result = interpreter.execute(&declare_b);
    assert_that!(result).is_ok();

    let result = interpreter.execute(&var_stmt);

    assert_that!(result).is_ok();
    assert_that!(interpreter.environment().get("foo")).is_equal_to(Ok(&Value::Number(5.)));
}

#[test]
fn execute_print_stmt_of_undefined_variable() {
    let stmt = Stmt::from(Print::new(Variable::new(token(
        TokenKind::Identifier,
        "foo",
        (4, 3),
    ))));

    let mut interpreter = Interpreter::default();
    let result = interpreter.execute(&stmt);

    assert_that!(result).err().is_equal_to(RuntimeError::new(
        RuntimeErrorCode::UndefinedVariable("foo".into()),
        token(TokenKind::Identifier, "foo", (4, 3)),
    ));
}

#[test]
fn evaluate_assign_expr_stmt_to_existing_variable() {
    let declare_foo = Stmt::from(Var::new(
        token(TokenKind::Identifier, "foo", (4, 3)),
        Expr::from(Literal::Number(123.)),
    ));

    let assign_to_foo = Expr::from(Assign::new(
        token(TokenKind::Identifier, "foo", (23, 3)),
        Expr::from(Literal::Number(99.)),
    ));

    let mut interpreter = Interpreter::default();
    let declare_result = interpreter.execute(&declare_foo);
    assert_that!(declare_result).is_ok();

    let assign_result = interpreter.evaluate(&assign_to_foo);

    assert_that!(assign_result).is_equal_to(Ok(Value::Number(99.)));
    assert_that!(interpreter.environment().get("foo")).is_equal_to(Ok(&Value::Number(99.)));
}

#[test]
fn evaluate_assign_expr_stmt_to_not_existing_variable() {
    let declare_foo = Stmt::from(Var::new(
        token(TokenKind::Identifier, "a", (4, 1)),
        Expr::from(Literal::Number(123.)),
    ));

    let assign_to_foo = Expr::from(Assign::new(
        token(TokenKind::Identifier, "foo", (23, 3)),
        Expr::from(Literal::Number(99.)),
    ));

    let mut interpreter = Interpreter::default();
    let declare_result = interpreter.execute(&declare_foo);
    assert_that!(declare_result).is_ok();

    let assign_result = interpreter.evaluate(&assign_to_foo);

    assert_that!(assign_result)
        .err()
        .is_equal_to(RuntimeError::new(
            RuntimeErrorCode::UndefinedVariable("foo".into()),
            token(TokenKind::Identifier, "foo", (23, 3)),
        ));
    assert_that!(interpreter.environment().get("foo"))
        .is_equal_to(Err(EnvironmentError::UndefinedVariable("foo".into())));
    assert_that!(interpreter.environment().get("a")).is_equal_to(Ok(&Value::Number(123.)));
}
