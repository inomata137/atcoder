mod test {
    #[test]
    fn sample1() {
        test(include_str!("1.in"), include_str!("1.out"));
    }

    #[test]
    fn sample2() {
        test(include_str!("2.in"), include_str!("2.out"));
    }

    fn test(input: &str, answer: &str) {
        const KEY: &str = "CARGO_PKG_NAME";

        let bin_name = std::env::var(KEY).unwrap_or_else(|_| panic!("No variable found: {KEY}"));
        let testdir = cli_test_dir::TestDir::new(&bin_name, "");
        let cmd = &mut testdir.cmd();
        let result = cli_test_dir::CommandExt::output_with_stdin(cmd, input);
        let output = cli_test_dir::ExpectStatus::expect_success(result);

        assert_eq!(cli_test_dir::OutputExt::stdout_str(&output), answer);
        assert!(cli_test_dir::OutputExt::stderr_str(&output).is_empty());
    }
}
