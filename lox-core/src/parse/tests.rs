use super::*;
use crate::expr::{ExprExt, assign, binary, grouping, literal, nil, unary, variable};
use crate::program::program;
use crate::stmt::{IfExt, StmtExt, block, if_, print, stmt, var};
use crate::token::{
    bang, bang_equal, equal_equal, greater, greater_equal, identifier, less, less_equal, minus,
    plus, slash, star,
};
use crate::tokenize::Tokenize;
use asserting::prelude::*;

#[test]
fn parse_empty_source_code() {
    let source_code = "";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([]));
}

#[test]
fn parse_equality_expression_literal_equal_literal() {
    let source_code = "1 == 1";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(binary(literal(1.), equal_equal((2, 2)), literal(1.)).expr());
}

#[test]
fn parse_equality_expression_literal_not_equal_literal() {
    let source_code = "10 != 1";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(binary(literal(10.), bang_equal((3, 2)), literal(1.)).expr());
}

#[test]
fn parse_comparison_expression_literal_greater_literal() {
    let source_code = "22 > 1";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(binary(literal(22.), greater((3, 1)), literal(1.)).expr());
}

#[test]
fn parse_comparison_expression_literal_greater_equal_literal() {
    let source_code = "2 >= 1";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(binary(literal(2.), greater_equal((2, 2)), literal(1.)).expr());
}

#[test]
fn parse_comparison_expression_literal_less_literal() {
    let source_code = "2 < 11";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(binary(literal(2.), less((2, 1)), literal(11.)).expr());
}

#[test]
fn parse_comparison_expression_literal_less_equal_literal() {
    let source_code = "22 <= 111";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(binary(literal(22.), less_equal((3, 2)), literal(111.)).expr());
}

#[test]
fn parse_term_expression_literal_minus_literal() {
    let source_code = "2 - 1";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(binary(literal(2.), minus((2, 1)), literal(1.)).expr());
}

#[test]
fn parse_term_expression_literal_plus_literal() {
    let source_code = "22 + 11";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(binary(literal(22.), plus((3, 1)), literal(11.)).expr());
}

#[test]
fn parse_term_expression_literal_multiplied_by_literal() {
    let source_code = "22 * 11";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(binary(literal(22.), star((3, 1)), literal(11.)).expr());
}

#[test]
fn parse_term_expression_literal_divided_by_literal() {
    let source_code = "6 / 2";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(binary(literal(6.), slash((2, 1)), literal(2.)).expr());
}

#[test]
fn parse_unary_expression_not_literal() {
    let source_code = "!true";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(unary(bang((0, 1)), literal(true)).expr());
}

#[test]
fn parse_unary_expression_negate_literal() {
    let source_code = "-42";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(unary(minus((0, 1)), literal(42.)).expr());
}

#[test]
fn parse_primary_literal_nil() {
    let source_code = "nil";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result).ok().is_equal_to(nil().expr());
}

#[test]
fn parse_primary_literal_false() {
    let source_code = "false";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result).ok().is_equal_to(literal(false).expr());
}

#[test]
fn parse_primary_literal_true() {
    let source_code = "true";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Literal::Bool(true)));
}

#[test]
fn parse_primary_literal_number() {
    let source_code = "123.456";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Literal::Number(123.456)));
}

#[test]
fn parse_primary_literal_string() {
    let source_code = "\"Hello, World!\"";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result)
        .ok()
        .is_equal_to(Expr::from(Literal::String("Hello, World!".into())));
}

#[test]
fn parse_primary_literal_parens() {
    let source_code = "(5 - (3 - 1)) + -1";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result).ok().is_equal_to(
        binary(
            grouping(binary(
                literal(5.),
                minus((3, 1)),
                grouping(binary(literal(3.), minus((8, 1)), literal(1.))),
            )),
            plus((14, 1)),
            unary(minus((16, 1)), literal(1.)),
        )
        .expr(),
    );
}

#[test]
fn parse_expression_statement_missing_semicolon() {
    let source_code = "84 / 2";

    let result = source_code.tokenize().parse();

    assert_that!(result).err().contains_exactly([SyntaxError {
        code: SyntaxErrorCode::MissingToken(TokenKind::Semicolon),
        location: (6, 0).into(),
    }]);
}

