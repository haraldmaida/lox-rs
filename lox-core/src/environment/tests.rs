use super::*;
use asserting::prelude::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn defining_a_variable_and_looking_it_up_it_returns_the_same_value(
        name in any::<String>(),
        value in any::<Value>(),
    ) {
        let mut environment = Environment::default();

        let symbol = Symbol::from(name);
        environment.define(symbol, value.clone());
        let maybe_value = environment.get(symbol);

        prop_assert_eq!(maybe_value, Ok(&value));
    }

    #[test]
    fn looking_up_a_non_existing_variable_returns_an_error(
        name in any::<String>(),
    ) {
        let environment = Environment::default();
        let symbol = Symbol::from(name);

        let maybe_value = environment.get(symbol);

        prop_assert_eq!(maybe_value, Err(EnvironmentError::UndefinedVariable(symbol)));
    }

    #[test]
    fn assigning_a_value_to_an_existing_variable_and_looking_it_up_returns_the_new_value(
        name in any::<String>(),
        value_before in any::<Value>(),
        value_after in any::<Value>(),
    ) {
        let mut environment = Environment::default();
        let symbol = Symbol::from(name);
        environment.define(symbol, value_before);

        let result = environment.assign(symbol, value_after.clone());

        prop_assert!(result.is_ok());
        let maybe_value = environment.get(symbol);
        prop_assert_eq!(maybe_value, Ok(&value_after));
    }

    #[test]
    fn assigning_a_value_to_a_non_existing_variable_returns_an_error(
        name in any::<String>(),
        value in any::<Value>(),
    ) {
        let mut environment = Environment::default();
        let symbol = Symbol::from(name);

        let result = environment.assign(symbol, value);

        prop_assert_eq!(result, Err(EnvironmentError::UndefinedVariable(symbol)));
    }
}

#[test]
fn can_create_new_scope_enclosing_the_current_one() {
    let mut environment = Environment::default();
    environment.define("foo", "some value");

    environment.create_new_scope();
    environment.define("bar", "another value");

    assert_that!(environment.get("foo")).is_equal_to(Ok(&Value::from("some value")));
    assert_that!(environment.get("bar")).is_equal_to(Ok(&Value::from("another value")));
}

#[test]
fn a_new_scope_shadows_a_variable_with_same_name_from_the_enclosing_scope() {
    let mut environment = Environment::default();
    environment.define("foo", "some value");
    environment.define("bar", "another value");

    environment.create_new_scope();
    environment.define("foo", "different value");
    environment.define("baz", "yet another value");

    assert_that!(environment.get("foo")).is_equal_to(Ok(&Value::from("different value")));
    assert_that!(environment.get("bar")).is_equal_to(Ok(&Value::from("another value")));
    assert_that!(environment.get("baz")).is_equal_to(Ok(&Value::from("yet another value")));
}

#[test]
fn can_assign_to_a_variable_defined_in_the_enclosing_scope() {
    let mut environment = Environment::default();
    environment.define("foo", "some value");

    environment.create_new_scope();
    environment.define("bar", "another value");

    let assign_result = environment.assign("foo", "different value");
    assert_that!(assign_result).is_ok();

    assert_that!(environment.get("foo")).is_equal_to(Ok(&Value::from("different value")));
    assert_that!(environment.get("bar")).is_equal_to(Ok(&Value::from("another value")));
}

#[test]
fn destroying_an_enclosing_scope_does_not_affect_the_enclosed_scope() {
    let mut environment = Environment::default();
    environment.define("foo", "some value");
    environment.define("bar", "another value");

    environment.create_new_scope();
    environment.define("foo", "different value");
    environment.define("baz", "yet another value");

    assert_that!(environment.get("foo")).is_equal_to(Ok(&Value::from("different value")));
    assert_that!(environment.get("bar")).is_equal_to(Ok(&Value::from("another value")));

    environment.destroy_current_scope();

    assert_that!(environment.get("foo")).is_equal_to(Ok(&Value::from("some value")));
    assert_that!(environment.get("bar")).is_equal_to(Ok(&Value::from("another value")));
    assert_that!(environment.get("baz"))
        .is_equal_to(Err(EnvironmentError::UndefinedVariable("baz".into())));
}

#[test]
fn the_root_scope_can_not_be_destroyed() {
    let mut environment = Environment::default();
    environment.define("foo", "some value");
    environment.define("bar", "another value");

    environment.destroy_current_scope();

    assert_that!(environment.get("foo")).is_equal_to(Ok(&Value::from("some value")));
    assert_that!(environment.get("bar")).is_equal_to(Ok(&Value::from("another value")));
}

#[test]
fn creating_a_new_scope_after_destroying_the_last_one() {
    let mut environment = Environment::default();
    environment.define("foo", "some value");
    environment.define("bar", "another value");

    assert_that!(environment.get("foo")).is_equal_to(Ok(&Value::from("some value")));
    assert_that!(environment.get("bar")).is_equal_to(Ok(&Value::from("another value")));
    assert_that!(environment.get("baz"))
        .is_equal_to(Err(EnvironmentError::UndefinedVariable("baz".into())));

    environment.create_new_scope();
    environment.define("foo", "different value");
    environment.define("baz", "yet another value");

    assert_that!(environment.get("foo")).is_equal_to(Ok(&Value::from("different value")));
    assert_that!(environment.get("bar")).is_equal_to(Ok(&Value::from("another value")));
    assert_that!(environment.get("baz")).is_equal_to(Ok(&Value::from("yet another value")));

    environment.destroy_current_scope();
    assert_that!(environment.get("foo")).is_equal_to(Ok(&Value::from("some value")));
    assert_that!(environment.get("bar")).is_equal_to(Ok(&Value::from("another value")));
    assert_that!(environment.get("baz"))
        .is_equal_to(Err(EnvironmentError::UndefinedVariable("baz".into())));

    environment.create_new_scope();
    environment.define("foo", "completely different value");
    environment.define("qux", "third value");

    assert_that!(environment.get("foo"))
        .is_equal_to(Ok(&Value::from("completely different value")));
    assert_that!(environment.get("bar")).is_equal_to(Ok(&Value::from("another value")));
    assert_that!(environment.get("baz"))
        .is_equal_to(Err(EnvironmentError::UndefinedVariable("baz".into())));
    assert_that!(environment.get("qux")).is_equal_to(Ok(&Value::from("third value")));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScopeAction {
    CreateNewScope,
    DestroyCurrentScope,
}

impl Arbitrary for ScopeAction {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        prop_oneof![Just(Self::CreateNewScope), Just(Self::DestroyCurrentScope),].boxed()
    }

    type Strategy = BoxedStrategy<Self>;
}

proptest! {
    #[test]
    fn arbitray_creating_and_destroying_scopes_preserves_the_path_to_the_root_scope(
        actions in prop::collection::vec(any::<ScopeAction>(), 1..100)
    ) {
        let mut environment = Environment::default();
        environment.define("foo", "the value");

        prop_assert_eq!(environment.get("foo"), Ok(&Value::from("the value")));

        for action in actions {
            match action {
                ScopeAction::CreateNewScope => {
                    environment.create_new_scope();
                },
                ScopeAction::DestroyCurrentScope => {
                    environment.destroy_current_scope();
                },
            }
            prop_assert_eq!(environment.get("foo"), Ok(&Value::from("the value")));
        }
    }
}
