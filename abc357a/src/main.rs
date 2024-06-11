use std::fmt::Display;
use proconio::*;

fn main() {
    input! {
        n: usize,
        mut m: usize,
        h: [usize; n]
    };
    for i in 0..n {
        if m >= h[i] {
            m -= h[i];
        } else {
            println!("{}", i);
            return;
        }
    }
    println!("{}", n);
}

#[allow(unused)]
fn print_vec<T: Display>(v: &Vec<T>) {
    if v.len() == 0 {
        return;
    }
    print!("{}", v[0]);
    for e in &v[1..] {
        print!(" {}", e);
    }
    println!();
}
