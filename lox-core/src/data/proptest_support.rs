use super::*;
use proptest::arbitrary::{Arbitrary, any};
use proptest::prop_oneof;
use proptest::strategy::{BoxedStrategy, Just, Strategy};

impl Arbitrary for Value {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(Self::Nil),
            any::<bool>().prop_map(Self::Bool),
            any::<f64>().prop_map(Self::Number),
            any::<String>().prop_map(Self::String)
        ]
        .boxed()
    }

    type Strategy = BoxedStrategy<Self>;
}
