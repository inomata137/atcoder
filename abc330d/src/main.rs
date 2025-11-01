use proconio::*;
use std::fmt::Display;

const O: char = 'o';

fn main() {
    input! {
        n: usize,
        s: [marker::Chars; n]
    }
    let mut xarr = vec![0usize; n];
    let mut yarr = vec![0usize; n];
    let mut ans = 0usize;
    for (i, x) in xarr.iter_mut().enumerate() {
        for (j, y) in yarr.iter_mut().enumerate() {
            if s[i][j] == O {
                *x += 1;
                *y += 1;
            }
        }
    }
    for (i, x) in xarr.iter().enumerate() {
        for (j, y) in yarr.iter().enumerate() {
            if s[i][j] == O {
                ans += (x - 1) * (y - 1)
            }
        }
    }
    println!("{ans}")
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
