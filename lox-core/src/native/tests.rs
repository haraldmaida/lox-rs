use super::*;
use asserting::prelude::*;

#[test]
fn clock_returns_the_current_time_as_a_float_in_seconds_and_fraction_of_seconds() {
    let time = clock(&[]);

    match time {
        Ok(Value::Nil) => panic!("expected a number, but got nil"),
        Ok(Value::Bool(val)) => panic!("expected a number, but got boolean {val:?}"),
        Ok(Value::Number(val)) => {
            assert_that!(val.fract()).is_not_close_to(0.);
        },
        Ok(Value::String(val)) => panic!("expected a number, but got string {val:?}"),
        Ok(Value::Callable(val)) => panic!("expected a number, but got callable {val:?}"),
        Err(err) => panic!("expected a number, but got error {err:?}"),
    }
}
