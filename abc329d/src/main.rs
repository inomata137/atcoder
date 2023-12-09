use std::{fmt::Display, collections::BTreeSet};
use proconio::{*, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m]
    };
    let mut va = vec![0; n];
    let mut bs: BTreeSet<usize> = BTreeSet::new();
    for i in a {
        bs.remove(&(va[i] * 1_000_000 + n - i));
        va[i] += 1;
        bs.insert(va[i] * 1_000_000 + n - i);
        let candidate = bs.last().unwrap();
        let candidate = n + 1 - candidate % 1_000_000;
        println!("{candidate}");
    }
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
