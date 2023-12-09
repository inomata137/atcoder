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
r#"3 7
1 2 2 3 1 3 3
"#,
r#"1
1
2
2
1
1
3
"#);
}

#[test]
fn sample2() {
test(
r#"100 5
100 90 80 70 60
"#,
r#"100
90
80
70
60
"#);
}

#[test]
fn sample3() {
test(
r#"9 8
8 8 2 2 8 8 2 2
"#,
r#"8
8
8
2
8
8
8
2
"#);
}
}
