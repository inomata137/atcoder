use proconio::*;

fn main() {
    input! {
        a: usize
    };
    let msg = if a % 2 == 0 { "Even" } else { "Odd" };
    println!("{msg}");
}
