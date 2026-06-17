use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn defining_a_variable_and_looking_it_up_its_returns_the_same_value(
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
