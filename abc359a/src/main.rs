use std::fmt::Display;
use proconio::*;

fn main() {
    input! {
        n: usize
    };
    let mut ans = 0;
    for _ in 0..n {
        input! {
            ch: String
        }
        if ch == "Takahashi" {
            ans += 1;
        }
    }
    println!("{}", ans)
}

#[allow(unused)]
fn print_vec<T: Display>(v: &Vec<T>) {
    if v.is_empty() {
        return;
    }
    print!("{}", v[0]);
    for e in &v[1..] {
        print!(" {}", e);
    }
    println!();
}
