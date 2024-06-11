use std::fmt::Display;
use std::collections::VecDeque;
use proconio::*;

const MOD: usize = 998244353;

fn to_binary(n: usize) -> VecDeque<bool> {
    let mut res = VecDeque::new();
    let mut n = n;
    while n > 0 {
        res.push_back(n % 2 == 1);
        n >>= 1;
    }
    res
}

fn main() {
    input! {
        n: usize
    };
    let len = n.to_string().len();
    let a = 10usize.pow(len.try_into().unwrap());
    let b = (a - 1) % MOD;
    let c = n % MOD;
    let mut r = 0usize;
    let mut bin = to_binary(n);
    while let Some(d) = bin.pop_back() {
        r = (r * ((b * r) % MOD + 2)) % MOD;
        if d {
            r = ((b + 1) * r + 1) % MOD;
        }
    }
    println!("{}", (r * c) % MOD);
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
