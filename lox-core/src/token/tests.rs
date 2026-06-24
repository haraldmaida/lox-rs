use super::*;
use asserting::prelude::*;

mod token_kind {
    use super::*;

    #[test]
    fn display_format_end_of_file() {
        let token_kind = TokenKind::EndOfFile;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("EOF");
    }

    #[test]
    fn display_format_left_paren() {
        let token_kind = TokenKind::LeftParen;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("(");
    }

    #[test]
    fn display_format_right_paren() {
        let token_kind = TokenKind::RightParen;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to(")");
    }

    #[test]
    fn display_format_left_brace() {
        let token_kind = TokenKind::LeftBrace;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("{");
    }

    #[test]
    fn display_format_right_brace() {
        let token_kind = TokenKind::RightBrace;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("}");
    }

    #[test]
    fn display_format_comma() {
        let token_kind = TokenKind::Comma;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to(",");
    }

    #[test]
    fn display_format_dot() {
        let token_kind = TokenKind::Dot;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to(".");
    }

    #[test]
    fn display_format_semicolon() {
        let token_kind = TokenKind::Semicolon;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to(";");
    }

    #[test]
    fn display_format_minus() {
        let token_kind = TokenKind::Minus;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("-");
    }

    #[test]
    fn display_format_plus() {
        let token_kind = TokenKind::Plus;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("+");
    }

    #[test]
    fn display_format_star() {
        let token_kind = TokenKind::Star;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("*");
    }

    #[test]
    fn display_format_slash() {
        let token_kind = TokenKind::Slash;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("/");
    }

    #[test]
    fn display_format_bang() {
        let token_kind = TokenKind::Bang;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("!");
    }

    #[test]
    fn display_format_equal() {
        let token_kind = TokenKind::Equal;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("=");
    }

    #[test]
    fn display_format_greater() {
        let token_kind = TokenKind::Greater;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to(">");
    }

    #[test]
    fn display_format_less() {
        let token_kind = TokenKind::Less;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("<");
    }

    #[test]
    fn display_format_bang_equal() {
        let token_kind = TokenKind::BangEqual;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("!=");
    }

    #[test]
    fn display_format_equal_equal() {
        let token_kind = TokenKind::EqualEqual;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("==");
    }

    #[test]
    fn display_format_greater_equal() {
        let token_kind = TokenKind::GreaterEqual;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to(">=");
    }

    #[test]
    fn display_format_less_equal() {
        let token_kind = TokenKind::LessEqual;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("<=");
    }

    #[test]
    fn display_format_string_literal() {
        let token_kind = TokenKind::StringLiteral;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("string literal");
    }

    #[test]
    fn display_format_number_literal() {
        let token_kind = TokenKind::NumberLiteral;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("number literal");
    }

    #[test]
    fn display_format_identifier() {
        let token_kind = TokenKind::Identifier;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("identifier");
    }

    #[test]
    fn display_format_and() {
        let token_kind = TokenKind::And;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("and");
    }

    #[test]
    fn display_format_class() {
        let token_kind = TokenKind::Class;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("class");
    }

    #[test]
    fn display_format_else() {
        let token_kind = TokenKind::Else;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("else");
    }

    #[test]
    fn display_format_false() {
        let token_kind = TokenKind::False;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("false");
    }

    #[test]
    fn display_format_fun() {
        let token_kind = TokenKind::Fun;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("fun");
    }

    #[test]
    fn display_format_for() {
        let token_kind = TokenKind::For;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("for");
    }

    #[test]
    fn display_format_if() {
        let token_kind = TokenKind::If;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("if");
    }

    #[test]
    fn display_format_nil() {
        let token_kind = TokenKind::Nil;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("nil");
    }

    #[test]
    fn display_format_or() {
        let token_kind = TokenKind::Or;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("or");
    }

    #[test]
    fn display_format_print() {
        let token_kind = TokenKind::Print;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("print");
    }

    #[test]
    fn display_format_return() {
        let token_kind = TokenKind::Return;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("return");
    }

    #[test]
    fn display_format_super() {
        let token_kind = TokenKind::Super;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("super");
    }

    #[test]
    fn display_format_this() {
        let token_kind = TokenKind::This;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("this");
    }

    #[test]
    fn display_format_true() {
        let token_kind = TokenKind::True;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("true");
    }

    #[test]
    fn display_format_var() {
        let token_kind = TokenKind::Var;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("var");
    }

    #[test]
    fn display_format_while() {
        let token_kind = TokenKind::While;
        let display_string = token_kind.to_string();
        assert_that!(display_string).is_equal_to("while");
    }

    #[test]
    fn alternate_display_format_end_of_file() {
        let token_kind = TokenKind::EndOfFile;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("EOF");
    }

