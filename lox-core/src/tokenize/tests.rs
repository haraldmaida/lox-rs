use super::*;
use crate::token::{identifier, literal_token, token};
use asserting::prelude::*;

mod lexing_error_code {
    use super::*;

    #[test]
    fn can_be_converted_from_io_error() {
        let code = LexingErrorCode::from(io::Error::from(io::ErrorKind::UnexpectedEof));

        assert_that!(code).is_equal_to(LexingErrorCode::IoError(
            "unexpected end of file".to_string(),
        ));
    }
}

mod lexing_error {
    use super::*;

    #[test]
    fn display_format_io_error() {
        let error = LexingError {
            code: LexingErrorCode::IoError("I/O error".to_string()),
            location: (1, 0).into(),
        };

        assert_that!(error.to_string()).is_equal_to("I/O error");
    }

    #[test]
    fn display_format_unexpected_character() {
        let error = LexingError {
            code: LexingErrorCode::UnexpectedCharacter('§'),
            location: (74, 23).into(),
        };

        assert_that!(error.to_string()).is_equal_to("unexpected character '§'");
    }
}

#[test]
fn can_tokenize_str() {
    let source_code = "()";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::LeftParen, "(", (0, 1))),
        Ok(token(TokenKind::RightParen, ")", (1, 1))),
        Ok(token(TokenKind::EndOfFile, "", (2, 0))),
    ]);
}

#[test]
fn tokenize_empty_source() {
    let source_code = "";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([Ok(token(TokenKind::EndOfFile, "", (0, 0)))]);
}

#[test]
fn tokenize_punctuations() {
    let source_code = "(){},.;";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::LeftParen, "(", (0, 1))),
        Ok(token(TokenKind::RightParen, ")", (1, 1))),
        Ok(token(TokenKind::LeftBrace, "{", (2, 1))),
        Ok(token(TokenKind::RightBrace, "}", (3, 1))),
        Ok(token(TokenKind::Comma, ",", (4, 1))),
        Ok(token(TokenKind::Dot, ".", (5, 1))),
        Ok(token(TokenKind::Semicolon, ";", (6, 1))),
        Ok(token(TokenKind::EndOfFile, "", (7, 0))),
    ]);
}

#[test]
fn tokenize_ignores_whitespace() {
    let source_code = " ( \t\n\r\u{000C} ) { \t\n\r\u{000C} } ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::LeftParen, "(", (1, 1))),
        Ok(token(TokenKind::RightParen, ")", (8, 1))),
        Ok(token(TokenKind::LeftBrace, "{", (10, 1))),
        Ok(token(TokenKind::RightBrace, "}", (17, 1))),
        Ok(token(TokenKind::EndOfFile, "", (19, 0))),
    ]);
}

#[test]
fn tokenize_single_character_operators() {
    let source_code = "- + * / ! = < >";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::Minus, "-", (0, 1))),
        Ok(token(TokenKind::Plus, "+", (2, 1))),
        Ok(token(TokenKind::Star, "*", (4, 1))),
        Ok(token(TokenKind::Slash, "/", (6, 1))),
        Ok(token(TokenKind::Bang, "!", (8, 1))),
        Ok(token(TokenKind::Equal, "=", (10, 1))),
        Ok(token(TokenKind::Less, "<", (12, 1))),
        Ok(token(TokenKind::Greater, ">", (14, 1))),
        Ok(token(TokenKind::EndOfFile, "", (15, 0))),
    ]);
}

#[test]
fn tokenize_two_character_operators() {
    let source_code = "== != <= >=";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::EqualEqual, "==", (0, 2))),
        Ok(token(TokenKind::BangEqual, "!=", (3, 2))),
        Ok(token(TokenKind::LessEqual, "<=", (6, 2))),
        Ok(token(TokenKind::GreaterEqual, ">=", (9, 2))),
        Ok(token(TokenKind::EndOfFile, "", (11, 0))),
    ]);
}

#[test]
fn tokenize_line_comment_not_on_last_line() {
    let source_code = "() // (some comment).\n{}";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::LeftParen, "(", (0, 1))),
        Ok(token(TokenKind::RightParen, ")", (1, 1))),
        Ok(token(TokenKind::LeftBrace, "{", (22, 1))),
        Ok(token(TokenKind::RightBrace, "}", (23, 1))),
        Ok(token(TokenKind::EndOfFile, "", (24, 0))),
    ]);
}

