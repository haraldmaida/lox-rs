use super::Location;
use asserting::prelude::*;
use proptest::prelude::*;

mod location {
    use super::*;

    #[test]
    fn default_location_is_at_line_1_char_0() {
        let location = Location::default();

        assert_that!(location).is_equal_to(Location { line: 1, char: 0 });
    }

    #[test]
    fn can_be_formatted_for_display() {
        let location = Location { line: 1, char: 2 };

        assert_that!(location).has_display_string("1:2");
    }

    proptest! {
        #[test]
        fn advance_char_adds_one_to_the_char_position_but_never_changes_the_line_position(
            line in any::<usize>(),
            char in any::<usize>().prop_filter("char position less than max usize", |n| *n < usize::MAX)
        ) {
            let mut location = Location { line, char };

            location.advance_char();

            assert_that!(location.char()).is_equal_to(char + 1);
            assert_that!(location.line()).is_equal_to(line);
        }
    }
}
