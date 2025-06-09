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
            r#"5
3 1 4 1 5
"#,
            r#"4 5 13 14 26
"#,
        );
    }

    #[test]
    fn sample2() {
        test(
            r#"6
1000000000 1000000000 1000000000 1000000000 1000000000 1000000000
"#,
            r#"1000000001 2000000001 3000000001 4000000001 5000000001 6000000001
"#,
        );
    }

    #[test]
    fn sample3() {
        test(
            r#"15
748 169 586 329 972 529 432 519 408 587 138 249 656 114 632
"#,
            r#"749 918 1921 2250 4861 5390 5822 6428 6836 7796 7934 8294 10109 10223 11373
"#,
        );
    }
}
