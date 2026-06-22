use super::*;
use crate::data::value;
use crate::expr::{
    Expr, ExprExt, assign, binary, grouping, literal, logical, nil, unary, variable,
};
use crate::parse::Parse;
use crate::stmt::{IfExt, StmtExt, block, function, if_, print, return_, var, while_};
use crate::token::{
    and, bang, bang_equal, equal_equal, greater, greater_equal, identifier, keyword, less,
    less_equal, minus, or, plus, slash, star,
};
use crate::tokenize::Tokenize;
use asserting::prelude::*;
use std::time::SystemTime;

#[test]
fn evaluate_literal_nil() {
    let expr = Expr::from(nil());

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Nil);
}

#[test]
fn evaluate_literal_bool() {
    let expr = Expr::from(literal(true));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_literal_number() {
    let expr = Expr::from(literal(123.456));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let Ok(Value::Number(value)) = interpreter.evaluate(&mut rtc, &expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(123.456);
}

#[test]
fn evaluate_literal_string() {
    let expr = Expr::from(Literal::String("Hello, world!".into()));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value)
        .ok()
        .is_equal_to(Value::String("Hello, world!".to_string()));
}

#[test]
fn evaluate_grouping_expression() {
    let expr = Expr::from(grouping(literal(123.456)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let Ok(Value::Number(value)) = interpreter.evaluate(&mut rtc, &expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(123.456);
}

#[test]
fn evaluate_unary_expr_bang_for_true() {
    let expr = Expr::from(unary(bang((1, 2)), literal(true)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_unary_expr_bang_for_false() {
    let expr = Expr::from(unary(bang((1, 2)), literal(false)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_unary_expr_bang_for_number_0() {
    let expr = Expr::from(unary(bang((1, 2)), literal(0.)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_unary_expr_bang_for_string() {
    let expr = Expr::from(unary(bang((1, 2)), Literal::String("0".into())));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_unary_expr_minus_with_number() {
    let expr = Expr::from(unary(minus((1, 2)), literal(123.456)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let Ok(Value::Number(value)) = interpreter.evaluate(&mut rtc, &expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(-123.456);
}

#[test]
fn evaluate_unary_expr_minus_with_boolean_returns_runtime_error() {
    let expr = Expr::from(unary(minus((1, 2)), literal(true)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        minus((1, 2)),
    ));
}

#[test]
fn evaluate_unary_expr_minus_with_string_returns_runtime_error() {
    let expr = Expr::from(unary(
        minus((1, 2)),
        Literal::String("Hello, world!".into()),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        minus((1, 2)),
    ));
}

#[test]
fn evaluate_unary_expr_minus_with_nil_returns_runtime_error() {
    let expr = Expr::from(unary(minus((1, 2)), Literal::Nil));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        minus((1, 2)),
    ));
}

#[test]
fn evaluate_unary_expr_with_illegal_operator() {
    let expr = Expr::from(unary(plus((1, 2)), literal(123.456)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::NotAnUnaryOperator,
        plus((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_minus_with_numbers() {
    let expr = Expr::from(binary(literal(123.456), minus((1, 2)), literal(789.012)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let Ok(Value::Number(value)) = interpreter.evaluate(&mut rtc, &expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(-665.556);
}

#[test]
fn evaluate_binary_expr_minus_with_booleans_returns_runtime_error() {
    let expr = Expr::from(binary(literal(true), minus((1, 2)), literal(false)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        minus((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_minus_with_strings_returns_runtime_error() {
    let expr = Expr::from(binary(
        literal(123.456),
        minus((1, 2)),
        Literal::String("Hello, world!".into()),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        minus((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_minus_with_nil_returns_runtime_error() {
    let expr = Expr::from(binary(Literal::Nil, minus((1, 2)), literal(123.456)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        minus((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_plus_with_numbers() {
    let expr = Expr::from(binary(literal(123.456), plus((1, 2)), literal(789.012)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let Ok(Value::Number(value)) = interpreter.evaluate(&mut rtc, &expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(912.468);
}

#[test]
fn evaluate_binary_expr_plus_with_strings() {
    let expr = Expr::from(binary(
        Literal::String("Hello, ".into()),
        plus((1, 2)),
        Literal::String("world!".into()),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value)
        .ok()
        .is_equal_to(Value::String("Hello, world!".into()));
}

#[test]
fn evaluate_binary_expr_plus_with_string_and_number_returns_runtime_error() {
    let expr = Expr::from(binary(
        Literal::String("Anna".into()),
        plus((1, 2)),
        literal(123.456),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandsOfDifferentType,
        plus((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_plus_with_number_and_string_returns_runtime_error() {
    let expr = Expr::from(binary(
        literal(123.456),
        plus((1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandsOfDifferentType,
        plus((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_plus_with_booleans_returns_runtime_error() {
    let expr = Expr::from(binary(literal(true), plus((1, 2)), literal(false)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumberOrString,
        plus((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_plus_with_nil_returns_runtime_error() {
    let expr = Expr::from(binary(Literal::Nil, plus((1, 2)), literal(123.456)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumberOrString,
        plus((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_star_with_numbers() {
    let expr = Expr::from(binary(literal(-123.456), star((1, 2)), literal(789.012)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let Ok(Value::Number(value)) = interpreter.evaluate(&mut rtc, &expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(-97_408.265_472);
}

#[test]
fn evaluate_binary_expr_star_with_booleans_returns_runtime_error() {
    let expr = Expr::from(binary(literal(true), star((1, 2)), literal(false)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        star((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_star_with_strings_returns_runtime_error() {
    let expr = Expr::from(binary(
        Literal::String("Hello, world!".into()),
        star((1, 2)),
        literal(123.456),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        star((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_star_with_nil_returns_runtime_error() {
    let expr = Expr::from(binary(Literal::Nil, star((1, 2)), literal(123.456)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        star((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_slash_with_numbers() {
    let expr = Expr::from(binary(literal(123.456), slash((1, 2)), literal(789.012)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let Ok(Value::Number(value)) = interpreter.evaluate(&mut rtc, &expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(0.156_469_103_131_511_3);
}

#[test]
fn evaluate_binary_expr_slash_with_booleans_returns_runtime_error() {
    let expr = Expr::from(binary(literal(true), slash((1, 2)), literal(false)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        slash((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_slash_with_strings_returns_runtime_error() {
    let expr = Expr::from(binary(
        Literal::String("Hello, world!".into()),
        slash((1, 2)),
        literal(123.456),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        slash((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_slash_with_nil_returns_runtime_error() {
    let expr = Expr::from(binary(Literal::Nil, slash((1, 2)), literal(123.456)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        slash((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_bangequal_with_numbers() {
    let expr = Expr::from(binary(
        literal(123.456),
        bang_equal((1, 2)),
        literal(789.012),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_bangequal_with_booleans() {
    let expr = Expr::from(binary(literal(true), bang_equal((1, 2)), literal(false)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_bangequal_with_strings() {
    let expr = Expr::from(binary(
        Literal::String("Anna".into()),
        bang_equal((1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_bangequal_with_nils() {
    let expr = Expr::from(binary(Literal::Nil, bang_equal((1, 2)), Literal::Nil));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_equalequal_with_numbers() {
    let expr = Expr::from(binary(
        literal(123.456),
        equal_equal((1, 2)),
        literal(789.012),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_equalequal_with_booleans() {
    let expr = Expr::from(binary(literal(true), equal_equal((1, 2)), literal(false)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_equalequal_with_strings() {
    let expr = Expr::from(binary(
        Literal::String("Anna".into()),
        equal_equal((1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_equalequal_with_nils() {
    let expr = Expr::from(binary(Literal::Nil, equal_equal((1, 2)), Literal::Nil));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greater_with_numbers() {
    let expr = Expr::from(binary(literal(123.456), greater((1, 2)), literal(789.012)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_greater_with_booleans() {
    let expr = Expr::from(binary(literal(true), greater((1, 2)), literal(false)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greater_with_strings() {
    let expr = Expr::from(binary(
        Literal::String("Billie".into()),
        greater((1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greater_with_nils() {
    let expr = Expr::from(binary(Literal::Nil, greater((1, 2)), Literal::Nil));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_greaterequal_with_numbers() {
    let expr = Expr::from(binary(
        literal(123.456),
        greater_equal((1, 2)),
        literal(789.012),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_greaterequal_with_booleans() {
    let expr = Expr::from(binary(literal(true), greater_equal((1, 2)), literal(true)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greaterequal_with_strings() {
    let expr = Expr::from(binary(
        Literal::String("Anna".into()),
        greater_equal((1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greaterequal_with_nils() {
    let expr = Expr::from(binary(Literal::Nil, greater_equal((1, 2)), Literal::Nil));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_less_with_numbers() {
    let expr = Expr::from(binary(literal(123.456), less((1, 2)), literal(789.012)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_less_with_booleans() {
    let expr = Expr::from(binary(literal(true), less((1, 2)), literal(false)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_less_with_strings() {
    let expr = Expr::from(binary(
        Literal::String("Anna".into()),
        less((1, 2)),
        Literal::String("Billie".into()),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_less_with_nils() {
    let expr = Expr::from(binary(Literal::Nil, less((1, 2)), Literal::Nil));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_lessequal_with_numbers() {
    let expr = Expr::from(binary(
        literal(123.456),
        less_equal((1, 2)),
        literal(789.012),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_lessequal_with_booleans() {
    let expr = Expr::from(binary(literal(false), less_equal((1, 2)), literal(true)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_lessequal_with_strings() {
    let expr = Expr::from(binary(
        Literal::String("Anna".into()),
        less_equal((1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_lessequal_with_nils() {
    let expr = Expr::from(binary(Literal::Nil, less_equal((1, 2)), Literal::Nil));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let value = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_with_illegal_operator() {
    let expr = Expr::from(binary(literal(123.456), bang((1, 2)), literal(789.012)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::NotABinaryOperator,
        bang((1, 2)),
    ));
}

#[test]
fn execute_print_stmt_with_expression() {
    let stmt = Stmt::from(print(binary(literal(84.), plus((1, 2)), literal(2.))));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &stmt);

    assert_that!(result).is_ok();
    assert_that!(String::from_utf8(stdout))
        .ok()
        .is_equal_to("86\n");
}

#[test]
fn execute_var_stmt_with_initializer() {
    let stmt = Stmt::from(var(
        identifier("my_var", (4, 6)),
        binary(literal(40.), plus((17, 1)), literal(2.)).expr(),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &stmt);

    assert_that!(result).is_ok();
    assert_that!(interpreter.environment().lookup("my_var")).is_equal_to(Ok(Value::Number(42.)));
}

#[test]
fn execute_var_stmt_without_initializer() {
    let stmt = Stmt::from(var(identifier("foo", (4, 3)), None));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &stmt);

    assert_that!(result).is_ok();
    assert_that!(interpreter.environment().lookup("foo")).is_equal_to(Ok(Value::Nil));
}

#[test]
fn execute_var_stmt_with_variable_expression() {
    let declare_a = Stmt::from(var(identifier("a", (4, 1)), literal(3.).expr()));
    let declare_b = Stmt::from(var(identifier("b", (14, 1)), literal(2.).expr()));
    let var_stmt = Stmt::from(var(
        identifier("foo", (24, 3)),
        binary(
            variable(identifier("a", (30, 1))),
            plus((32, 1)),
            variable(identifier("b", (34, 1))),
        )
        .expr(),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &declare_a);
    assert_that!(result).is_ok();
    let result = interpreter.execute(&mut rtc, &declare_b);
    assert_that!(result).is_ok();

    let result = interpreter.execute(&mut rtc, &var_stmt);

    assert_that!(result).is_ok();
    assert_that!(interpreter.environment().lookup("foo")).is_equal_to(Ok(Value::Number(5.)));
}

#[test]
fn execute_print_stmt_of_undefined_variable() {
    let stmt = Stmt::from(print(variable(identifier("foo", (4, 3)))));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &stmt);

    assert_that!(result).err().is_equal_to(RuntimeError::new(
        RuntimeErrorCode::UndefinedVariable("foo".into()),
        identifier("foo", (4, 3)),
    ));
}

#[test]
fn evaluate_assign_expr_stmt_to_existing_variable() {
    let declare_foo = Stmt::from(var(identifier("foo", (4, 3)), literal(123.).expr()));

    let assign_to_foo = Expr::from(assign(identifier("foo", (23, 3)), literal(99.)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let declare_result = interpreter.execute(&mut rtc, &declare_foo);
    assert_that!(declare_result).is_ok();

    let assign_result = interpreter.evaluate(&mut rtc, &assign_to_foo);

    assert_that!(assign_result).is_equal_to(Ok(Value::Number(99.)));
    assert_that!(interpreter.environment().lookup("foo")).is_equal_to(Ok(Value::Number(99.)));
}

#[test]
fn evaluate_assign_expr_stmt_to_not_existing_variable() {
    let declare_foo = Stmt::from(var(identifier("a", (4, 1)), literal(123.).expr()));

    let assign_to_foo = Expr::from(assign(identifier("foo", (23, 3)), literal(99.)));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let declare_result = interpreter.execute(&mut rtc, &declare_foo);
    assert_that!(declare_result).is_ok();

    let assign_result = interpreter.evaluate(&mut rtc, &assign_to_foo);

    assert_that!(assign_result)
        .err()
        .is_equal_to(RuntimeError::new(
            RuntimeErrorCode::UndefinedVariable("foo".into()),
            identifier("foo", (23, 3)),
        ));
    assert_that!(interpreter.environment().lookup("foo"))
        .is_equal_to(Err(EnvironmentError::IdentifierNotFound("foo".into())));
    assert_that!(interpreter.environment().lookup("a")).is_equal_to(Ok(Value::Number(123.)));
}

#[test]
fn execute_block_that_is_empty() {
    let stmt = Stmt::from(block(vec![]));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &stmt);

    assert_that!(result).is_ok();
}

#[test]
fn execute_block_with_var_declarations_and_assignments() {
    let declare_a = Stmt::from(var(identifier("a", (4, 1)), literal(3.).expr()));
    let declare_b = Stmt::from(var(identifier("b", (14, 1)), literal(2.).expr()));
    let assign_b = Expr::from(assign(identifier("b", (24, 1)), literal(7.).expr()));
    let assign_a = Expr::from(assign(identifier("a", (34, 1)), literal(5.).expr()));
    let block = Stmt::from(block(vec![declare_b, assign_b.stmt(), assign_a.stmt()]));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &declare_a);
    assert_that!(result).is_ok();

    let result = interpreter.execute(&mut rtc, &block);

    assert_that!(result).is_ok();
    assert_that!(interpreter.environment().lookup("a")).is_equal_to(Ok(Value::Number(5.)));
    assert_that!(interpreter.environment().lookup("b"))
        .err()
        .is_equal_to(EnvironmentError::IdentifierNotFound("b".into()));
}

#[test]
fn execute_block_with_var_declarations_and_assignments_and_runtime_error() {
    let declare_a = Stmt::from(var(identifier("a", (4, 1)), literal(3.).expr()));
    let declare_b = Stmt::from(var(identifier("b", (14, 1)), literal(2.).expr()));
    let assign_c = Expr::from(assign(identifier("c", (24, 1)), literal(7.).expr()));
    let assign_a = Expr::from(assign(identifier("a", (34, 1)), literal(5.).expr()));
    let block = Stmt::from(block(vec![declare_b, assign_c.stmt(), assign_a.stmt()]));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &declare_a);
    assert_that!(result).is_ok();

    let result = interpreter.execute(&mut rtc, &block);

    assert_that!(result).err().is_equal_to(RuntimeError::new(
        RuntimeErrorCode::UndefinedVariable("c".into()),
        identifier("c", (24, 1)),
    ));
    assert_that!(interpreter.environment().lookup("a")).is_equal_to(Ok(Value::Number(3.)));
    assert_that!(interpreter.environment().lookup("b"))
        .err()
        .is_equal_to(EnvironmentError::IdentifierNotFound("b".into()));
}

#[test]
fn execute_if_stmt_single_then_stmt_no_else_branch() {
    let stmt = Stmt::from(if_(literal(true), print(literal("Hello, World!")).stmt()));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &stmt);

    assert_that!(result).is_ok();
    assert_that!(String::from_utf8(stdout))
        .ok()
        .is_equal_to("Hello, World!\n");
}

#[test]
fn execute_if_stmt_single_then_stmt_and_single_else_stmt() {
    let stmt = Stmt::from(
        if_(
            binary(literal(42.), greater((7, 1)), literal(43.)),
            print(literal("Hello, World!")).stmt(),
        )
        .else_(print(literal("Goodbye, World!")).stmt()),
    );

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &stmt);

    assert_that!(result).is_ok();
    assert_that!(String::from_utf8(stdout))
        .ok()
        .is_equal_to("Goodbye, World!\n");
}

#[test]
fn execute_if_stmt_multiple_then_stmts_no_else_branch() {
    let declare_x = Stmt::from(var(identifier("x", (4, 1)), None));
    let stmt = Stmt::from(if_(
        literal(true),
        block([
            assign(
                identifier("x", (12, 1)),
                binary(literal(4.), plus((18, 1)), literal(3.)).expr(),
            )
            .expr()
            .stmt(),
            print(variable(identifier("x", (29, 1)))).stmt(),
        ]),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &declare_x);
    assert_that!(result).is_ok();

    let result = interpreter.execute(&mut rtc, &stmt);

    assert_that!(result).is_ok();
    assert_that!(String::from_utf8(stdout))
        .ok()
        .is_equal_to("7\n");
}

#[test]
fn execute_if_stmt_multiple_then_stmts_and_multiple_else_stmts() {
    let declare_x = Stmt::from(var(identifier("x", (4, 1)), literal(99.).expr()));
    let stmt = Stmt::from(
        if_(
            binary(variable(identifier("x", (4, 1))), less((6, 1)), literal(0.)),
            block([
                assign(
                    identifier("x", (13, 1)),
                    binary(
                        variable(identifier("x", (17, 1))),
                        minus((19, 1)),
                        literal(3.),
                    )
                    .expr(),
                )
                .expr()
                .stmt(),
                print(variable(identifier("x", (30, 1)))).stmt(),
            ]),
        )
        .else_(block([
            assign(
                identifier("x", (42, 1)),
                binary(
                    variable(identifier("x", (46, 1))),
                    plus((48, 1)),
                    literal(3.),
                )
                .expr(),
            )
            .expr()
            .stmt(),
            print(variable(identifier("x", (59, 1)))).stmt(),
        ])),
    );

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &declare_x);
    assert_that!(result).is_ok();

    let result = interpreter.execute(&mut rtc, &stmt);

    assert_that!(result).is_ok();
    assert_that!(String::from_utf8(stdout))
        .ok()
        .is_equal_to("102\n");
}

#[test]
fn execute_if_without_else_branch_and_nested_if_with_else_branch() {
    let declare_x = Stmt::from(var(identifier("x", (4, 1)), literal(0.).expr()));
    let declare_y = Stmt::from(var(identifier("y", (4, 1)), None));
    let stmt = Stmt::from(if_(
        variable(identifier("x", (4, 1))),
        if_(
            variable(identifier("y", (11, 1))),
            print(literal("x and y")),
        )
        .else_(print(literal("x only"))),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &declare_x);
    assert_that!(result).is_ok();
    let result = interpreter.execute(&mut rtc, &declare_y);
    assert_that!(result).is_ok();

    let result = interpreter.execute(&mut rtc, &stmt);

    assert_that!(result).is_ok();
    assert_that!(String::from_utf8(stdout))
        .ok()
        .is_equal_to("x only\n");
}

#[test]
fn evaluate_logical_and_where_both_conditions_are_true() {
    let declare_x = Stmt::from(var(identifier("x", (4, 1)), literal(25.).expr()));
    let declare_y = Stmt::from(var(identifier("y", (4, 1)), literal(10.).expr()));
    let expr = Expr::from(logical(
        binary(
            assign(identifier("x", (4, 1)), literal(50.)),
            equal_equal((19, 2)),
            literal(50.),
        ),
        and((6, 3)),
        binary(
            assign(identifier("y", (4, 1)), literal(0.)),
            equal_equal((19, 2)),
            literal(0.),
        ),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let x_result = interpreter.execute(&mut rtc, &declare_x);
    assert_that!(x_result).is_ok();
    let y_result = interpreter.execute(&mut rtc, &declare_y);
    assert_that!(y_result).is_ok();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).ok().is_equal_to(value(true));
    // expecting that both assignments are executed
    assert_that!(interpreter.environment().lookup("x")).is_equal_to(Ok(Value::Number(50.)));
    assert_that!(interpreter.environment().lookup("y")).is_equal_to(Ok(Value::Number(0.)));
}

#[test]
fn evaluate_logical_and_where_first_condition_is_false() {
    let declare_x = Stmt::from(var(identifier("x", (4, 1)), literal(25.).expr()));
    let declare_y = Stmt::from(var(identifier("y", (4, 1)), literal(10.).expr()));
    let expr = Expr::from(logical(
        binary(
            assign(identifier("x", (4, 1)), literal(50.)),
            equal_equal((19, 2)),
            literal(25.),
        ),
        and((6, 3)),
        binary(
            assign(identifier("y", (4, 1)), literal(0.)),
            equal_equal((19, 2)),
            literal(0.),
        ),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let x_result = interpreter.execute(&mut rtc, &declare_x);
    assert_that!(x_result).is_ok();
    let y_result = interpreter.execute(&mut rtc, &declare_y);
    assert_that!(y_result).is_ok();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).ok().is_equal_to(value(false));
    // expecting the left assignment is executed
    assert_that!(interpreter.environment().lookup("x")).is_equal_to(Ok(Value::Number(50.)));
    // expecting the `and` stops evaluating after the left condition because it is `false`
    // second assignment right from the `and` is not executed
    assert_that!(interpreter.environment().lookup("y")).is_equal_to(Ok(Value::Number(10.)));
}

#[test]
fn evaluate_logical_or_where_first_condition_is_true() {
    let declare_x = Stmt::from(var(identifier("x", (4, 1)), literal(25.).expr()));
    let declare_y = Stmt::from(var(identifier("y", (4, 1)), literal(10.).expr()));
    let expr = Expr::from(logical(
        binary(
            assign(identifier("x", (4, 1)), literal(50.)),
            equal_equal((19, 2)),
            literal(50.),
        ),
        or((6, 3)),
        binary(
            assign(identifier("y", (4, 1)), literal(0.)),
            equal_equal((19, 2)),
            literal(0.),
        ),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let x_result = interpreter.execute(&mut rtc, &declare_x);
    assert_that!(x_result).is_ok();
    let y_result = interpreter.execute(&mut rtc, &declare_y);
    assert_that!(y_result).is_ok();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).ok().is_equal_to(value(true));
    // expecting the left assignment is executed
    assert_that!(interpreter.environment().lookup("x")).is_equal_to(Ok(Value::Number(50.)));
    // expecting the `or` stops after the first condition because its `true`
    // second assignment right from the `or` is not executed
    assert_that!(interpreter.environment().lookup("y")).is_equal_to(Ok(Value::Number(10.)));
}

#[test]
fn evaluate_logical_or_where_second_condition_is_true() {
    let declare_x = Stmt::from(var(identifier("x", (4, 1)), literal(25.).expr()));
    let declare_y = Stmt::from(var(identifier("y", (4, 1)), literal(10.).expr()));
    let expr = Expr::from(logical(
        binary(
            assign(identifier("x", (4, 1)), literal(50.)),
            equal_equal((19, 2)),
            literal(25.),
        ),
        or((6, 3)),
        binary(
            assign(identifier("y", (4, 1)), literal(0.)),
            equal_equal((19, 2)),
            literal(0.),
        ),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let x_result = interpreter.execute(&mut rtc, &declare_x);
    assert_that!(x_result).is_ok();
    let y_result = interpreter.execute(&mut rtc, &declare_y);
    assert_that!(y_result).is_ok();

    let result = interpreter.evaluate(&mut rtc, &expr);

    assert_that!(result).ok().is_equal_to(value(true));
    // expecting that both assignments are executed
    assert_that!(interpreter.environment().lookup("x")).is_equal_to(Ok(Value::Number(50.)));
    assert_that!(interpreter.environment().lookup("y")).is_equal_to(Ok(Value::Number(0.)));
}

#[test]
fn execute_while_loop_with_single_statement() {
    let declare_x = Stmt::from(var(identifier("x", (4, 1)), literal(0.).expr()));
    let while_loop = Stmt::from(while_(
        binary(
            variable(identifier("x", (7, 1))),
            less((9, 1)),
            literal(10.),
        ),
        assign(
            identifier("x", (15, 1)),
            binary(
                variable(identifier("x", (19, 1))),
                plus((21, 1)),
                literal(1.),
            ),
        )
        .expr(),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &declare_x);
    assert_that!(result).is_ok();

    let result = interpreter.execute(&mut rtc, &while_loop);

    assert_that!(result).is_ok();
    assert_that!(interpreter.environment().lookup("x")).is_equal_to(Ok(Value::Number(10.)));
    assert_that!(String::from_utf8(stdout)).ok().is_equal_to("");
}

#[test]
fn execute_while_loop_with_multiple_statements() {
    let declare_x = Stmt::from(var(identifier("x", (4, 1)), literal(0.).expr()));
    let while_loop = Stmt::from(while_(
        binary(
            variable(identifier("x", (7, 1))),
            less((9, 1)),
            literal(10.),
        ),
        block([
            assign(
                identifier("x", (17, 1)),
                binary(
                    variable(identifier("x", (21, 1))),
                    plus((23, 1)),
                    literal(1.),
                ),
            )
            .expr()
            .stmt(),
            print(variable(identifier("x", (34, 1)))).stmt(),
        ]),
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &declare_x);
    assert_that!(result).is_ok();

    let result = interpreter.execute(&mut rtc, &while_loop);

    assert_that!(result).is_ok();
    assert_that!(interpreter.environment().lookup("x")).is_equal_to(Ok(Value::Number(10.)));
    assert_that!(String::from_utf8(stdout))
        .ok()
        .is_equal_to("1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n");
}

#[test]
fn execute_function_declaration() {
    let stmt = Stmt::from(function(
        identifier("foo", (4, 1)),
        vec![identifier("x", (8, 1)), identifier("y", (11, 1))],
        [return_(
            keyword(TokenKind::Return, "return", (16, 6)),
            binary(
                variable(identifier("x", (23, 1))),
                plus((25, 1)),
                variable(identifier("y", (27, 1))),
            )
            .expr(),
        )
        .stmt()],
    ));

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    let result = interpreter.execute(&mut rtc, &stmt);

    assert_that!(result).is_ok();
    assert_that!(interpreter.environment().lookup("foo"))
        .ok()
        .is_equal_to(Value::from(LoxFunction::new(
            function(
                identifier("foo", (4, 1)),
                vec![identifier("x", (8, 1)), identifier("y", (11, 1))],
                [return_(
                    keyword(TokenKind::Return, "return", (16, 6)),
                    binary(
                        variable(identifier("x", (23, 1))),
                        plus((25, 1)),
                        variable(identifier("y", (27, 1))),
                    )
                    .expr(),
                )
                .stmt()],
            ),
            interpreter.environment().clone(),
        )));
}

#[test]
fn execute_function_declaration_and_call() {
    let source_code = r#"
    fun sayHi(first, last) {
        print "Hi, " + first + " " + last + "!";
    }

    sayHi("Dear", "Reader");
"#;

    let program = source_code
        .tokenize()
        .parse()
        .expect("failed to parse source code");

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    interpreter.interpret(&mut rtc, &program);

    assert_that!(String::from_utf8(stderr)).ok().is_empty();
    assert_that!(String::from_utf8(stdout))
        .ok()
        .is_equal_to("Hi, Dear Reader!\n");
}

#[test]
fn execute_function_declaration_and_call_with_return_value() {
    let source_code = r"
    fun fib(n) {
        if (n <= 1) return n;
        return fib(n - 2) + fib(n - 1);
    }

    for (var i = 0; i < 20; i = i + 1) {
        print fib(i);
    }
";
    let program = source_code
        .tokenize()
        .parse()
        .expect("failed to parse source code");

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    interpreter.interpret(&mut rtc, &program);

    assert_that!(String::from_utf8(stderr)).ok().is_empty();
    assert_that!(String::from_utf8(stdout)).ok().is_equal_to(
        "0\n1\n1\n2\n3\n5\n8\n13\n21\n34\n55\n89\n144\n233\n377\n610\n987\n1597\n2584\n4181\n",
    );
}

fn system_time_as_secs() -> f64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

#[test]
fn execute_native_function_call_to_clock() {
    let start_time = system_time_as_secs();

    let source_code = "print clock();";

    let program = source_code
        .tokenize()
        .parse()
        .expect("failed to parse source code");

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    interpreter.interpret(&mut rtc, &program);

    assert_that!(String::from_utf8(stderr)).ok().is_empty();
    let result = String::from_utf8(stdout)
        .map_err(|err| err.to_string())
        .and_then(|s| s.trim().parse::<f64>().map_err(|err| err.to_string()));

    let end_time = system_time_as_secs();

    assert_that!(result)
        .ok()
        .is_not_close_to(0.)
        .is_between(start_time, end_time);
}

#[test]
fn execute_closure_count() {
    let source_code = r"
    fun makeCounter() {
        var i = 0;

        fun count() {
            i = i + 1;
            return i;
        }

        return count;
    }

    var counter = makeCounter();
    print counter();
    print counter();
    print counter();
";

    let program = source_code
        .tokenize()
        .parse()
        .expect("failed to parse source code");

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);
    let mut interpreter = Interpreter::default();

    interpreter.interpret(&mut rtc, &program);

    assert_that!(String::from_utf8(stderr)).ok().is_empty();
    assert_that!(String::from_utf8(stdout))
        .ok()
        .is_equal_to("1\n2\n3\n");
}