#[test]
fn parse_expression_statement() {
    let source_code = "84 / 2;";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([stmt(
        binary(literal(84.), slash((3, 1)), literal(2.)).expr(),
    )]));
}

#[test]
fn parse_print_string_literal() {
    let source_code = "print \"Hello, World!\";";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(program([print(literal("Hello, World!")).stmt()]));
}

#[test]
fn parse_declaration_of_variable() {
    let source_code = "var x = 42;";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([var(
        identifier("x", (4, 1)),
        literal(42.).expr(),
    )
    .stmt()]));
}

#[test]
fn parse_print_variable() {
    let source_code = "print foo;";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(program([print(variable(identifier("foo", (6, 3)))).stmt()]));
}

#[test]
fn parse_assignment_to_variable() {
    let source_code = "var foo = \"before\";\nfoo = \"after\";";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([
        var(identifier("foo", (4, 3)), literal("before").expr()).stmt(),
        assign(identifier("foo", (20, 3)), literal("after"))
            .expr()
            .stmt(),
    ]));
}

#[test]
fn parse_assignment_to_invalid_assignment_target() {
    let source_code = "var foo = \"before\";\na + b = \"after\";";

    let result = source_code.tokenize().parse();

    assert_that!(result).err().contains_exactly([SyntaxError {
        code: SyntaxErrorCode::InvalidAssignmentTarget,
        location: (35, 1).into(),
    }]);
}

#[test]
fn parse_block_that_is_empty() {
    let source_code = "{}";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(program([block([]).stmt()]));
}

#[test]
fn parse_block_containing_one_statement() {
    let source_code = "{ print \"Hello, World!\"; }";

    let result = source_code.tokenize().parse();

    assert_that!(result)
        .ok()
        .is_equal_to(program([
            block([print(literal("Hello, World!")).stmt()]).stmt()
        ]));
}

#[test]
fn parse_block_containing_multiple_statements() {
    let source_code = "{ var x = 1;\nx = x * 2;\nprint x; }";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([block([
        var(identifier("x", (6, 1)), literal(1.).expr()).stmt(),
        assign(
            identifier("x", (13, 1)),
            binary(
                variable(identifier("x", (17, 1))),
                star((19, 1)),
                literal(2.).expr(),
            ),
        )
        .expr()
        .stmt(),
        print(variable(identifier("x", (30, 1)))).stmt(),
    ])
    .stmt()]));
}

#[test]
fn parse_if_stmt_single_then_stmt_no_else_branch() {
    let source_code = "if (true) print \"Hello, World!\";";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([if_(
        literal(true),
        print(literal("Hello, World!")).stmt(),
    )
    .stmt()]));
}

#[test]
fn parse_if_stmt_single_then_stmt_and_single_else_stmt() {
    let source_code = "if (42 > 41) print \"Hello, World!\"; else print \"Goodbye, World!\";";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([if_(
        binary(literal(42.), greater((7, 1)), literal(41.)),
        print(literal("Hello, World!")).stmt(),
    )
    .else_(print(literal("Goodbye, World!")).stmt())
    .stmt()]));
}

#[test]
fn parse_if_stmt_multiple_then_stmts_no_else_branch() {
    let source_code = "if (true) { x = 4 + 3;\nprint x; }";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([if_(
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
    )
    .stmt()]));
}

#[test]
fn parse_if_stmt_multiple_then_stmts_and_multiple_else_stmts() {
    let source_code = "if (x < 0) { x = x - 3;\nprint x; } else { x = x + 3;\nprint x; }";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([if_(
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
    ]))
    .stmt()]));
}

#[test]
fn parse_if_without_else_branch_and_nested_if_with_else_branch() {
    let source_code = "if (x) if (y) print \"x and y\"; else print \"x only\";";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([if_(
        variable(identifier("x", (4, 1))),
        if_(
            variable(identifier("y", (11, 1))),
            print(literal("x and y")),
        )
        .else_(print(literal("x only"))),
    )
    .stmt()]));
}
