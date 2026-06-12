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

mod token {
    use super::*;

    #[test]
    fn debug_format_end_of_file() {
        let token = Token::EndOfFile;

        let debug_string = format!("{token:?}");

        assert_that!(debug_string).is_equal_to("EOF  null");
    }

    #[test]
    fn debug_format_comma() {
        let token = Token::Comma;

        let debug_string = format!("{token:?}");

        assert_that!(debug_string).is_equal_to("COMMA , null");
    }

    #[test]
    fn debug_format_left_paren() {
        let token = Token::LeftParen;

        let debug_string = format!("{token:?}");

        assert_that!(debug_string).is_equal_to("LEFT_PAREN ( null");
    }

    #[test]
    fn debug_format_right_paren() {
        let token = Token::RightParen;

        let debug_string = format!("{token:?}");

        assert_that!(debug_string).is_equal_to("RIGHT_PAREN ) null");
    }

    #[test]
    fn debug_format_left_brace() {
        let token = Token::LeftBrace;

        let debug_string = format!("{token:?}");

        assert_that!(debug_string).is_equal_to("LEFT_BRACE { null");
    }

    #[test]
    fn debug_format_right_brace() {
        let token = Token::RightBrace;

        let debug_string = format!("{token:?}");

        assert_that!(debug_string).is_equal_to("RIGHT_BRACE } null");
    }

    #[test]
    fn debug_format_dot() {
        let token = Token::Dot;

        let debug_string = format!("{token:?}");

        assert_that!(debug_string).is_equal_to("DOT . null");
    }

    #[test]
    fn debug_format_semicolon() {
        let token = Token::Semicolon;

        let debug_string = format!("{token:?}");

        assert_that!(debug_string).is_equal_to("SEMICOLON ; null");
    }
}

#[test]
fn can_tokenize_str() {
    let mut source_code = "()";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::LeftParen),
        Ok(Token::RightParen),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn can_tokenize_from_buf_reader() {
    let mut source_code = BufReader::new(Cursor::new("()"));

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::LeftParen),
        Ok(Token::RightParen),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_empty_source() {
    let mut source_code = "";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([Ok(Token::EndOfFile)]);
}

