use super::{Location, *};
use asserting::prelude::*;
use std::io::{BufReader, Cursor};

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
            location: Location { line: 1, char: 0 },
        };

        assert_that!(error.to_string()).is_equal_to("I/O error at 1:0");
    }

    #[test]
    fn display_format_unexpected_character() {
        let error = LexingError {
            code: LexingErrorCode::UnexpectedCharacter('§'),
            location: Location { line: 74, char: 23 },
        };

        assert_that!(error.to_string()).is_equal_to("unexpected character '§' at 74:23");
    }
}

#[test]
fn can_tokenize_str() {
    let mut source_code = "()";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 2))),
    ]);
}

#[test]
fn can_tokenize_from_buf_reader() {
    let mut source_code = BufReader::new(Cursor::new("()"));

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 2))),
    ]);
}

#[test]
fn tokenize_empty_source() {
    let mut source_code = "";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([Ok(Token::new_nonliteral(
        TokenKind::EndOfFile,
        "",
        (1, 0),
    ))]);
}

#[test]
fn tokenize_punctuations() {
    let mut source_code = "(){},.;";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::LeftBrace, "{", (1, 3))),
        Ok(Token::new_nonliteral(TokenKind::RightBrace, "}", (1, 4))),
        Ok(Token::new_nonliteral(TokenKind::Comma, ",", (1, 5))),
        Ok(Token::new_nonliteral(TokenKind::Dot, ".", (1, 6))),
        Ok(Token::new_nonliteral(TokenKind::Semicolon, ";", (1, 7))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 7))),
    ]);
}

#[test]
fn tokenize_ignores_whitespace() {
    let mut source_code = " ( \t\n\r\u{000C} ) { \t\n\r\u{000C} } ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (2, 4))),
        Ok(Token::new_nonliteral(TokenKind::LeftBrace, "{", (2, 6))),
        Ok(Token::new_nonliteral(TokenKind::RightBrace, "}", (3, 4))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (3, 5))),
    ]);
}

#[test]
fn tokenize_single_character_operators() {
    let mut source_code = "- + * / ! = < >";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::Minus, "-", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::Plus, "+", (1, 3))),
        Ok(Token::new_nonliteral(TokenKind::Star, "*", (1, 5))),
        Ok(Token::new_nonliteral(TokenKind::Slash, "/", (1, 7))),
        Ok(Token::new_nonliteral(TokenKind::Bang, "!", (1, 9))),
        Ok(Token::new_nonliteral(TokenKind::Equal, "=", (1, 11))),
        Ok(Token::new_nonliteral(TokenKind::Less, "<", (1, 13))),
        Ok(Token::new_nonliteral(TokenKind::Greater, ">", (1, 15))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 15))),
    ]);
}

#[test]
fn tokenize_two_character_operators() {
    let mut source_code = "== != <= >=";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::EqualEqual, "==", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::BangEqual, "!=", (1, 4))),
        Ok(Token::new_nonliteral(TokenKind::LessEqual, "<=", (1, 7))),
        Ok(Token::new_nonliteral(
            TokenKind::GreaterEqual,
            ">=",
            (1, 10),
        )),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 11))),
    ]);
}

#[test]
fn tokenize_line_comment_not_on_last_line() {
    let mut source_code = "() // (some comment).\n{}";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::LeftBrace, "{", (2, 1))),
        Ok(Token::new_nonliteral(TokenKind::RightBrace, "}", (2, 2))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (2, 2))),
    ]);
}

#[test]
fn tokenize_line_comment_at_end_of_file() {
    let mut source_code = "() // (some comment).";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 21))),
    ]);
}

#[test]
fn tokenize_line_comments_on_two_subsequent_lines() {
    let mut source_code = "() \n// (first line of comment).\n// (second line of comment).\n{}";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::LeftBrace, "{", (4, 1))),
        Ok(Token::new_nonliteral(TokenKind::RightBrace, "}", (4, 2))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (4, 2))),
    ]);
}

