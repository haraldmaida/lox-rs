use super::*;
use crate::expr::{Expr, ExprExt, assign, binary, grouping, literal, nil, unary, variable};
use crate::stmt::{StmtExt, block, print, var};
use crate::token::{
    bang, bang_equal, equal_equal, greater, greater_equal, identifier, less, less_equal, minus,
    plus, slash, star,
};
use asserting::prelude::*;

#[test]
fn evaluate_literal_nil() {
    let expr = Expr::from(nil());

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Nil);
}

#[test]
fn evaluate_literal_bool() {
    let expr = Expr::from(literal(true));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_literal_number() {
    let expr = Expr::from(literal(123.456));

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
    let expr = Expr::from(grouping(literal(123.456)));

    let mut interpreter = Interpreter::default();
    let Ok(Value::Number(value)) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(123.456);
}

#[test]
fn evaluate_unary_expr_bang_for_true() {
    let expr = Expr::from(unary(bang((1, 2)), literal(true)));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_unary_expr_bang_for_false() {
    let expr = Expr::from(unary(bang((1, 2)), literal(false)));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_unary_expr_bang_for_number_0() {
    let expr = Expr::from(unary(bang((1, 2)), literal(0.)));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_unary_expr_bang_for_string() {
    let expr = Expr::from(unary(bang((1, 2)), Literal::String("0".into())));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_unary_expr_minus_with_number() {
    let expr = Expr::from(unary(minus((1, 2)), literal(123.456)));

    let mut interpreter = Interpreter::default();
    let Ok(Value::Number(value)) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(-123.456);
}

#[test]
fn evaluate_unary_expr_minus_with_boolean_returns_runtime_error() {
    let expr = Expr::from(unary(minus((1, 2)), literal(true)));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

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

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        minus((1, 2)),
    ));
}

#[test]
fn evaluate_unary_expr_minus_with_nil_returns_runtime_error() {
    let expr = Expr::from(unary(minus((1, 2)), Literal::Nil));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        minus((1, 2)),
    ));
}

#[test]
fn evaluate_unary_expr_with_illegal_operator() {
    let expr = Expr::from(unary(plus((1, 2)), literal(123.456)));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::NotAnUnaryOperator,
        plus((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_minus_with_numbers() {
    let expr = Expr::from(binary(literal(123.456), minus((1, 2)), literal(789.012)));

    let mut interpreter = Interpreter::default();
    let Ok(Value::Number(value)) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(-665.556);
}

#[test]
fn evaluate_binary_expr_minus_with_booleans_returns_runtime_error() {
    let expr = Expr::from(binary(literal(true), minus((1, 2)), literal(false)));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

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

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        minus((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_minus_with_nil_returns_runtime_error() {
    let expr = Expr::from(binary(Literal::Nil, minus((1, 2)), literal(123.456)));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        minus((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_plus_with_numbers() {
    let expr = Expr::from(binary(literal(123.456), plus((1, 2)), literal(789.012)));

    let mut interpreter = Interpreter::default();
    let Ok(Value::Number(value)) = interpreter.evaluate(&expr) else {
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

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

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

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

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

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandsOfDifferentType,
        plus((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_plus_with_booleans_returns_runtime_error() {
    let expr = Expr::from(binary(literal(true), plus((1, 2)), literal(false)));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumberOrString,
        plus((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_plus_with_nil_returns_runtime_error() {
    let expr = Expr::from(binary(Literal::Nil, plus((1, 2)), literal(123.456)));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumberOrString,
        plus((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_star_with_numbers() {
    let expr = Expr::from(binary(literal(-123.456), star((1, 2)), literal(789.012)));

    let mut interpreter = Interpreter::default();
    let Ok(Value::Number(value)) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(-97_408.265_472);
}

#[test]
fn evaluate_binary_expr_star_with_booleans_returns_runtime_error() {
    let expr = Expr::from(binary(literal(true), star((1, 2)), literal(false)));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

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

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        star((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_star_with_nil_returns_runtime_error() {
    let expr = Expr::from(binary(Literal::Nil, star((1, 2)), literal(123.456)));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        star((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_slash_with_numbers() {
    let expr = Expr::from(binary(literal(123.456), slash((1, 2)), literal(789.012)));

    let mut interpreter = Interpreter::default();
    let Ok(Value::Number(value)) = interpreter.evaluate(&expr) else {
        panic!("expected a number");
    };

    assert_that!(value).is_close_to(0.156_469_103_131_511_3);
}

#[test]
fn evaluate_binary_expr_slash_with_booleans_returns_runtime_error() {
    let expr = Expr::from(binary(literal(true), slash((1, 2)), literal(false)));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

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

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::OperandNotANumber,
        slash((1, 2)),
    ));
}

#[test]
fn evaluate_binary_expr_slash_with_nil_returns_runtime_error() {
    let expr = Expr::from(binary(Literal::Nil, slash((1, 2)), literal(123.456)));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

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

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_bangequal_with_booleans() {
    let expr = Expr::from(binary(literal(true), bang_equal((1, 2)), literal(false)));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_bangequal_with_strings() {
    let expr = Expr::from(binary(
        Literal::String("Anna".into()),
        bang_equal((1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_bangequal_with_nils() {
    let expr = Expr::from(binary(Literal::Nil, bang_equal((1, 2)), Literal::Nil));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_equalequal_with_numbers() {
    let expr = Expr::from(binary(
        literal(123.456),
        equal_equal((1, 2)),
        literal(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_equalequal_with_booleans() {
    let expr = Expr::from(binary(literal(true), equal_equal((1, 2)), literal(false)));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_equalequal_with_strings() {
    let expr = Expr::from(binary(
        Literal::String("Anna".into()),
        equal_equal((1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_equalequal_with_nils() {
    let expr = Expr::from(binary(Literal::Nil, equal_equal((1, 2)), Literal::Nil));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greater_with_numbers() {
    let expr = Expr::from(binary(literal(123.456), greater((1, 2)), literal(789.012)));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_greater_with_booleans() {
    let expr = Expr::from(binary(literal(true), greater((1, 2)), literal(false)));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greater_with_strings() {
    let expr = Expr::from(binary(
        Literal::String("Billie".into()),
        greater((1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greater_with_nils() {
    let expr = Expr::from(binary(Literal::Nil, greater((1, 2)), Literal::Nil));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_greaterequal_with_numbers() {
    let expr = Expr::from(binary(
        literal(123.456),
        greater_equal((1, 2)),
        literal(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_greaterequal_with_booleans() {
    let expr = Expr::from(binary(literal(true), greater_equal((1, 2)), literal(true)));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greaterequal_with_strings() {
    let expr = Expr::from(binary(
        Literal::String("Anna".into()),
        greater_equal((1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_greaterequal_with_nils() {
    let expr = Expr::from(binary(Literal::Nil, greater_equal((1, 2)), Literal::Nil));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_less_with_numbers() {
    let expr = Expr::from(binary(literal(123.456), less((1, 2)), literal(789.012)));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_less_with_booleans() {
    let expr = Expr::from(binary(literal(true), less((1, 2)), literal(false)));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_less_with_strings() {
    let expr = Expr::from(binary(
        Literal::String("Anna".into()),
        less((1, 2)),
        Literal::String("Billie".into()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_less_with_nils() {
    let expr = Expr::from(binary(Literal::Nil, less((1, 2)), Literal::Nil));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(false));
}

#[test]
fn evaluate_binary_expr_lessequal_with_numbers() {
    let expr = Expr::from(binary(
        literal(123.456),
        less_equal((1, 2)),
        literal(789.012),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_lessequal_with_booleans() {
    let expr = Expr::from(binary(literal(false), less_equal((1, 2)), literal(true)));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_lessequal_with_strings() {
    let expr = Expr::from(binary(
        Literal::String("Anna".into()),
        less_equal((1, 2)),
        Literal::String("Anna".into()),
    ));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_lessequal_with_nils() {
    let expr = Expr::from(binary(Literal::Nil, less_equal((1, 2)), Literal::Nil));

    let mut interpreter = Interpreter::default();
    let value = interpreter.evaluate(&expr);

    assert_that!(value).ok().is_equal_to(Value::Bool(true));
}

#[test]
fn evaluate_binary_expr_with_illegal_operator() {
    let expr = Expr::from(binary(literal(123.456), bang((1, 2)), literal(789.012)));

    let mut interpreter = Interpreter::default();
    let result = interpreter.evaluate(&expr);

    assert_that!(result).has_error(RuntimeError::new(
        RuntimeErrorCode::NotABinaryOperator,
        bang((1, 2)),
    ));
}

#[test]
fn execute_print_stmt_with_expression() {
    let stmt = Stmt::from(print(binary(literal(84.), plus((1, 2)), literal(2.))));

    let mut interpreter = Interpreter::default();
    let result = interpreter.execute(&stmt);

    assert_that!(result).is_ok();
}

#[test]
fn execute_var_stmt_with_initializer() {
    let stmt = Stmt::from(var(
        identifier("my_var", (4, 6)),
        binary(literal(40.), plus((17, 1)), literal(2.)).expr(),
    ));

    let mut interpreter = Interpreter::default();
    let result = interpreter.execute(&stmt);

    assert_that!(result).is_ok();
    assert_that!(interpreter.environment().get("my_var")).is_equal_to(Ok(&Value::Number(42.)));
}

#[test]
fn execute_var_stmt_without_initializer() {
    let stmt = Stmt::from(var(identifier("foo", (4, 3)), None));

    let mut interpreter = Interpreter::default();
    let result = interpreter.execute(&stmt);

    assert_that!(result).is_ok();
    assert_that!(interpreter.environment().get("foo")).is_equal_to(Ok(&Value::Nil));
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
    let stmt = Stmt::from(print(variable(identifier("foo", (4, 3)))));

    let mut interpreter = Interpreter::default();
    let result = interpreter.execute(&stmt);

    assert_that!(result).err().is_equal_to(RuntimeError::new(
        RuntimeErrorCode::UndefinedVariable("foo".into()),
        identifier("foo", (4, 3)),
    ));
}

#[test]
fn evaluate_assign_expr_stmt_to_existing_variable() {
    let declare_foo = Stmt::from(var(identifier("foo", (4, 3)), literal(123.).expr()));

    let assign_to_foo = Expr::from(assign(identifier("foo", (23, 3)), literal(99.)));

    let mut interpreter = Interpreter::default();
    let declare_result = interpreter.execute(&declare_foo);
    assert_that!(declare_result).is_ok();

    let assign_result = interpreter.evaluate(&assign_to_foo);

    assert_that!(assign_result).is_equal_to(Ok(Value::Number(99.)));
    assert_that!(interpreter.environment().get("foo")).is_equal_to(Ok(&Value::Number(99.)));
}

#[test]
fn evaluate_assign_expr_stmt_to_not_existing_variable() {
    let declare_foo = Stmt::from(var(identifier("a", (4, 1)), literal(123.).expr()));

    let assign_to_foo = Expr::from(assign(identifier("foo", (23, 3)), literal(99.)));

    let mut interpreter = Interpreter::default();
    let declare_result = interpreter.execute(&declare_foo);
    assert_that!(declare_result).is_ok();

    let assign_result = interpreter.evaluate(&assign_to_foo);

    assert_that!(assign_result)
        .err()
        .is_equal_to(RuntimeError::new(
            RuntimeErrorCode::UndefinedVariable("foo".into()),
            identifier("foo", (23, 3)),
        ));
    assert_that!(interpreter.environment().get("foo"))
        .is_equal_to(Err(EnvironmentError::UndefinedVariable("foo".into())));
    assert_that!(interpreter.environment().get("a")).is_equal_to(Ok(&Value::Number(123.)));
}

#[test]
fn execute_block_that_is_empty() {
    let stmt = Stmt::from(block(vec![]));

    let mut interpreter = Interpreter::default();
    let result = interpreter.execute(&stmt);

    assert_that!(result).is_ok();
}

#[test]
fn execute_block_with_var_declarations_and_assignments() {
    let declare_a = Stmt::from(var(identifier("a", (4, 1)), literal(3.).expr()));
    let declare_b = Stmt::from(var(identifier("b", (14, 1)), literal(2.).expr()));
    let assign_b = Expr::from(assign(identifier("b", (24, 1)), literal(7.).expr()));
    let assign_a = Expr::from(assign(identifier("a", (34, 1)), literal(5.).expr()));
    let block = Stmt::from(block(vec![declare_b, assign_b.stmt(), assign_a.stmt()]));

    let mut interpreter = Interpreter::default();
    let result = interpreter.execute(&declare_a);
    assert_that!(result).is_ok();

    let result = interpreter.execute(&block);

    assert_that!(result).is_ok();
    assert_that!(interpreter.environment().get("a")).is_equal_to(Ok(&Value::Number(5.)));
    assert_that!(interpreter.environment().get("b"))
        .err()
        .is_equal_to(EnvironmentError::UndefinedVariable("b".into()));
}

#[test]
fn execute_block_with_var_declarations_and_assignments_and_runtime_error() {
    let declare_a = Stmt::from(var(identifier("a", (4, 1)), literal(3.).expr()));
    let declare_b = Stmt::from(var(identifier("b", (14, 1)), literal(2.).expr()));
    let assign_c = Expr::from(assign(identifier("c", (24, 1)), literal(7.).expr()));
    let assign_a = Expr::from(assign(identifier("a", (34, 1)), literal(5.).expr()));
    let block = Stmt::from(block(vec![declare_b, assign_c.stmt(), assign_a.stmt()]));

    let mut interpreter = Interpreter::default();
    let result = interpreter.execute(&declare_a);
    assert_that!(result).is_ok();

    let result = interpreter.execute(&block);

    assert_that!(result).err().is_equal_to(RuntimeError::new(
        RuntimeErrorCode::UndefinedVariable("c".into()),
        identifier("c", (24, 1)),
    ));
    assert_that!(interpreter.environment().get("a")).is_equal_to(Ok(&Value::Number(3.)));
    assert_that!(interpreter.environment().get("b"))
        .err()
        .is_equal_to(EnvironmentError::UndefinedVariable("b".into()));
}