#[test]
fn tokenize_punctuations() {
    let mut source_code = "(){},.;";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::LeftParen),
        Ok(Token::RightParen),
        Ok(Token::LeftBrace),
        Ok(Token::RightBrace),
        Ok(Token::Comma),
        Ok(Token::Dot),
        Ok(Token::Semicolon),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_ignores_whitespace() {
    let mut source_code = " ( \t\n\r\u{000C} ) { \t\n\r\u{000C} } ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::LeftParen),
        Ok(Token::RightParen),
        Ok(Token::LeftBrace),
        Ok(Token::RightBrace),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_single_character_operators() {
    let mut source_code = "- + * / ! = < >";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Minus),
        Ok(Token::Plus),
        Ok(Token::Star),
        Ok(Token::Slash),
        Ok(Token::Bang),
        Ok(Token::Equal),
        Ok(Token::Less),
        Ok(Token::Greater),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_two_character_operators() {
    let mut source_code = "== != <= >=";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::EqualEqual),
        Ok(Token::BangEqual),
        Ok(Token::LessEqual),
        Ok(Token::GreaterEqual),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_line_comment_not_on_last_line() {
    let mut source_code = "() // (some comment).\n{}";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::LeftParen),
        Ok(Token::RightParen),
        Ok(Token::LeftBrace),
        Ok(Token::RightBrace),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_line_comment_at_end_of_file() {
    let mut source_code = "() // (some comment).";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::LeftParen),
        Ok(Token::RightParen),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_line_comments_on_two_subsequent_lines() {
    let mut source_code = "() \n// (first line of comment).\n// (second line of comment).\n{}";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::LeftParen),
        Ok(Token::RightParen),
        Ok(Token::LeftBrace),
        Ok(Token::RightBrace),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_unexpected_character_at_line_1_char_4() {
    let mut source_code = "(){§},.;";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::LeftParen),
        Ok(Token::RightParen),
        Ok(Token::LeftBrace),
        Err(LexingError {
            code: LexingErrorCode::UnexpectedCharacter('§'),
            location: Location { line: 1, char: 4 },
        }),
        Ok(Token::RightBrace),
        Ok(Token::Comma),
        Ok(Token::Dot),
        Ok(Token::Semicolon),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_unexpected_character_at_line_3_char_1() {
    let mut source_code = "{\n;\n§}\n";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::LeftBrace),
        Ok(Token::Semicolon),
        Err(LexingError {
            code: LexingErrorCode::UnexpectedCharacter('§'),
            location: Location { line: 3, char: 1 },
        }),
        Ok(Token::RightBrace),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_string_literal_empty_string() {
    let mut source_code = r#""""#;

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::StringLiteral(String::new())),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_string_literal_with_some_characters() {
    let mut source_code = r#""some text""#;

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::StringLiteral("some text".to_string())),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_string_literal_with_line_breaks() {
    let mut source_code = "\"first line\nsecond line\r\nthird line\"";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::StringLiteral(
            "first line\nsecond line\r\nthird line".to_string(),
        )),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_string_literal_which_is_unterminated() {
    let mut source_code = r#""some text"#;

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Err(LexingError {
            code: LexingErrorCode::UnterminatedStringLiteral("some text".to_string()),
            location: Location { line: 1, char: 10 },
        }),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_number_literal_integer_0() {
    let mut source_code = "0";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([Ok(Token::NumberLiteral(0.)), Ok(Token::EndOfFile)]);
}

#[test]
fn tokenize_number_literal_integer_256() {
    let mut source_code = "256";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([Ok(Token::NumberLiteral(256.)), Ok(Token::EndOfFile)]);
}

#[test]
fn tokenize_number_literal_float_1_98() {
    let mut source_code = "1.98";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([Ok(Token::NumberLiteral(1.98)), Ok(Token::EndOfFile)]);
}

#[test]
fn tokenize_list_of_number_literal_float_1_98() {
    let mut source_code = "811.2344, 5.0, 0.67";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::NumberLiteral(811.2344)),
        Ok(Token::Comma),
        Ok(Token::NumberLiteral(5.0)),
        Ok(Token::Comma),
        Ok(Token::NumberLiteral(0.67)),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_number_literal_with_dot_at_the_end() {
    let mut source_code = "400,\n4.,\n655";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::NumberLiteral(400.)),
        Ok(Token::Comma),
        Err(LexingError {
            code: LexingErrorCode::InvalidNumberLiteral("4.".to_string()),
            location: Location { line: 2, char: 3 },
        }),
        Ok(Token::Comma),
        Ok(Token::NumberLiteral(655.)),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_identifier_letters_only() {
    let mut source_code = "someIdentifier";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Identifier("someIdentifier".to_string())),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_identifier_alphanumeric() {
    let mut source_code = "club42";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Identifier("club42".to_string())),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_identifier_starting_with_underscore() {
    let mut source_code = "_identifierWithUnderscore";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Identifier("_identifierWithUnderscore".to_string())),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_identifier_and_semicolon() {
    let mut source_code = "number2add;";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Identifier("number2add".to_string())),
        Ok(Token::Semicolon),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_boolean_true() {
    let mut source_code = " true ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([Ok(Token::True), Ok(Token::EndOfFile)]);
}

#[test]
fn tokenize_boolean_false() {
    let mut source_code = " false; ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::False),
        Ok(Token::Semicolon),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_keyword_and() {
    let mut source_code = " and ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([Ok(Token::And), Ok(Token::EndOfFile)]);
}

#[test]
fn tokenize_keyword_or() {
    let mut source_code = " or ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([Ok(Token::Or), Ok(Token::EndOfFile)]);
}

#[test]
fn tokenize_keyword_class() {
    let mut source_code = " class Foo {} ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Class),
        Ok(Token::Identifier("Foo".to_string())),
        Ok(Token::LeftBrace),
        Ok(Token::RightBrace),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_keyword_fun() {
    let mut source_code = " fun foo() {} ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Fun),
        Ok(Token::Identifier("foo".to_string())),
        Ok(Token::LeftParen),
        Ok(Token::RightParen),
        Ok(Token::LeftBrace),
        Ok(Token::RightBrace),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_keyword_super() {
    let mut source_code = " super.method(); ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Super),
        Ok(Token::Dot),
        Ok(Token::Identifier("method".to_string())),
        Ok(Token::LeftParen),
        Ok(Token::RightParen),
        Ok(Token::Semicolon),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_keyword_this() {
    let mut source_code = " this.name = \"John\"; ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::This),
        Ok(Token::Dot),
        Ok(Token::Identifier("name".to_string())),
        Ok(Token::Equal),
        Ok(Token::StringLiteral("John".to_string())),
        Ok(Token::Semicolon),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_keyword_var() {
    let mut source_code = " var x = 10; ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Var),
        Ok(Token::Identifier("x".to_string())),
        Ok(Token::Equal),
        Ok(Token::NumberLiteral(10.)),
        Ok(Token::Semicolon),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_keyword_nil() {
    let mut source_code = " nil ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([Ok(Token::Nil), Ok(Token::EndOfFile)]);
}

#[test]
fn tokenize_keyword_return() {
    let mut source_code = " return 42; ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Return),
        Ok(Token::NumberLiteral(42.)),
        Ok(Token::Semicolon),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_keywords_if_else() {
    let mut source_code = " if (x == 99) { } else { } ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::If),
        Ok(Token::LeftParen),
        Ok(Token::Identifier("x".to_string())),
        Ok(Token::EqualEqual),
        Ok(Token::NumberLiteral(99.)),
        Ok(Token::RightParen),
        Ok(Token::LeftBrace),
        Ok(Token::RightBrace),
        Ok(Token::Else),
        Ok(Token::LeftBrace),
        Ok(Token::RightBrace),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_keyword_for() {
    let mut source_code = " for (;;) { } ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::For),
        Ok(Token::LeftParen),
        Ok(Token::Semicolon),
        Ok(Token::Semicolon),
        Ok(Token::RightParen),
        Ok(Token::LeftBrace),
        Ok(Token::RightBrace),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_keyword_while() {
    let mut source_code = " while (x > 0) { } ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::While),
        Ok(Token::LeftParen),
        Ok(Token::Identifier("x".to_string())),
        Ok(Token::Greater),
        Ok(Token::NumberLiteral(0.)),
        Ok(Token::RightParen),
        Ok(Token::LeftBrace),
        Ok(Token::RightBrace),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_keyword_print() {
    let mut source_code = " print(\"Hello, world!\"); ";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Print),
        Ok(Token::LeftParen),
        Ok(Token::StringLiteral("Hello, world!".to_string())),
        Ok(Token::RightParen),
        Ok(Token::Semicolon),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_bang_identifier() {
    let mut source_code = "!a";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Bang),
        Ok(Token::Identifier("a".to_string())),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_equal_string_literal() {
    let mut source_code = "a=\"Jane\";";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Identifier("a".to_string())),
        Ok(Token::Equal),
        Ok(Token::StringLiteral("Jane".to_string())),
        Ok(Token::Semicolon),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_greater_number_literal() {
    let mut source_code = "a>0.5";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Identifier("a".to_string())),
        Ok(Token::Greater),
        Ok(Token::NumberLiteral(0.5)),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_less_number_literal() {
    let mut source_code = "a<18";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Identifier("a".to_string())),
        Ok(Token::Less),
        Ok(Token::NumberLiteral(18.)),
        Ok(Token::EndOfFile),
    ]);
}

#[test]
fn tokenize_slash_number_literal() {
    let mut source_code = "a/2.5";

    let tokens = source_code.tokenize().collect::<Vec<_>>();

    assert_that!(tokens).contains_exactly([
        Ok(Token::Identifier("a".to_string())),
        Ok(Token::Slash),
        Ok(Token::NumberLiteral(2.5)),
        Ok(Token::EndOfFile),
    ]);
}