#[test]
fn tokenize_line_comment_at_end_of_file() {
    let source_code = "() // (some comment).";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::LeftParen, "(", (0, 1))),
        Ok(token(TokenKind::RightParen, ")", (1, 1))),
        Ok(token(TokenKind::EndOfFile, "", (21, 0))),
    ]);
}

#[test]
fn tokenize_line_comments_on_two_subsequent_lines() {
    let source_code = "() \n// (first line of comment).\n// (second line of comment).\n{}";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::LeftParen, "(", (0, 1))),
        Ok(token(TokenKind::RightParen, ")", (1, 1))),
        Ok(token(TokenKind::LeftBrace, "{", (61, 1))),
        Ok(token(TokenKind::RightBrace, "}", (62, 1))),
        Ok(token(TokenKind::EndOfFile, "", (63, 0))),
    ]);
}

#[test]
fn tokenize_unexpected_character_at_line_1_char_4() {
    let source_code = "(){§},.;";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::LeftParen, "(", (0, 1))),
        Ok(token(TokenKind::RightParen, ")", (1, 1))),
        Ok(token(TokenKind::LeftBrace, "{", (2, 1))),
        Err(LexingError {
            code: LexingErrorCode::UnexpectedCharacter('§'),
            location: (3, 2).into(),
        }),
        Ok(token(TokenKind::RightBrace, "}", (5, 1))),
        Ok(token(TokenKind::Comma, ",", (6, 1))),
        Ok(token(TokenKind::Dot, ".", (7, 1))),
        Ok(token(TokenKind::Semicolon, ";", (8, 1))),
        Ok(token(TokenKind::EndOfFile, "", (9, 0))),
    ]);
}

#[test]
fn tokenize_unexpected_character_at_line_3_char_1() {
    let source_code = "{\n;\n§}\n";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::LeftBrace, "{", (0, 1))),
        Ok(token(TokenKind::Semicolon, ";", (2, 1))),
        Err(LexingError {
            code: LexingErrorCode::UnexpectedCharacter('§'),
            location: (4, 2).into(),
        }),
        Ok(token(TokenKind::RightBrace, "}", (6, 1))),
        Ok(token(TokenKind::EndOfFile, "", (8, 0))),
    ]);
}

#[test]
fn tokenize_string_literal_empty_string() {
    let source_code = r#""""#;

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(literal_token("", "\"\"", (0, 2))),
        Ok(token(TokenKind::EndOfFile, "", (2, 0))),
    ]);
}

#[test]
fn tokenize_string_literal_with_some_characters() {
    let source_code = r#""some text""#;

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(literal_token("some text", "\"some text\"", (0, 11))),
        Ok(token(TokenKind::EndOfFile, "", (11, 0))),
    ]);
}

#[test]
fn tokenize_string_literal_with_line_breaks() {
    let source_code = "\"first line\nsecond line\r\nthird line\"";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(literal_token(
            "first line\nsecond line\r\nthird line",
            "\"first line\nsecond line\r\nthird line\"",
            (0, 36),
        )),
        Ok(token(TokenKind::EndOfFile, "", (36, 0))),
    ]);
}

#[test]
fn tokenize_string_literal_which_is_unterminated() {
    let source_code = r#""some text"#;

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Err(LexingError {
            code: LexingErrorCode::UnterminatedStringLiteral("\"some text".to_string()),
            location: (0, 10).into(),
        }),
        Ok(token(TokenKind::EndOfFile, "", (10, 0))),
    ]);
}

#[test]
fn tokenize_number_literal_integer_0() {
    let source_code = "0";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(literal_token(0., "0", (0, 1))),
        Ok(token(TokenKind::EndOfFile, "", (1, 0))),
    ]);
}

#[test]
fn tokenize_number_literal_integer_256() {
    let source_code = "256";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(literal_token(256., "256", (0, 3))),
        Ok(token(TokenKind::EndOfFile, "", (3, 0))),
    ]);
}

