use std::fmt::Display;
use std::collections::BTreeSet;
use proconio::*;

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
