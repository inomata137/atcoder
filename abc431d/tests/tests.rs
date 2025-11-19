mod test {
    #[test]
    fn sample1() {
        test(include_str!("1.in"), include_str!("1.out"));
    }

    #[test]
    fn sample2() {
        test(include_str!("2.in"), include_str!("2.out"));
    }

    #[test]
    fn sample3() {
        test(include_str!("3.in"), include_str!("3.out"));
    }

    #[test]
    fn sample4() {
        test(include_str!("4.in"), include_str!("4.out"));
    }

    fn test(input: &str, answer: &str) {
        use cli_test_dir::{CommandExt, ExpectStatus, OutputExt};
        const KEY: &str = "CARGO_PKG_NAME";

        let bin_name = std::env::var(KEY).unwrap_or_else(|_| panic!("No variable found: {KEY}"));
        let testdir = cli_test_dir::TestDir::new(&bin_name, "");
        let output = testdir.cmd().output_with_stdin(input).expect_success();
        assert_eq!(output.stdout_str(), answer);
        assert!(output.stderr_str().is_empty());
    }
}
