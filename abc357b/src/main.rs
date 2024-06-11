use std::fmt::Display;
use proconio::*;

fn main() {
    input! {
        s: marker::Chars
    };
    let len = s.len();
    let mut upper = 0;
    for c in &s {
        if c.is_uppercase() {
            upper += 1;
        }
    }
    let lower = len - upper;
    if upper > lower {
        for c in s {
            print!("{}", c.to_ascii_uppercase());
        }
    } else {
        for c in s {
            print!("{}", c.to_ascii_lowercase());
        }
    }
    println!();
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
