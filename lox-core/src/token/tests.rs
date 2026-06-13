use super::*;
use asserting::prelude::*;

mod token {
    use super::*;

    #[test]
    fn debug_format_end_of_file() {
        let token = Token::EndOfFile;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("EOF  null");
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
    fn debug_format_comma() {
        let token = Token::Comma;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("COMMA , null");
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

    #[test]
    fn debug_format_minus() {
        let token = Token::Minus;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("MINUS - null");
    }

    #[test]
    fn debug_format_plus() {
        let token = Token::Plus;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("PLUS + null");
    }

    #[test]
    fn debug_format_star() {
        let token = Token::Star;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("STAR * null");
    }

    #[test]
    fn debug_format_slash() {
        let token = Token::Slash;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("SLASH / null");
    }

    #[test]
    fn debug_format_bang() {
        let token = Token::Bang;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("BANG ! null");
    }

    #[test]
    fn debug_format_equal() {
        let token = Token::Equal;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("EQUAL = null");
    }

    #[test]
    fn debug_format_greater() {
        let token = Token::Greater;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("GREATER > null");
    }

    #[test]
    fn debug_format_less() {
        let token = Token::Less;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("LESS < null");
    }

    #[test]
    fn debug_format_bang_equal() {
        let token = Token::BangEqual;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("BANG_EQUAL != null");
    }

    #[test]
    fn debug_format_equal_equal() {
        let token = Token::EqualEqual;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("EQUAL_EQUAL == null");
    }

    #[test]
    fn debug_format_greater_equal() {
        let token = Token::GreaterEqual;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("GREATER_EQUAL >= null");
    }

    #[test]
    fn debug_format_less_equal() {
        let token = Token::LessEqual;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("LESS_EQUAL <= null");
    }

    #[test]
    fn debug_format_string_literal() {
        let token = Token::StringLiteral("Hello, World!".to_string());
        let debug_string = format!("{token:?}");
        assert_that!(debug_string)
            .is_equal_to("STRING_LITERAL \"Hello, World!\" \"Hello, World!\"");
    }

    #[test]
    fn debug_format_number_literal() {
        let token = Token::NumberLiteral(123.456);
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("NUMBER_LITERAL 123.456 123.456");
    }

    #[test]
    fn debug_format_identifier() {
        let token = Token::Identifier("foo".to_string());
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("IDENTIFIER foo foo");
    }

    #[test]
    fn debug_format_and() {
        let token = Token::And;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("AND and null");
    }

    #[test]
    fn debug_format_class() {
        let token = Token::Class;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("CLASS class null");
    }

    #[test]
    fn debug_format_else() {
        let token = Token::Else;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("ELSE else null");
    }

    #[test]
    fn debug_format_false() {
        let token = Token::False;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("FALSE false null");
    }

    #[test]
    fn debug_format_fun() {
        let token = Token::Fun;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("FUN fun null");
    }

    #[test]
    fn debug_format_for() {
        let token = Token::For;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("FOR for null");
    }

    #[test]
    fn debug_format_if() {
        let token = Token::If;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("IF if null");
    }

    #[test]
    fn debug_format_nil() {
        let token = Token::Nil;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("NIL nil null");
    }

    #[test]
    fn debug_format_or() {
        let token = Token::Or;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("OR or null");
    }

    #[test]
    fn debug_format_print() {
        let token = Token::Print;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("PRINT print null");
    }

    #[test]
    fn debug_format_return() {
        let token = Token::Return;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("RETURN return null");
    }

    #[test]
    fn debug_format_super() {
        let token = Token::Super;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("SUPER super null");
    }

    #[test]
    fn debug_format_this() {
        let token = Token::This;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("THIS this null");
    }

    #[test]
    fn debug_format_true() {
        let token = Token::True;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("TRUE true null");
    }

    #[test]
    fn debug_format_var() {
        let token = Token::Var;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("VAR var null");
    }

    #[test]
    fn debug_format_while() {
        let token = Token::While;
        let debug_string = format!("{token:?}");
        assert_that!(debug_string).is_equal_to("WHILE while null");
    }
}
