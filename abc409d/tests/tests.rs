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
            r#"3
7
atcoder
1
x
5
snuke
"#,
            r#"acodert
x
nsuke
"#,
        );
    }
}