#[test]
fn tokenize_unexpected_character_at_line_1_char_4() {
    let mut source_code = "(){§},.;";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::LeftBrace, "{", (1, 3))),
        Err(LexingError {
            code: LexingErrorCode::UnexpectedCharacter('§'),
            location: Location { line: 1, char: 4 },
        }),
        Ok(Token::new_nonliteral(TokenKind::RightBrace, "}", (1, 5))),
        Ok(Token::new_nonliteral(TokenKind::Comma, ",", (1, 6))),
        Ok(Token::new_nonliteral(TokenKind::Dot, ".", (1, 7))),
        Ok(Token::new_nonliteral(TokenKind::Semicolon, ";", (1, 8))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 8))),
    ]);
}

#[test]
fn tokenize_unexpected_character_at_line_3_char_1() {
    let mut source_code = "{\n;\n§}\n";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::LeftBrace, "{", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::Semicolon, ";", (2, 1))),
        Err(LexingError {
            code: LexingErrorCode::UnexpectedCharacter('§'),
            location: Location { line: 3, char: 1 },
        }),
        Ok(Token::new_nonliteral(TokenKind::RightBrace, "}", (3, 2))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (4, 0))),
    ]);
}

#[test]
fn tokenize_string_literal_empty_string() {
    let mut source_code = r#""""#;

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_literal("", "\"\"", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 2))),
    ]);
}

#[test]
fn tokenize_string_literal_with_some_characters() {
    let mut source_code = r#""some text""#;

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_literal("some text", "\"some text\"", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 11))),
    ]);
}

#[test]
fn tokenize_string_literal_with_line_breaks() {
    let mut source_code = "\"first line\nsecond line\r\nthird line\"";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_literal(
            "first line\nsecond line\r\nthird line",
            "\"first line\nsecond line\r\nthird line\"",
            (1, 1),
        )),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (3, 11))),
    ]);
}

#[test]
fn tokenize_string_literal_which_is_unterminated() {
    let mut source_code = r#""some text"#;

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Err(LexingError {
            code: LexingErrorCode::UnterminatedStringLiteral("\"some text".to_string()),
            location: Location { line: 1, char: 1 },
        }),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 10))),
    ]);
}

#[test]
fn tokenize_number_literal_integer_0() {
    let mut source_code = "0";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_literal(0., "0", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 1))),
    ]);
}

#[test]
fn tokenize_number_literal_integer_256() {
    let mut source_code = "256";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_literal(256., "256", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 3))),
    ]);
}

#[test]
fn tokenize_number_literal_float_1_98() {
    let mut source_code = "1.98";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_literal(1.98, "1.98", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 4))),
    ]);
}

#[test]
fn tokenize_list_of_number_literal_float_1_98() {
    let mut source_code = "811.2344, 5.0, 0.67";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_literal(811.2344, "811.2344", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::Comma, ",", (1, 9))),
        Ok(Token::new_literal(5.0, "5.0", (1, 11))),
        Ok(Token::new_nonliteral(TokenKind::Comma, ",", (1, 14))),
        Ok(Token::new_literal(0.67, "0.67", (1, 16))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 19))),
    ]);
}

#[test]
fn tokenize_number_literal_with_trailing_dot() {
    let mut source_code = "400,\n4.,\n655";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_literal(400., "400", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::Comma, ",", (1, 4))),
        Ok(Token::new_literal(4., "4.", (2, 1))),
        Ok(Token::new_nonliteral(TokenKind::Comma, ",", (2, 3))),
        Ok(Token::new_literal(655., "655", (3, 1))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (3, 3))),
    ]);
}

#[test]
fn tokenize_number_literal_with_leading_dot() {
    let mut source_code = "400 .655";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_literal(400., "400", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::Dot, ".", (1, 5))),
        Ok(Token::new_literal(655., "655", (1, 6))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 8))),
    ]);
}

#[test]
fn tokenize_number_literal_with_two_dots() {
    let mut source_code = "123.456.789";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_literal(123.456, "123.456", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::Dot, ".", (1, 8))),
        Ok(Token::new_literal(789., "789", (1, 9))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 11))),
    ]);
}

#[test]
fn tokenize_number_literal_with_trailing_dot_at_end_of_file() {
    let mut source_code = " 777.";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_literal(777., "777.", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 5))),
    ]);
}

#[test]
fn tokenize_identifier_letters_only() {
    let mut source_code = "someIdentifier";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_identifier("someIdentifier", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 14))),
    ]);
}

