use proconio::*;
use std::fmt::Display;

fn main() {
    input! {
        n: usize,
        mut m: usize,
        h: [usize; n]
    };
    for (i, h) in h.iter().enumerate().take(n) {
        if m >= *h {
            m -= h;
        } else {
            println!("{}", i);
            return;
        }
    }
    println!("{}", n);
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
