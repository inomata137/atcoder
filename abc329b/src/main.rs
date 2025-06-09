use proconio::*;
use std::collections::BTreeSet;
use std::fmt::Display;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };
    let mut bs: BTreeSet<usize> = BTreeSet::new();
    for e in a {
        bs.insert(e);
    }
    bs.pop_last();
    println!("{}", bs.last().unwrap())
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
