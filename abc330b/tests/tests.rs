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
r#"5 4 7
3 1 4 9 7
"#,
r#"4 4 4 7 7
"#);
}

#[test]
fn sample2() {
test(
r#"3 10 10
11 10 9
"#,
r#"10 10 10
"#);
}
}
