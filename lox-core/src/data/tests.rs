use super::*;
use asserting::prelude::*;
use proptest::prelude::*;

mod value {
    use super::*;

    #[test]
    fn nil_is_not_truthy() {
        assert_that!(Value::Nil.is_truthy()).is_false();
    }

    #[test]
    fn boolean_false_is_not_truthy() {
        assert_that!(Value::Bool(false).is_truthy()).is_false();
    }

    #[test]
    fn boolean_true_is_truthy() {
        assert_that!(Value::Bool(true).is_truthy()).is_true();
    }

    proptest! {
        #[test]
        fn any_number_including_0_and_negative_numbers_is_truthy(
            num in any::<f64>(),
        ) {
            prop_assert!(Value::Number(num).is_truthy());
        }

        #[test]
        fn any_string_including_empty_strings_and_string_of_char_0_is_truthy(
            string in any::<String>(),
        ) {
            prop_assert!(Value::String(string).is_truthy());
        }
    }

    #[test]
    fn nil_is_equal_to_nil() {
        assert_that!(Value::Nil == Value::Nil).is_true();
    }

    #[test]
    fn nil_is_not_equal_to_bool() {
        assert_that!(Value::Nil == Value::Bool(false)).is_false();
        assert_that!(Value::Nil == Value::Bool(true)).is_false();
    }

    proptest! {
        #[test]
        fn nil_is_not_equal_to_any_number(
            num in any::<f64>(),
        ) {
            prop_assert!(Value::Nil != Value::Number(num));
        }

        #[test]
        fn nil_is_not_equal_to_any_string(
            strg in any::<String>(),
        ) {
            prop_assert!(Value::Nil != Value::String(strg));
        }
    }

    #[test]
    fn boolean_false_is_equal_to_false() {
        assert_that!(Value::Bool(false) == Value::Bool(false)).is_true();
    }

    #[test]
    fn boolean_false_is_not_equal_to_true() {
        assert_that!(Value::Bool(false) == Value::Bool(true)).is_false();
    }

    #[test]
    fn boolean_true_is_equal_to_true() {
        assert_that!(Value::Bool(true) == Value::Bool(true)).is_true();
    }

    #[test]
    fn boolean_true_is_not_equal_to_false() {
        assert_that!(Value::Bool(true) == Value::Bool(false)).is_false();
    }

    proptest! {
        #[test]
        fn any_boolean_is_not_equal_to_any_number(
            boolean in any::<bool>(),
            num in any::<f64>(),
        ) {
            prop_assert!(Value::Bool(boolean) != Value::Number(num));
        }

        #[test]
        fn any_boolean_is_not_equal_to_any_string(
            boolean in any::<bool>(),
            strg in any::<String>(),
        ) {
            prop_assert!(Value::Bool(boolean) != Value::String(strg));
        }

        #[test]
        fn any_number_is_equal_to_the_same_number(
            num in any::<f64>(),
        ) {
            prop_assert!(Value::Number(num) == Value::Number(num));
        }

        #[allow(clippy::float_cmp)]
        #[test]
        fn any_number_is_not_equal_to_another_number(
            (a, b) in (any::<f64>(), any::<f64>()).prop_filter("a != b", |(a, b)| a != b),
        ) {
            prop_assert!(Value::Number(a) != Value::Number(b));
        }

        #[test]
        fn any_number_is_not_equal_to_any_string(
            num in any::<f64>(),
            strg in any::<String>(),
        ) {
            prop_assert!(Value::Number(num) != Value::String(strg));
        }

        #[test]
        fn any_string_is_equal_to_the_same_string(
            strg in any::<String>(),
        ) {
            prop_assert!(Value::String(strg.clone()) == Value::String(strg));
        }

        #[test]
        fn any_string_is_not_equal_to_another_string(
            (a, b) in (any::<String>(), any::<String>()).prop_filter("a != b", |(a, b)| a != b),
        ) {
            prop_assert!(Value::String(a) != Value::String(b));
        }
    }

    #[test]
    fn display_format_nil() {
        let value = Value::Nil;

        let formatted = value.to_string();

        assert_that!(formatted).is_equal_to("nil");
    }

    #[test]
    fn display_format_boolean_false() {
        let value = Value::Bool(false);

        let formatted = value.to_string();

        assert_that!(formatted).is_equal_to("false");
    }

    #[test]
    fn display_format_boolean_true() {
        let value = Value::Bool(true);

        let formatted = value.to_string();

        assert_that!(formatted).is_equal_to("true");
    }

    #[test]
    fn display_format_number() {
        let value = Value::Number(123_456.789_012);

        let formatted = value.to_string();

        assert_that!(formatted).is_equal_to("123456.789012");
    }

    proptest! {
        #[test]
        fn display_format_number_with_no_fractional_part_does_not_contain_decimal_point(
            num in any::<i32>().prop_map(f64::from),
        ) {
            let value = Value::Number(num);

            let formatted = value.to_string();

            assert_that!(formatted).does_not_end_with(".0").does_not_contain('.');
        }
    }

    #[test]
    fn display_format_string() {
        let value = Value::String("Hello, world!".into());

        let formatted = format!("{value}");

        assert_that!(formatted).is_equal_to("Hello, world!");
    }

    proptest! {
        #[test]
        fn display_format_string_does_not_contain_double_quotes(
            strg in any::<String>().prop_filter("string not surrounded by \")", |strg| !strg.starts_with('"') && !strg.ends_with('"')),
        ) {
            let value = Value::String(strg);

            let formatted = format!("{value}");

            assert_that!(formatted).does_not_start_with('"').does_not_end_with('"');
        }
    }
}
