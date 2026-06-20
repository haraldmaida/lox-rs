use super::*;
use asserting::prelude::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn defining_a_variable_and_looking_it_up_returns_the_same_value(
        name in any::<String>(),
        value in any::<Value>(),
    ) {
        let environment = Environment::global();

        let symbol = Symbol::from(name);
        environment.define(symbol, value.clone());
        let maybe_value = environment.lookup(symbol);

        prop_assert_eq!(maybe_value, Ok(value));
    }

    #[test]
    fn looking_up_a_non_existing_variable_returns_an_error(
        name in any::<String>(),
    ) {
        let environment = Environment::global();
        let symbol = Symbol::from(name);

        let maybe_value = environment.lookup(symbol);

        prop_assert_eq!(maybe_value, Err(EnvironmentError::UndefinedVariable(symbol)));
    }

    #[test]
    fn assigning_a_value_to_an_existing_variable_and_looking_it_up_returns_the_new_value(
        name in any::<String>(),
        value_before in any::<Value>(),
        value_after in any::<Value>(),
    ) {
        let environment = Environment::global();
        let symbol = Symbol::from(name);
        environment.define(symbol, value_before);

        let result = environment.assign(symbol, value_after.clone());

        prop_assert!(result.is_ok());
        let maybe_value = environment.lookup(symbol);
        prop_assert_eq!(maybe_value, Ok(value_after));
    }

    #[test]
    fn assigning_a_value_to_a_non_existing_variable_returns_an_error(
        name in any::<String>(),
        value in any::<Value>(),
    ) {
        let environment = Environment::global();
        let symbol = Symbol::from(name);

        let result = environment.assign(symbol, value);

        prop_assert_eq!(result, Err(EnvironmentError::UndefinedVariable(symbol)));
    }
}

#[test]
fn can_create_new_local_environment_that_is_enclosed_by_the_current_one() {
    let global_env = Environment::global();
    global_env.define("foo", "some value");

    let local_env = global_env.new_local();
    local_env.define("bar", "another value");

    assert_that!(local_env.lookup("foo")).is_equal_to(Ok(Value::from("some value")));
    assert_that!(local_env.lookup("bar")).is_equal_to(Ok(Value::from("another value")));
}

#[test]
fn a_new_local_environment_shadows_a_variable_with_same_name_from_the_enclosing_environment() {
    let global_env = Environment::global();
    global_env.define("foo", "some value");
    global_env.define("bar", "another value");

    let local_env = global_env.new_local();
    local_env.define("foo", "different value");
    local_env.define("baz", "yet another value");

    assert_that!(local_env.lookup("foo")).is_equal_to(Ok(Value::from("different value")));
    assert_that!(local_env.lookup("bar")).is_equal_to(Ok(Value::from("another value")));
    assert_that!(local_env.lookup("baz")).is_equal_to(Ok(Value::from("yet another value")));
}

#[test]
fn can_assign_to_a_variable_defined_in_the_enclosing_environment() {
    let global_env = Environment::global();
    global_env.define("foo", "some value");

    let local_env = global_env.new_local();
    local_env.define("bar", "another value");

    let assign_result = local_env.assign("foo", "different value");
    assert_that!(assign_result).is_ok();

    assert_that!(local_env.lookup("foo")).is_equal_to(Ok(Value::from("different value")));
    assert_that!(local_env.lookup("bar")).is_equal_to(Ok(Value::from("another value")));
}

#[test]
fn dropping_a_local_environment_does_not_affect_the_enclosing_environment() {
    let global_env = Environment::global();
    global_env.define("foo", "some value");
    global_env.define("bar", "another value");

    let local_env = global_env.new_local();
    local_env.define("foo", "different value");
    local_env.define("baz", "yet another value");

    assert_that!(local_env.lookup("foo")).is_equal_to(Ok(Value::from("different value")));
    assert_that!(local_env.lookup("bar")).is_equal_to(Ok(Value::from("another value")));

    drop(local_env);

    assert_that!(global_env.lookup("foo")).is_equal_to(Ok(Value::from("some value")));
    assert_that!(global_env.lookup("bar")).is_equal_to(Ok(Value::from("another value")));
    assert_that!(global_env.lookup("baz"))
        .is_equal_to(Err(EnvironmentError::UndefinedVariable("baz".into())));
}

#[test]
fn creating_a_new_local_environment_after_dropping_the_last_one() {
    let global_env = Environment::global();
    global_env.define("foo", "some value");
    global_env.define("bar", "another value");

    assert_that!(global_env.lookup("foo")).is_equal_to(Ok(Value::from("some value")));
    assert_that!(global_env.lookup("bar")).is_equal_to(Ok(Value::from("another value")));
    assert_that!(global_env.lookup("baz"))
        .is_equal_to(Err(EnvironmentError::UndefinedVariable("baz".into())));

    let local_env = global_env.new_local();
    local_env.define("foo", "different value");
    local_env.define("baz", "yet another value");

    assert_that!(local_env.lookup("foo")).is_equal_to(Ok(Value::from("different value")));
    assert_that!(local_env.lookup("bar")).is_equal_to(Ok(Value::from("another value")));
    assert_that!(local_env.lookup("baz")).is_equal_to(Ok(Value::from("yet another value")));

    drop(local_env);

    assert_that!(global_env.lookup("foo")).is_equal_to(Ok(Value::from("some value")));
    assert_that!(global_env.lookup("bar")).is_equal_to(Ok(Value::from("another value")));
    assert_that!(global_env.lookup("baz"))
        .is_equal_to(Err(EnvironmentError::UndefinedVariable("baz".into())));

    let local_env = global_env.new_local();
    local_env.define("foo", "completely different value");
    local_env.define("qux", "third value");

    assert_that!(local_env.lookup("foo"))
        .is_equal_to(Ok(Value::from("completely different value")));
    assert_that!(local_env.lookup("bar")).is_equal_to(Ok(Value::from("another value")));
    assert_that!(local_env.lookup("baz"))
        .is_equal_to(Err(EnvironmentError::UndefinedVariable("baz".into())));
    assert_that!(local_env.lookup("qux")).is_equal_to(Ok(Value::from("third value")));
}
