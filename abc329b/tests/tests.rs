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
r#"5
2 1 3 3 2
"#,
r#"2
"#);
}

#[test]
fn sample2() {
test(
r#"4
4 3 2 1
"#,
r#"3
"#);
}

#[test]
fn sample3() {
test(
r#"8
22 22 18 16 22 18 18 22
"#,
r#"18
"#);
}
}
