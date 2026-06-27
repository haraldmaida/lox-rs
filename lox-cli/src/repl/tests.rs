use super::*;
use asserting::prelude::*;
use std::io::Cursor;

fn run_repl(input: &str) -> (String, String) {
    let stdin_simulator = Cursor::new(input);
    let mut stdout_capture = Vec::new();
    let mut stderr_capture = Vec::new();

    run(stdin_simulator, &mut stdout_capture, &mut stderr_capture)
        .unwrap_or_else(|err| panic!("I/O error running the REPL: {err}"));

    let output = String::from_utf8(stdout_capture).unwrap_or_else(|err| {
        format!("output-stream of REPL contains invalid UTF-8 characters: {err}")
    });
    let error = String::from_utf8(stderr_capture).unwrap_or_else(|err| {
        format!("error-stream of REPL contains invalid UTF-8 characters: {err}")
    });

    (output, error)
}

#[test]
fn starting_and_quitting_the_repl() {
    let input = ":quit";

    let (stdout, stderr) = run_repl(input);

    assert_that!(stdout).is_empty();
    assert_that!(stderr).is_equal_to(
        r"Welcome to the Lox REPL Version 1.0.0!
Enter :quit to exit.
Enter :clear to reset the interpreter.
>> :: Quitting the REPL. Goodbye!
",
    );
}

#[test]
fn empty_input_is_ignored_and_prints_a_new_prompt() {
    let input = "\n:quit\n";

    let (stdout, stderr) = run_repl(input);

    assert_that!(stdout).is_empty();
    assert_that!(stderr).is_equal_to(
        r"Welcome to the Lox REPL Version 1.0.0!
Enter :quit to exit.
Enter :clear to reset the interpreter.
>> >> :: Quitting the REPL. Goodbye!
",
    );
}

#[test]
fn define_a_variable_and_print_it() {
    let input = r#"var hello = "Hello!";
print hello;
:quit
"#;

    let (stdout, stderr) = run_repl(input);

    assert_that!(stderr).is_equal_to(
        r"Welcome to the Lox REPL Version 1.0.0!
Enter :quit to exit.
Enter :clear to reset the interpreter.
>> >> >> :: Quitting the REPL. Goodbye!
",
    );
    assert_that!(stdout).is_equal_to("Hello!\n");
}

#[test]
fn meta_command_clear_resets_the_interpreter() {
    let input = r#"var hello = "Hello!";
print hello;
:clear
print hello;
:quit
"#;

    let (stdout, stderr) = run_repl(input);

    assert_that!(stderr).is_equal_to(
        r"Welcome to the Lox REPL Version 1.0.0!
Enter :quit to exit.
Enter :clear to reset the interpreter.
>> >> >> :: the state of the interpreter has been cleared.
>> use of undefined variable 'hello'
>> :: Quitting the REPL. Goodbye!
",
    );
    assert_that!(stdout).is_equal_to("Hello!\n");
}