#[test]
fn tokenize_identifier_alphanumeric() {
    let mut source_code = "club42";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_identifier("club42", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 6))),
    ]);
}

#[test]
fn tokenize_identifier_starting_with_underscore() {
    let mut source_code = "_identifierWithUnderscore";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_identifier("_identifierWithUnderscore", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 25))),
    ]);
}

#[test]
fn tokenize_identifier_and_semicolon() {
    let mut source_code = "number2add;";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_identifier("number2add", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::Semicolon, ";", (1, 11))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 11))),
    ]);
}

#[test]
fn tokenize_boolean_true() {
    let mut source_code = " true ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::True, "true", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 6))),
    ]);
}

#[test]
fn tokenize_boolean_false() {
    let mut source_code = " false; ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::False, "false", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::Semicolon, ";", (1, 7))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 8))),
    ]);
}

#[test]
fn tokenize_keyword_and() {
    let mut source_code = " and ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::And, "and", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 5))),
    ]);
}

#[test]
fn tokenize_keyword_or() {
    let mut source_code = " or ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::Or, "or", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 4))),
    ]);
}

#[test]
fn tokenize_keyword_class() {
    let mut source_code = " class Foo {} ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::Class, "class", (1, 2))),
        Ok(Token::new_identifier("Foo", (1, 8))),
        Ok(Token::new_nonliteral(TokenKind::LeftBrace, "{", (1, 12))),
        Ok(Token::new_nonliteral(TokenKind::RightBrace, "}", (1, 13))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 14))),
    ]);
}

#[test]
fn tokenize_keyword_fun() {
    let mut source_code = " fun foo() {} ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::Fun, "fun", (1, 2))),
        Ok(Token::new_identifier("foo", (1, 6))),
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 9))),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (1, 10))),
        Ok(Token::new_nonliteral(TokenKind::LeftBrace, "{", (1, 12))),
        Ok(Token::new_nonliteral(TokenKind::RightBrace, "}", (1, 13))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 14))),
    ]);
}

#[test]
fn tokenize_keyword_super() {
    let mut source_code = " super.method(); ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::Super, "super", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::Dot, ".", (1, 7))),
        Ok(Token::new_identifier("method", (1, 8))),
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 14))),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (1, 15))),
        Ok(Token::new_nonliteral(TokenKind::Semicolon, ";", (1, 16))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 17))),
    ]);
}

#[test]
fn tokenize_keyword_this() {
    let mut source_code = " this.name = \"John\"; ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::This, "this", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::Dot, ".", (1, 6))),
        Ok(Token::new_identifier("name", (1, 7))),
        Ok(Token::new_nonliteral(TokenKind::Equal, "=", (1, 12))),
        Ok(Token::new_literal("John", "\"John\"", (1, 14))),
        Ok(Token::new_nonliteral(TokenKind::Semicolon, ";", (1, 20))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 21))),
    ]);
}

#[test]
fn tokenize_keyword_var() {
    let mut source_code = " var x = 10; ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::Var, "var", (1, 2))),
        Ok(Token::new_identifier("x", (1, 6))),
        Ok(Token::new_nonliteral(TokenKind::Equal, "=", (1, 8))),
        Ok(Token::new_literal(10., "10", (1, 10))),
        Ok(Token::new_nonliteral(TokenKind::Semicolon, ";", (1, 12))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 13))),
    ]);
}

#[test]
fn tokenize_keyword_nil() {
    let mut source_code = " nil ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::Nil, "nil", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 5))),
    ]);
}

#[test]
fn tokenize_keyword_return() {
    let mut source_code = " return 42; ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::Return, "return", (1, 2))),
        Ok(Token::new_literal(42., "42", (1, 9))),
        Ok(Token::new_nonliteral(TokenKind::Semicolon, ";", (1, 11))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 12))),
    ]);
}