#[test]
fn tokenize_number_literal_float_1_98() {
    let source_code = "1.98";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(literal_token(1.98, "1.98", (0, 4))),
        Ok(token(TokenKind::EndOfFile, "", (4, 0))),
    ]);
}

#[test]
fn tokenize_list_of_number_literal_float_1_98() {
    let source_code = "811.2344, 5.0, 0.67";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(literal_token(811.2344, "811.2344", (0, 8))),
        Ok(token(TokenKind::Comma, ",", (8, 1))),
        Ok(literal_token(5.0, "5.0", (10, 3))),
        Ok(token(TokenKind::Comma, ",", (13, 1))),
        Ok(literal_token(0.67, "0.67", (15, 4))),
        Ok(token(TokenKind::EndOfFile, "", (19, 0))),
    ]);
}

#[test]
fn tokenize_number_literal_with_trailing_dot() {
    let source_code = "400,\n4.,\n655";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(literal_token(400., "400", (0, 3))),
        Ok(token(TokenKind::Comma, ",", (3, 1))),
        Ok(literal_token(4., "4.", (5, 2))),
        Ok(token(TokenKind::Comma, ",", (7, 1))),
        Ok(literal_token(655., "655", (9, 3))),
        Ok(token(TokenKind::EndOfFile, "", (12, 0))),
    ]);
}

#[test]
fn tokenize_number_literal_with_leading_dot() {
    let source_code = "400 .655";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(literal_token(400., "400", (0, 3))),
        Ok(token(TokenKind::Dot, ".", (4, 1))),
        Ok(literal_token(655., "655", (5, 3))),
        Ok(token(TokenKind::EndOfFile, "", (8, 0))),
    ]);
}

#[test]
fn tokenize_number_literal_with_two_dots() {
    let source_code = "123.456.789";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(literal_token(123.456, "123.456", (0, 7))),
        Ok(token(TokenKind::Dot, ".", (7, 1))),
        Ok(literal_token(789., "789", (8, 3))),
        Ok(token(TokenKind::EndOfFile, "", (11, 0))),
    ]);
}

#[test]
fn tokenize_number_literal_with_trailing_dot_at_end_of_file() {
    let source_code = " 777.";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(literal_token(777., "777.", (1, 4))),
        Ok(token(TokenKind::EndOfFile, "", (5, 0))),
    ]);
}

#[test]
fn tokenize_identifier_letters_only() {
    let source_code = "someIdentifier";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(identifier("someIdentifier", (0, 14))),
        Ok(token(TokenKind::EndOfFile, "", (14, 0))),
    ]);
}

#[test]
fn tokenize_identifier_alphanumeric() {
    let source_code = "club42";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(identifier("club42", (0, 6))),
        Ok(token(TokenKind::EndOfFile, "", (6, 0))),
    ]);
}

#[test]
fn tokenize_identifier_starting_with_underscore() {
    let source_code = "_identifierWithUnderscore";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(identifier("_identifierWithUnderscore", (0, 25))),
        Ok(token(TokenKind::EndOfFile, "", (25, 0))),
    ]);
}

#[test]
fn tokenize_identifier_and_semicolon() {
    let source_code = "number2add;";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(identifier("number2add", (0, 10))),
        Ok(token(TokenKind::Semicolon, ";", (10, 1))),
        Ok(token(TokenKind::EndOfFile, "", (11, 0))),
    ]);
}

#[test]
fn tokenize_boolean_true() {
    let source_code = " true ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::True, "true", (1, 4))),
        Ok(token(TokenKind::EndOfFile, "", (6, 0))),
    ]);
}

#[test]
fn tokenize_boolean_false() {
    let source_code = " false; ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::False, "false", (1, 5))),
        Ok(token(TokenKind::Semicolon, ";", (6, 1))),
        Ok(token(TokenKind::EndOfFile, "", (8, 0))),
    ]);
}

#[test]
fn tokenize_keyword_and() {
    let source_code = " and ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::And, "and", (1, 3))),
        Ok(token(TokenKind::EndOfFile, "", (5, 0))),
    ]);
}

#[test]
fn tokenize_keyword_or() {
    let source_code = " or ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::Or, "or", (1, 2))),
        Ok(token(TokenKind::EndOfFile, "", (4, 0))),
    ]);
}

