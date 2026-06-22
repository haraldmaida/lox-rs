use super::*;
use asserting::prelude::*;

#[test]
fn a_new_resolver_starts_with_an_empty_scopes_stack() {
    let resolver = Resolver::default();

    assert_that!(resolver.scopes).is_empty();
}
