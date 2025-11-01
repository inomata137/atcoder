use marker::Usize1;
use proconio::*;
use std::fmt::Display;

fn main() {
    input! {
        n: usize,
        a: [Usize1; 2 * n]
    };
    let mut ans = 0;
    for i in 2..(2 * n) {
        if a[i - 2] == a[i] {
            ans += 1;
        }
    }
    println!("{}", ans)
}

#[allow(unused)]
fn print_vec<T: Display>(v: &[T]) {
    if v.is_empty() {
        return;
    }
    print!("{}", v[0]);
    for e in &v[1..] {
        print!(" {}", e);
    }
    println!();
}
