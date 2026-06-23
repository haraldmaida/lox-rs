use super::*;
use crate::data::Symbol;
use crate::parse::Parse;
use crate::token::{identifier, return_};
use crate::tokenize::Tokenize;
use asserting::prelude::*;
use proptest::prelude::*;

fn parse(source: &str) -> Vec<Stmt> {
    source.tokenize().parse().expect("parsing should succeed")
}

#[test]
fn a_new_resolver_starts_with_an_empty_scopes_stack() {
    let resolver = Resolver::default();

    assert_that!(resolver.scopes).is_empty();
}

proptest! {
    #[test]
    fn begin_scope_pushes_a_new_empty_scope(
        current_depth in 0_usize..=150,
    ) {
        let mut resolver = Resolver::default();
        for _ in 0..current_depth {
            resolver.begin_scope();
        }

        resolver.begin_scope();

        prop_assert_eq!(resolver.scopes.len(), current_depth + 1);
        prop_assert!(resolver.scopes[current_depth].is_empty());
    }

    #[test]
    fn end_scope_pops_the_top_scope(
        current_depth in 1_usize..=150,
    ) {
        let mut resolver = Resolver::default();
        for _ in 0..current_depth {
            resolver.begin_scope();
        }
        resolver.end_scope();

        prop_assert_eq!(resolver.scopes.len(), current_depth - 1);
    }

    #[test]
    fn declare_inserts_the_symbol_into_the_innermost_scope_as_declared(
        current_depth in 1_usize..=150,
    ) {
        let mut resolver = Resolver::default();
        for _ in 0..current_depth {
            resolver.begin_scope();
        }

        resolver.declare(identifier("a", (0, 1))).expect("call to Resolver::declare should succeed");

        prop_assert_eq!(resolver.scopes[current_depth - 1].get(&Symbol::intern("a")), Some(&VarState::Declared));
    }

    #[test]
    fn define_inserts_the_symbol_into_the_innermost_scope_as_initialized(
        current_depth in 1_usize..=150,
    ) {
        let mut resolver = Resolver::default();
        for _ in 0..current_depth {
            resolver.begin_scope();
        }

        resolver.define(identifier("a", (0, 1)));

        prop_assert_eq!(resolver.scopes[current_depth - 1].get(&Symbol::intern("a")), Some(&VarState::Initialized));
    }
}

#[test]
fn resolve_calculates_the_distance_to_a_local_variable() {
    let mut resolver = Resolver::default();
    let statements = parse("{ var a = 1; a; }");

    let resolution_map = resolver
        .resolve(&statements)
        .expect("resolution should succeed");

    assert_that!(resolution_map.get_distance(identifier("a", (13, 1)))).is_equal_to(Some(0));
}

#[test]
fn resolve_calculates_the_distance_to_a_variable_in_an_outer_scope() {
    let mut resolver = Resolver::default();
    let statements = parse("{ var a = 1; { a; } }");

    let resolution_map = resolver
        .resolve(&statements)
        .expect("resolution should succeed");

    assert_that!(resolution_map.get_distance(identifier("a", (15, 1)))).is_equal_to(Some(1));
}

#[test]
fn resolve_handles_shadowing_correctly() {
    let mut resolver = Resolver::default();
    let statements = parse("{ var a = 1; { var a = 2; a; } }");

    let resolution_map = resolver
        .resolve(&statements)
        .expect("resolution should succeed");

    assert_that!(resolution_map.get_distance(identifier("a", (26, 1)))).is_equal_to(Some(0));
}

#[test]
fn resolve_resolves_function_parameters() {
    let mut resolver = Resolver::default();
    let statements = parse("fun foo(a) { a; }");

    let resolution_map = resolver
        .resolve(&statements)
        .expect("resolution should succeed");

    assert_that!(resolution_map.get_distance(identifier("a", (13, 1)))).is_equal_to(Some(0));
    assert_that!(resolution_map.get_distance(identifier("a", (8, 1)))).is_none();
}

#[test]
fn resolve_returns_error_when_reading_local_variable_in_its_own_initializer() {
    let mut resolver = Resolver::default();
    let statements = parse("{ var a = a; }");

    let result = resolver.resolve(&statements);

    assert_that!(result).err().contains_exactly([ResolverError {
        code: ResolverErrorCode::CannotReadLocalVariableInInitializer,
        token: identifier("a", (10, 1)),
        location: (10, 1).into(),
    }]);
}

#[test]
fn resolve_resolves_variables_in_assignments() {
    let mut resolver = Resolver::default();
    let statements = parse("{ var a = 1; a = 2; }");

    let resolution_map = resolver
        .resolve(&statements)
        .expect("resolution should succeed");

    assert_that!(resolution_map.get_distance(identifier("a", (13, 1)))).is_equal_to(Some(0));
}

#[test]
fn resolve_resolves_variables_in_if_statements() {
    let mut resolver = Resolver::default();
    let statements =
        parse("{ var a = true; if (a) { var b = a; print b; } else { var c = a; print c; } }");

    let resolution_map = resolver
        .resolve(&statements)
        .expect("resolution should succeed");

    // condition
    assert_that!(resolution_map.get_distance(identifier("a", (20, 1)))).is_equal_to(Some(0));
    // then branch
    assert_that!(resolution_map.get_distance(identifier("a", (33, 1)))).is_equal_to(Some(1));
    assert_that!(resolution_map.get_distance(identifier("b", (42, 1)))).is_equal_to(Some(0));
    // else branch
    assert_that!(resolution_map.get_distance(identifier("a", (62, 1)))).is_equal_to(Some(1));
    assert_that!(resolution_map.get_distance(identifier("c", (71, 1)))).is_equal_to(Some(0));
}

#[test]
fn resolve_does_not_track_global_variables_in_resolution_map() {
    let mut resolver = Resolver::default();
    let statements = parse("var a = 1; a;");

    let resolution_map = resolver
        .resolve(&statements)
        .expect("resolution should succeed");

    assert_that!(resolution_map.get_distance(identifier("a", (11, 1)))).is_none();
}

#[test]
fn resolve_finds_redeclared_variable_in_same_local_scope() {
    let mut resolver = Resolver::default();
    let statements = parse("fun bad() { var a = \"first\"; var a = \"second\"; }");

    let result = resolver.resolve(&statements);

    assert_that!(result).err().contains_exactly([ResolverError {
        code: ResolverErrorCode::RedeclaredVariableInSameScope,
        token: identifier("a", (33, 1)),
        location: (33, 1).into(),
    }]);
}

#[test]
fn resolve_finds_return_stmt_outside_of_any_function() {
    let mut resolver = Resolver::default();
    let statements = parse("return \"at top level\";");

    let result = resolver.resolve(&statements);

    assert_that!(result).err().contains_exactly([ResolverError {
        code: ResolverErrorCode::CannotReturnFromOutsideFunction,
        token: return_((0, 6)),
        location: (0, 6).into(),
    }]);
}
