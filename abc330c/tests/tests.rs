// use cli_test_dir::*;
// use std::env;

// const KEY: &'static str = "CARGO_PKG_NAME";

// fn test(input: &str, answer: &str) {
//     let bin_name = env::var(KEY).expect(&format!("No variable found: {KEY}"));
//     let testdir = TestDir::new(&bin_name, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(input)
//         .expect_success();
//     assert_eq!(output.stdout_str(), answer);
//     assert!(output.stderr_str().is_empty());
// }

// mod test {
// use crate::test;

// #[test]
// fn sample1() {
// test(
// r#"12
// "#,
// r#"Even
// "#);
// }

// #[test]
// fn sample2() {
// test(
// r#"123
// "#,
// r#"Odd
// "#);
// }

// #[test]
// fn sample3() {
// test(
// r#"0
// "#,
// r#"Even
// "#);
// }
// }
