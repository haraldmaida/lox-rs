use std::ffi::OsStr;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};

const TESTSUITE_DIR: &str = "../craftinginterpreters/test/";

fn main() {
    generate_test_suite_from_craftinginterpreters();
}

fn generate_test_suite_from_craftinginterpreters() {
    println!("cargo:rerun-if-changed={TESTSUITE_DIR}");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    let testcases_filepath = Path::new(&out_dir).join("generated_lox_tests.rs");

    let testsuite_root_path = Path::new(TESTSUITE_DIR);

    let mut test_code = String::new();
    //  scan all test sources excluding benchmark tests
    for source_file in scan_for_lox_sources(testsuite_root_path)
        .filter(|path| !path.to_str().unwrap_or("").contains("benchmark"))
    {
        // determine test case name
        let relative_path = source_file
            .strip_prefix(testsuite_root_path)
            .expect("failed to determine relative path of test file");
        let test_name = relative_path
            .to_string_lossy()
            .replace(['\\', '/', '.', '-', ' ', ':'], "_");

        // generate Rust code for this test case
        let source_filename = source_file
            .to_str()
            .expect("failed to convert source file path to string");
        write!(
            test_code,
            r"
#[test]
fn {test_name}() {{
    run_one_lox_test({source_filename:?});
}}
"
        )
        .expect("failed to write test code");
    }

    fs::write(&testcases_filepath, test_code).unwrap_or_else(|err| {
        panic!(
            "failed to write test suite to file '{}': {err}",
            testcases_filepath.display()
        )
    });
}

fn scan_for_lox_sources(root_path: impl AsRef<Path>) -> LoxSources {
    LoxSources {
        to_scan: vec![root_path.as_ref().to_owned()],
    }
}

struct LoxSources {
    to_scan: Vec<PathBuf>,
}

impl Iterator for LoxSources {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(path) = self.to_scan.pop() {
            if path.is_file() && path.extension() == Some(OsStr::new("lox")) {
                return Some(path);
            } else if path.is_dir() {
                self.to_scan.extend(
                    fs::read_dir(&path)
                        .unwrap_or_else(|err| {
                            panic!(
                                "failed to scan folder '{}' for lox sources: {err}",
                                path.display()
                            )
                        })
                        .map(|entry| {
                            entry
                                .unwrap_or_else(|err| panic!("failed to read folder entry: {err}"))
                                .path()
                        }),
                );
            }
        }
        None
    }
}