#[test]
fn tokenize_keyword_class() {
    let source_code = " class Foo {} ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::Class, "class", (1, 5))),
        Ok(identifier("Foo", (7, 3))),
        Ok(token(TokenKind::LeftBrace, "{", (11, 1))),
        Ok(token(TokenKind::RightBrace, "}", (12, 1))),
        Ok(token(TokenKind::EndOfFile, "", (14, 0))),
    ]);
}

#[test]
fn tokenize_keyword_fun() {
    let source_code = " fun foo() {} ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::Fun, "fun", (1, 3))),
        Ok(identifier("foo", (5, 3))),
        Ok(token(TokenKind::LeftParen, "(", (8, 1))),
        Ok(token(TokenKind::RightParen, ")", (9, 1))),
        Ok(token(TokenKind::LeftBrace, "{", (11, 1))),
        Ok(token(TokenKind::RightBrace, "}", (12, 1))),
        Ok(token(TokenKind::EndOfFile, "", (14, 0))),
    ]);
}

#[test]
fn tokenize_keyword_super() {
    let source_code = " super.method(); ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::Super, "super", (1, 5))),
        Ok(token(TokenKind::Dot, ".", (6, 1))),
        Ok(identifier("method", (7, 6))),
        Ok(token(TokenKind::LeftParen, "(", (13, 1))),
        Ok(token(TokenKind::RightParen, ")", (14, 1))),
        Ok(token(TokenKind::Semicolon, ";", (15, 1))),
        Ok(token(TokenKind::EndOfFile, "", (17, 0))),
    ]);
}

#[test]
fn tokenize_keyword_this() {
    let source_code = " this.name = \"John\"; ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::This, "this", (1, 4))),
        Ok(token(TokenKind::Dot, ".", (5, 1))),
        Ok(identifier("name", (6, 4))),
        Ok(token(TokenKind::Equal, "=", (11, 1))),
        Ok(literal_token("John", "\"John\"", (13, 6))),
        Ok(token(TokenKind::Semicolon, ";", (19, 1))),
        Ok(token(TokenKind::EndOfFile, "", (21, 0))),
    ]);
}

#[test]
fn tokenize_keyword_var() {
    let source_code = " var x = 10; ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::Var, "var", (1, 3))),
        Ok(identifier("x", (5, 1))),
        Ok(token(TokenKind::Equal, "=", (7, 1))),
        Ok(literal_token(10., "10", (9, 2))),
        Ok(token(TokenKind::Semicolon, ";", (11, 1))),
        Ok(token(TokenKind::EndOfFile, "", (13, 0))),
    ]);
}

#[test]
fn tokenize_keyword_nil() {
    let source_code = " nil ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::Nil, "nil", (1, 3))),
        Ok(token(TokenKind::EndOfFile, "", (5, 0))),
    ]);
}

#[test]
fn tokenize_keyword_return() {
    let source_code = " return 42; ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::Return, "return", (1, 6))),
        Ok(literal_token(42., "42", (8, 2))),
        Ok(token(TokenKind::Semicolon, ";", (10, 1))),
        Ok(token(TokenKind::EndOfFile, "", (12, 0))),
    ]);
}

#[test]
fn tokenize_keywords_if_else() {
    let source_code = " if (x == 99) { } else { } ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::If, "if", (1, 2))),
        Ok(token(TokenKind::LeftParen, "(", (4, 1))),
        Ok(identifier("x", (5, 1))),
        Ok(token(TokenKind::EqualEqual, "==", (7, 2))),
        Ok(literal_token(99., "99", (10, 2))),
        Ok(token(TokenKind::RightParen, ")", (12, 1))),
        Ok(token(TokenKind::LeftBrace, "{", (14, 1))),
        Ok(token(TokenKind::RightBrace, "}", (16, 1))),
        Ok(token(TokenKind::Else, "else", (18, 4))),
        Ok(token(TokenKind::LeftBrace, "{", (23, 1))),
        Ok(token(TokenKind::RightBrace, "}", (25, 1))),
        Ok(token(TokenKind::EndOfFile, "", (27, 0))),
    ]);
}

