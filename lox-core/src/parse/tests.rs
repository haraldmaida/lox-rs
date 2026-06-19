use super::*;
use crate::expr::{
    ExprExt, assign, binary, call, grouping, literal, logical, nil, unary, variable,
};
use crate::program::program;
use crate::stmt::{IfExt, StmtExt, block, if_, print, stmt, var, while_};
use crate::token::{
    and, bang, bang_equal, equal_equal, greater, greater_equal, identifier, less, less_equal,
    minus, or, plus, right_paren, slash, star,
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

#[test]
fn parse_logical_and() {
    let source_code = "x >= 1 and x < 10";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result).ok().is_equal_to(
        logical(
            binary(
                variable(identifier("x", (0, 1))),
                greater_equal((2, 2)),
                literal(1.),
            ),
            and((7, 3)),
            binary(
                variable(identifier("x", (11, 1))),
                less((13, 1)),
                literal(10.),
            ),
        )
        .expr(),
    );
}

#[test]
fn parse_logical_or() {
    let source_code = "x < 1 or x >= 10";

    let result = source_code.tokenize().parse_expr();

    assert_that!(result).ok().is_equal_to(
        logical(
            binary(variable(identifier("x", (0, 1))), less((2, 1)), literal(1.)),
            or((6, 2)),
            binary(
                variable(identifier("x", (9, 1))),
                greater_equal((11, 2)),
                literal(10.),
            ),
        )
        .expr(),
    );
}

#[test]
fn parse_while_loop_with_single_statement() {
    let source_code = "while (x < 10) x = x + 1;";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([while_(
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
    )
    .stmt()]));
}

#[test]
fn parse_while_loop_with_multiple_statements() {
    let source_code = "while (x < 10) { x = x + 1; print x; }";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([while_(
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
    )
    .stmt()]));
}

#[test]
fn parse_for_loop_with_one_statement() {
    let source_code = "for (var i = 0; i < 10; i = i + 1) print i;";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([block([
        var(identifier("i", (9, 1)), literal(0.).expr()).stmt(),
        while_(
            binary(
                variable(identifier("i", (16, 1))),
                less((18, 1)),
                literal(10.),
            ),
            block([
                print(variable(identifier("i", (41, 1)))).stmt(),
                assign(
                    identifier("i", (24, 1)),
                    binary(
                        variable(identifier("i", (28, 1))),
                        plus((30, 1)),
                        literal(1.),
                    ),
                )
                .expr()
                .stmt(),
            ]),
        )
        .stmt(),
    ])
    .stmt()]));
}

#[test]
fn parse_for_loop_with_multiple_statements() {
    let source_code = "for (var i = 0; i < 10; i = i + 1) { var x = i * 2; print x; }";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([block([
        var(identifier("i", (9, 1)), literal(0.).expr()).stmt(),
        while_(
            binary(
                variable(identifier("i", (16, 1))),
                less((18, 1)),
                literal(10.),
            ),
            block([
                block([
                    var(
                        identifier("x", (41, 1)),
                        binary(
                            variable(identifier("i", (45, 1))),
                            star((47, 1)),
                            literal(2.),
                        )
                        .expr(),
                    )
                    .stmt(),
                    print(variable(identifier("x", (58, 1)))).stmt(),
                ])
                .stmt(),
                assign(
                    identifier("i", (24, 1)),
                    binary(
                        variable(identifier("i", (28, 1))),
                        plus((30, 1)),
                        literal(1.),
                    ),
                )
                .expr()
                .stmt(),
            ]),
        )
        .stmt(),
    ])
    .stmt()]));
}

#[test]
fn parse_for_loop_with_no_parts() {
    let source_code = "for (;;) print \"Hello, World!\";";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([while_(
        literal(true),
        print(literal("Hello, World!")).stmt(),
    )
    .stmt()]));
}

#[test]
fn parse_for_loop_without_initializer() {
    let source_code = "var i = 5; for (; i < 10; i = i + 1) print i;";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([
        var(identifier("i", (4, 1)), literal(5.).expr()).stmt(),
        while_(
            binary(
                variable(identifier("i", (18, 1))),
                less((20, 1)),
                literal(10.),
            ),
            block([
                print(variable(identifier("i", (43, 1)))).stmt(),
                assign(
                    identifier("i", (26, 1)),
                    binary(
                        variable(identifier("i", (30, 1))),
                        plus((32, 1)),
                        literal(1.),
                    ),
                )
                .expr()
                .stmt(),
            ])
            .stmt(),
        )
        .stmt(),
    ]));
}

