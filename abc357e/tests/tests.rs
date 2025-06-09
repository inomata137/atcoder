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
            r#"4
2 1 1 4
"#,
            r#"8
"#,
        );
    }

    #[test]
    fn sample2() {
        test(
            r#"5
2 4 3 1 2
"#,
            r#"14
"#,
        );
    }

    #[test]
    fn sample3() {
        test(
            r#"10
6 10 4 1 5 9 8 6 5 1
"#,
            r#"41
"#,
        );
    }
}