    #[test]
    fn alternate_display_format_left_paren() {
        let token_kind = TokenKind::LeftParen;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("LEFT_PAREN");
    }

    #[test]
    fn alternate_display_format_right_paren() {
        let token_kind = TokenKind::RightParen;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("RIGHT_PAREN");
    }

    #[test]
    fn alternate_display_format_left_brace() {
        let token_kind = TokenKind::LeftBrace;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("LEFT_BRACE");
    }

    #[test]
    fn alternate_display_format_right_brace() {
        let token_kind = TokenKind::RightBrace;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("RIGHT_BRACE");
    }

    #[test]
    fn alternate_display_format_comma() {
        let token_kind = TokenKind::Comma;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("COMMA");
    }

    #[test]
    fn alternate_display_format_dot() {
        let token_kind = TokenKind::Dot;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("DOT");
    }

    #[test]
    fn alternate_display_format_semicolon() {
        let token_kind = TokenKind::Semicolon;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("SEMICOLON");
    }

    #[test]
    fn alternate_display_format_minus() {
        let token_kind = TokenKind::Minus;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("MINUS");
    }

    #[test]
    fn alternate_display_format_plus() {
        let token_kind = TokenKind::Plus;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("PLUS");
    }

    #[test]
    fn alternate_display_format_star() {
        let token_kind = TokenKind::Star;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("STAR");
    }

    #[test]
    fn alternate_display_format_slash() {
        let token_kind = TokenKind::Slash;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("SLASH");
    }

    #[test]
    fn alternate_display_format_bang() {
        let token_kind = TokenKind::Bang;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("BANG");
    }

    #[test]
    fn alternate_display_format_equal() {
        let token_kind = TokenKind::Equal;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("EQUAL");
    }

    #[test]
    fn alternate_display_format_greater() {
        let token_kind = TokenKind::Greater;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("GREATER");
    }

    #[test]
    fn alternate_display_format_less() {
        let token_kind = TokenKind::Less;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("LESS");
    }

    #[test]
    fn alternate_display_format_bang_equal() {
        let token_kind = TokenKind::BangEqual;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("BANG_EQUAL");
    }

    #[test]
    fn alternate_display_format_equal_equal() {
        let token_kind = TokenKind::EqualEqual;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("EQUAL_EQUAL");
    }

    #[test]
    fn alternate_display_format_greater_equal() {
        let token_kind = TokenKind::GreaterEqual;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("GREATER_EQUAL");
    }

    #[test]
    fn alternate_display_format_less_equal() {
        let token_kind = TokenKind::LessEqual;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("LESS_EQUAL");
    }

    #[test]
    fn alternate_display_format_string_literal() {
        let token_kind = TokenKind::StringLiteral;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("STRING");
    }

    #[test]
    fn alternate_display_format_number_literal() {
        let token_kind = TokenKind::NumberLiteral;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("NUMBER");
    }

    #[test]
    fn alternate_display_format_identifier() {
        let token_kind = TokenKind::Identifier;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("IDENTIFIER");
    }

    #[test]
    fn alternate_display_format_and() {
        let token_kind = TokenKind::And;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("AND");
    }

    #[test]
    fn alternate_display_format_class() {
        let token_kind = TokenKind::Class;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("CLASS");
    }

    #[test]
    fn alternate_display_format_else() {
        let token_kind = TokenKind::Else;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("ELSE");
    }

    #[test]
    fn alternate_display_format_false() {
        let token_kind = TokenKind::False;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("FALSE");
    }

    #[test]
    fn alternate_display_format_fun() {
        let token_kind = TokenKind::Fun;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("FUN");
    }

    #[test]
    fn alternate_display_format_for() {
        let token_kind = TokenKind::For;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("FOR");
    }

    #[test]
    fn alternate_display_format_if() {
        let token_kind = TokenKind::If;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("IF");
    }

    #[test]
    fn alternate_display_format_nil() {
        let token_kind = TokenKind::Nil;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("NIL");
    }

    #[test]
    fn alternate_display_format_or() {
        let token_kind = TokenKind::Or;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("OR");
    }

    #[test]
    fn alternate_display_format_print() {
        let token_kind = TokenKind::Print;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("PRINT");
    }

    #[test]
    fn alternate_display_format_return() {
        let token_kind = TokenKind::Return;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("RETURN");
    }

    #[test]
    fn alternate_display_format_super() {
        let token_kind = TokenKind::Super;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("SUPER");
    }

    #[test]
    fn alternate_display_format_this() {
        let token_kind = TokenKind::This;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("THIS");
    }

    #[test]
    fn alternate_display_format_true() {
        let token_kind = TokenKind::True;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("TRUE");
    }