#[test]
fn tokenize_keyword_for() {
    let source_code = " for (;;) { } ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::For, "for", (1, 3))),
        Ok(token(TokenKind::LeftParen, "(", (5, 1))),
        Ok(token(TokenKind::Semicolon, ";", (6, 1))),
        Ok(token(TokenKind::Semicolon, ";", (7, 1))),
        Ok(token(TokenKind::RightParen, ")", (8, 1))),
        Ok(token(TokenKind::LeftBrace, "{", (10, 1))),
        Ok(token(TokenKind::RightBrace, "}", (12, 1))),
        Ok(token(TokenKind::EndOfFile, "", (14, 0))),
    ]);
}

#[test]
fn tokenize_keyword_while() {
    let source_code = " while (x > 0) { } ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::While, "while", (1, 5))),
        Ok(token(TokenKind::LeftParen, "(", (7, 1))),
        Ok(identifier("x", (8, 1))),
        Ok(token(TokenKind::Greater, ">", (10, 1))),
        Ok(literal_token(0., "0", (12, 1))),
        Ok(token(TokenKind::RightParen, ")", (13, 1))),
        Ok(token(TokenKind::LeftBrace, "{", (15, 1))),
        Ok(token(TokenKind::RightBrace, "}", (17, 1))),
        Ok(token(TokenKind::EndOfFile, "", (19, 0))),
    ]);
}

#[test]
fn tokenize_keyword_print() {
    let source_code = " print(\"Hello, world!\"); ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::Print, "print", (1, 5))),
        Ok(token(TokenKind::LeftParen, "(", (6, 1))),
        Ok(literal_token("Hello, world!", "\"Hello, world!\"", (7, 15))),
        Ok(token(TokenKind::RightParen, ")", (22, 1))),
        Ok(token(TokenKind::Semicolon, ";", (23, 1))),
        Ok(token(TokenKind::EndOfFile, "", (25, 0))),
    ]);
}

#[test]
fn tokenize_bang_identifier() {
    let source_code = "!a";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(token(TokenKind::Bang, "!", (0, 1))),
        Ok(identifier("a", (1, 1))),
        Ok(token(TokenKind::EndOfFile, "", (2, 0))),
    ]);
}

#[test]
fn tokenize_equal_string_literal() {
    let source_code = "a=\"Jane\";";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(identifier("a", (0, 1))),
        Ok(token(TokenKind::Equal, "=", (1, 1))),
        Ok(literal_token("Jane", "\"Jane\"", (2, 6))),
        Ok(token(TokenKind::Semicolon, ";", (8, 1))),
        Ok(token(TokenKind::EndOfFile, "", (9, 0))),
    ]);
}

#[test]
fn tokenize_greater_number_literal() {
    let source_code = "a>0.5";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(identifier("a", (0, 1))),
        Ok(token(TokenKind::Greater, ">", (1, 1))),
        Ok(literal_token(0.5, "0.5", (2, 3))),
        Ok(token(TokenKind::EndOfFile, "", (5, 0))),
    ]);
}

#[test]
fn tokenize_less_number_literal() {
    let source_code = "a<18";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(identifier("a", (0, 1))),
        Ok(token(TokenKind::Less, "<", (1, 1))),
        Ok(literal_token(18., "18", (2, 2))),
        Ok(token(TokenKind::EndOfFile, "", (4, 0))),
    ]);
}

#[test]
fn tokenize_slash_number_literal() {
    let source_code = "a/2.5";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(identifier("a", (0, 1))),
        Ok(token(TokenKind::Slash, "/", (1, 1))),
        Ok(literal_token(2.5, "2.5", (2, 3))),
        Ok(token(TokenKind::EndOfFile, "", (5, 0))),
    ]);
}

#[test]
fn tokenize_dot_after_integer_literal() {
    let source_code = "1.neg()";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(literal_token(1., "1", (0, 1))),
        Ok(token(TokenKind::Dot, ".", (1, 1))),
        Ok(identifier("neg", (2, 3))),
        Ok(token(TokenKind::LeftParen, "(", (5, 1))),
        Ok(token(TokenKind::RightParen, ")", (6, 1))),
        Ok(token(TokenKind::EndOfFile, "", (7, 0))),
    ]);
}
