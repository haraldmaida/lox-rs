use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn defining_a_variable_and_looking_it_up_its_returns_the_same_value(
        name in any::<String>(),
        value in any::<Value>(),
    ) {
        let mut env = Environment::default();

        let symbol = Symbol::from(name);
        env.define(symbol, value.clone());
        let maybe_value = env.get(symbol);

        prop_assert_eq!(maybe_value, Some(&value));
    }
}
