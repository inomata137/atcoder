use cli_test_dir::*;
use std::env;

const KEY: &'static str = "CARGO_PKG_NAME";

fn test(input: &str, answer: &str) {
    let bin_name = env::var(KEY).expect(&format!("No variable found: {KEY}"));
    let testdir = TestDir::new(&bin_name, "");
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .expect_success();
    assert_eq!(output.stdout_str(), answer);
    assert!(output.stderr_str().is_empty());
}

mod test {
use crate::test;

#[test]
fn sample1() {
test(
r#"5 60
60 20 100 90 40
"#,
r#"3
"#);
}

#[test]
fn sample2() {
test(
r#"4 80
79 78 77 76
"#,
r#"0
"#);
}

#[test]
fn sample3() {
test(
r#"10 50
31 41 59 26 53 58 97 93 23 84
"#,
r#"6
"#);
}
}
