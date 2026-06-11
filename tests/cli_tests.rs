// workaround for false positive 'unused crate dependencies' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
mod fixtures;

#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("tests/cli/**/*.toml")
        .default_bin_name("lox")
        .run();
}
