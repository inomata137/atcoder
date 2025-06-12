use cli_test_dir::*;
use std::env;

const KEY: &str = "CARGO_PKG_NAME";

fn test(input: &str, answer: &str) {
    let bin_name = env::var(KEY).unwrap_or_else(|_| panic!("No variable found: {KEY}"));
    let testdir = TestDir::new(&bin_name, "");
    let output = testdir.cmd().output_with_stdin(input).expect_success();
    assert_eq!(output.stdout_str(), answer);
    assert!(output.stderr_str().is_empty());
}

mod test {
    use crate::test;

    #[test]
    fn sample1() {
        test(
            r#"5 6
4 3 1 2
"#,
            r#"2
"#,
        );
    }

    #[test]
    fn sample2() {
        test(
            r#"4 4
1 1 1
"#,
            r#"0
"#,
        );
    }

    #[test]
    fn sample3() {
        test(
            r#"10 12
4 4 5 7 1 7 0 8 5
"#,
            r#"13
"#,
        );
    }
}
