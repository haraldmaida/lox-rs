use super::{Location, *};
use asserting::prelude::*;

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
        Err(TokenizeError {
            code: TokenizeErrorCode::UnexpectedCharacter('§'),
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
        Err(TokenizeError {
            code: TokenizeErrorCode::UnexpectedCharacter('§'),
            location: Location { line: 3, char: 1 },
        }),
        Ok(Token::RightBrace),
        Ok(Token::EndOfFile),
    ]);
}

mod tokenize_error_code {
    use super::*;

    #[test]
    fn can_be_converted_from_io_error() {
        let code = TokenizeErrorCode::from(io::Error::from(io::ErrorKind::UnexpectedEof));

        assert_that!(code).is_equal_to(TokenizeErrorCode::IoError(
            "unexpected end of file".to_string(),
        ));
    }
}

mod tokenize_error {
    use super::*;

    #[test]
    fn display_format_io_error() {
        let error = TokenizeError {
            code: TokenizeErrorCode::IoError("I/O error".to_string()),
            location: Location { line: 1, char: 0 },
        };

        assert_that!(error.to_string()).is_equal_to("I/O error at 1:0");
    }

    #[test]
    fn display_format_unexpected_character() {
        let error = TokenizeError {
            code: TokenizeErrorCode::UnexpectedCharacter('§'),
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