#[test]
fn parse_for_loop_without_condition() {
    let source_code = "for (var i = 0; ; i = i + 1) print i;";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([block([
        var(identifier("i", (9, 1)), literal(0.).expr()).stmt(),
        while_(
            literal(true),
            block([
                print(variable(identifier("i", (35, 1)))).stmt(),
                assign(
                    identifier("i", (18, 1)),
                    binary(
                        variable(identifier("i", (22, 1))),
                        plus((24, 1)),
                        literal(1.),
                    ),
                )
                .expr()
                .stmt(),
            ]),
        )
        .stmt(),
    ])
    .stmt()]));
}

#[test]
fn parse_for_loop_without_increment() {
    let source_code = "for (var i = 0; i < 10;) print i;";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([block([
        var(identifier("i", (9, 1)), literal(0.).expr()).stmt(),
        while_(
            binary(
                variable(identifier("i", (16, 1))),
                less((18, 1)),
                literal(10.),
            ),
            print(variable(identifier("i", (31, 1)))).stmt(),
        )
        .stmt(),
    ])
    .stmt()]));
}

#[test]
fn parse_function_call_with_no_arguments() {
    let source_code = "foo();";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([call(
        variable(identifier("foo", (0, 3))),
        right_paren((4, 1)),
        [],
    )
    .expr()
    .stmt()]));
}

#[test]
fn parse_function_call_with_one_arguments() {
    let source_code = "foo(42);";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([call(
        variable(identifier("foo", (0, 3))),
        right_paren((6, 1)),
        [literal(42.).expr()],
    )
    .expr()
    .stmt()]));
}

#[test]
fn parse_function_call_with_two_arguments() {
    let source_code = "foo(42, \"Hello, World!\");";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([call(
        variable(identifier("foo", (0, 3))),
        right_paren((23, 1)),
        [literal(42.).expr(), literal("Hello, World!").expr()],
    )
    .expr()
    .stmt()]));
}

#[test]
fn parse_function_call_with_three_arguments() {
    let source_code = "foo(x, 42, \"Hello, World!\");";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([call(
        variable(identifier("foo", (0, 3))),
        right_paren((26, 1)),
        [
            variable(identifier("x", (4, 1))).expr(),
            literal(42.).expr(),
            literal("Hello, World!").expr(),
        ],
    )
    .expr()
    .stmt()]));
}

#[test]
fn parse_function_call_with_grouping_in_an_argument() {
    let source_code = "foo(x, (42 - 3));";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([call(
        variable(identifier("foo", (0, 3))),
        right_paren((15, 1)),
        [
            variable(identifier("x", (4, 1))).expr(),
            grouping(binary(literal(42.), minus((11, 1)), literal(3.))).expr(),
        ],
    )
    .expr()
    .stmt()]));
}

#[test]
fn parse_function_call_on_return_value_of_a_function_call() {
    let source_code = "foo(a)()(b);";

    let result = source_code.tokenize().parse();

    assert_that!(result).ok().is_equal_to(program([call(
        call(
            call(
                variable(identifier("foo", (0, 3))),
                right_paren((5, 1)),
                [variable(identifier("a", (4, 1))).expr()],
            ),
            right_paren((7, 1)),
            [],
        ),
        right_paren((10, 1)),
        [variable(identifier("b", (9, 1))).expr()],
    )
    .expr()
    .stmt()]));
}

fn generate_function_call_source_code(fun_name: &str, num_args: usize) -> String {
    format!(
        "{fun_name}(1{});",
        (2..=num_args)
            .map(|n| format!(", {n}"))
            .fold(String::new(), |acc, n| acc + &n)
    )
}

#[test]
fn parse_function_call_with_255_arguments() {
    // foo(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255);
    let source_code = generate_function_call_source_code("foo", 255);

    let result = source_code.tokenize().parse();

    assert_that!(result).is_ok();
}

#[test]
fn parse_function_call_with_256_arguments() {
    // foo(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256);
    let source_code = generate_function_call_source_code("foo", 256);

    let result = source_code.tokenize().parse();

    assert_that!(result).err().is_equal_to([SyntaxError {
        code: SyntaxErrorCode::TooManyCallArguments(256, 255),
        location: (1174, 1).into(),
    }]);
}