    #[test]
    fn alternate_display_format_var() {
        let token_kind = TokenKind::Var;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("VAR");
    }

    #[test]
    fn alternate_display_format_while() {
        let token_kind = TokenKind::While;
        let display_string = format!("{token_kind:#}");
        assert_that!(display_string).is_equal_to("WHILE");
    }
}

mod token {
    use super::*;

    #[test]
    fn display_format_end_of_file() {
        let token = Token {
            kind: TokenKind::EndOfFile,
            literal: None,
            lexeme: "".into(),
            location: (2, 1).into(),
        };
        let display_string = token.to_string();
        assert_that!(display_string).is_equal_to("");
    }

    #[test]
    fn display_format_nil() {
        let token = Token {
            kind: TokenKind::Nil,
            literal: None,
            lexeme: "nil".into(),
            location: (2, 1).into(),
        };
        let display_string = token.to_string();
        assert_that!(display_string).is_equal_to("nil");
    }

    #[test]
    fn display_format_true() {
        let token = Token {
            kind: TokenKind::True,
            literal: None,
            lexeme: "true".into(),
            location: (2, 1).into(),
        };
        let display_string = token.to_string();
        assert_that!(display_string).is_equal_to("true");
    }

    #[test]
    fn display_format_bang_equal() {
        let token = Token {
            kind: TokenKind::BangEqual,
            literal: None,
            lexeme: "!=".into(),
            location: (2, 1).into(),
        };
        let display_string = token.to_string();
        assert_that!(display_string).is_equal_to("!=");
    }

    #[test]
    fn display_format_string_literal() {
        let token = Token {
            kind: TokenKind::StringLiteral,
            literal: Some(Literal::String("Hello, World!".into())),
            lexeme: "\"Hello, World!\"".into(),
            location: (2, 1).into(),
        };
        let display_string = token.to_string();
        assert_that!(display_string).is_equal_to("\"Hello, World!\"");
    }

    #[test]
    fn display_format_number_literal_integer() {
        let token = Token {
            kind: TokenKind::NumberLiteral,
            literal: Some(Literal::Number(123.)),
            lexeme: "123".into(),
            location: (2, 1).into(),
        };
        let display_string = token.to_string();
        assert_that!(display_string).is_equal_to("123");
    }

    #[test]
    fn display_format_number_literal_float() {
        let token = Token {
            kind: TokenKind::NumberLiteral,
            literal: Some(Literal::Number(123.456)),
            lexeme: "123.456".into(),
            location: (2, 1).into(),
        };
        let display_string = token.to_string();
        assert_that!(display_string).is_equal_to("123.456");
    }

    #[test]
    fn display_format_identifier() {
        let token = Token {
            kind: TokenKind::Identifier,
            literal: None,
            lexeme: "foo".into(),
            location: (2, 1).into(),
        };
        let display_string = token.to_string();
        assert_that!(display_string).is_equal_to("foo");
    }

    #[test]
    fn alternate_display_format_end_of_file() {
        let token = Token {
            kind: TokenKind::EndOfFile,
            literal: None,
            lexeme: "".into(),
            location: (2, 1).into(),
        };
        let display_string = format!("{token:#}");
        assert_that!(display_string).is_equal_to("EOF  null");
    }

    #[test]
    fn alternate_display_format_string_literal() {
        let token = Token {
            kind: TokenKind::StringLiteral,
            literal: Some(Literal::String("Hello, World!".into())),
            lexeme: "\"Hello, World!\"".into(),
            location: (2, 1).into(),
        };
        let display_string = format!("{token:#}");
        assert_that!(display_string).is_equal_to("STRING \"Hello, World!\" Hello, World!");
    }

    #[test]
    fn alternate_display_format_number_literal_integer() {
        let token = Token {
            kind: TokenKind::NumberLiteral,
            literal: Some(Literal::Number(123.)),
            lexeme: "123".into(),
            location: (2, 1).into(),
        };
        let display_string = format!("{token:#}");
        assert_that!(display_string).is_equal_to("NUMBER 123 123.0");
    }

    #[test]
    fn alternate_display_format_number_literal_float() {
        let token = Token {
            kind: TokenKind::NumberLiteral,
            literal: Some(Literal::Number(123.456)),
            lexeme: "123.456".into(),
            location: (2, 1).into(),
        };
        let display_string = format!("{token:#}");
        assert_that!(display_string).is_equal_to("NUMBER 123.456 123.456");
    }

    #[test]
    fn alternate_display_format_identifier() {
        let token = Token {
            kind: TokenKind::Identifier,
            literal: None,
            lexeme: "foo".into(),
            location: (2, 1).into(),
        };
        let display_string = format!("{token:#}");
        assert_that!(display_string).is_equal_to("IDENTIFIER foo null");
    }
}