#[test]
fn tokenize_keywords_if_else() {
    let mut source_code = " if (x == 99) { } else { } ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::If, "if", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 5))),
        Ok(Token::new_identifier("x", (1, 6))),
        Ok(Token::new_nonliteral(TokenKind::EqualEqual, "==", (1, 8))),
        Ok(Token::new_literal(99., "99", (1, 11))),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (1, 13))),
        Ok(Token::new_nonliteral(TokenKind::LeftBrace, "{", (1, 15))),
        Ok(Token::new_nonliteral(TokenKind::RightBrace, "}", (1, 17))),
        Ok(Token::new_nonliteral(TokenKind::Else, "else", (1, 19))),
        Ok(Token::new_nonliteral(TokenKind::LeftBrace, "{", (1, 24))),
        Ok(Token::new_nonliteral(TokenKind::RightBrace, "}", (1, 26))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 27))),
    ]);
}

#[test]
fn tokenize_keyword_for() {
    let mut source_code = " for (;;) { } ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::For, "for", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 6))),
        Ok(Token::new_nonliteral(TokenKind::Semicolon, ";", (1, 7))),
        Ok(Token::new_nonliteral(TokenKind::Semicolon, ";", (1, 8))),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (1, 9))),
        Ok(Token::new_nonliteral(TokenKind::LeftBrace, "{", (1, 11))),
        Ok(Token::new_nonliteral(TokenKind::RightBrace, "}", (1, 13))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 14))),
    ]);
}

#[test]
fn tokenize_keyword_while() {
    let mut source_code = " while (x > 0) { } ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::While, "while", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 8))),
        Ok(Token::new_identifier("x", (1, 9))),
        Ok(Token::new_nonliteral(TokenKind::Greater, ">", (1, 11))),
        Ok(Token::new_literal(0., "0", (1, 13))),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (1, 14))),
        Ok(Token::new_nonliteral(TokenKind::LeftBrace, "{", (1, 16))),
        Ok(Token::new_nonliteral(TokenKind::RightBrace, "}", (1, 18))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 19))),
    ]);
}

#[test]
fn tokenize_keyword_print() {
    let mut source_code = " print(\"Hello, world!\"); ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::Print, "print", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 7))),
        Ok(Token::new_literal(
            "Hello, world!",
            "\"Hello, world!\"",
            (1, 8),
        )),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (1, 23))),
        Ok(Token::new_nonliteral(TokenKind::Semicolon, ";", (1, 24))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 25))),
    ]);
}

#[test]
fn tokenize_bang_identifier() {
    let mut source_code = "!a";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_nonliteral(TokenKind::Bang, "!", (1, 1))),
        Ok(Token::new_identifier("a", (1, 2))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 2))),
    ]);
}

#[test]
fn tokenize_equal_string_literal() {
    let mut source_code = "a=\"Jane\";";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_identifier("a", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::Equal, "=", (1, 2))),
        Ok(Token::new_literal("Jane", "\"Jane\"", (1, 3))),
        Ok(Token::new_nonliteral(TokenKind::Semicolon, ";", (1, 9))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 9))),
    ]);
}

#[test]
fn tokenize_greater_number_literal() {
    let mut source_code = "a>0.5";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_identifier("a", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::Greater, ">", (1, 2))),
        Ok(Token::new_literal(0.5, "0.5", (1, 3))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 5))),
    ]);
}

#[test]
fn tokenize_less_number_literal() {
    let mut source_code = "a<18";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_identifier("a", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::Less, "<", (1, 2))),
        Ok(Token::new_literal(18., "18", (1, 3))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 4))),
    ]);
}

#[test]
fn tokenize_slash_number_literal() {
    let mut source_code = "a/2.5";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_identifier("a", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::Slash, "/", (1, 2))),
        Ok(Token::new_literal(2.5, "2.5", (1, 3))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 5))),
    ]);
}

#[test]
fn tokenize_dot_after_integer_literal() {
    let mut source_code = "1.neg()";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::new_literal(1., "1", (1, 1))),
        Ok(Token::new_nonliteral(TokenKind::Dot, ".", (1, 2))),
        Ok(Token::new_identifier("neg", (1, 3))),
        Ok(Token::new_nonliteral(TokenKind::LeftParen, "(", (1, 6))),
        Ok(Token::new_nonliteral(TokenKind::RightParen, ")", (1, 7))),
        Ok(Token::new_nonliteral(TokenKind::EndOfFile, "", (1, 7))),
    ]);
}
