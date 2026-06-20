use super::*;
use asserting::prelude::*;

#[test]
fn clock_returns_the_current_time_as_a_float_in_seconds_and_fraction_of_seconds() {
    let time = clock();

    match time {
        Value::Nil => panic!("expected a number, but got nil"),
        Value::Bool(val) => panic!("expected a number, but got boolean {val:?}"),
        Value::Number(val) => {
            assert_that!(val.fract()).is_not_close_to(0.);
        },
        Value::String(val) => panic!("expected a number, but got string {val:?}"),
        Value::Callable(val) => panic!("expected a number, but got callable {val:?}"),
    }
}
