use std::fmt::Display;
use proconio::*;

const O: char = 'o';

fn main() {
    input! {
        n: usize,
        s: [marker::Chars; n]
    }
    let mut xarr = vec![0usize; n];
    let mut yarr = vec![0usize; n];
    let mut ans = 0usize;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == O {
                xarr[i] += 1;
                yarr[j] += 1;
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == O {
                ans += (xarr[i] - 1) * (yarr[j] - 1)
            }
        }
    }
    println!("{ans}")
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
