// workaround for false positive 'unused crate dependencies' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
mod fixtures;

use asserting::prelude::*;
use lox_core::interpreter::Interpreter;
use lox_core::parse::Parse;
use lox_core::program::IntoProgram;
use lox_core::resolver::Resolve;
use lox_core::runtime::RuntimeContext;
use lox_core::tokenize::Tokenize;
use std::fs;

fn run_with_interpreter(source_code: &str) -> (Vec<u8>, Vec<u8>) {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut rtc = RuntimeContext::new(&mut stdout, &mut stderr);

    let statements = match source_code.tokenize().parse() {
        Ok(statements) => statements,
        Err(syntax_errors) => {
            for error in syntax_errors {
                rtc.write_error(error);
            }
            return (stdout, stderr);
        },
    };
    let program = match statements.resolve().into_program() {
        Ok(program) => program,
        Err(resolution_errors) => {
            for error in resolution_errors {
                rtc.write_error(error);
            }
            return (stdout, stderr);
        },
    };

    let mut interpreter = Interpreter::default();
    interpreter.interpret(&mut rtc, &program);

    (stdout, stderr)
}

fn extract_expected_output(keyword: &str, test_source: &str) -> String {
    let keyword_len = keyword.len();
    test_source
        .lines()
        .filter_map(|line| {
            line.find(keyword)
                .map(|index| line[index + keyword_len..].trim())
        })
        .fold(String::new(), |mut output, line| {
            output.push_str(line);
            output.push('\n');
            output
        })
}

fn run_one_lox_test(source_file: impl AsRef<std::path::Path>) {
    let source_file = source_file.as_ref();
    let source_code = fs::read_to_string(source_file).unwrap_or_else(|err| {
        panic!(
            "failed to read lox source file '{}': {err}",
            source_file.display()
        )
    });

    let expected_output = extract_expected_output("// expect:", &source_code);
    let expected_errors = extract_expected_output("// Error", &source_code);

    let (stdout, stderr) = run_with_interpreter(&source_code);

    //TODO extraction of expected errors does not work yet
    let stdout = String::from_utf8(stdout);
    if stdout.as_ref() != Ok(&expected_output) {
        assert_that!(String::from_utf8(stderr))
            .named("stderr")
            .ok()
            .is_equal_to(expected_errors);
    }
    assert_that!(stdout)
        .named("stdout")
        .ok()
        .is_equal_to(expected_output);
}

include!(concat!(env!("OUT_DIR"), "/generated_lox_tests.rs"));
