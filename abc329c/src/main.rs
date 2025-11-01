use proconio::*;
use std::fmt::Display;

fn main() {
    input! {
        n: usize,
        s: marker::Chars
    };
    let mut v = vec![0usize; 26];
    let mut left = 0;
    let mut prev = s[0];
    for (right, el) in s.iter().enumerate().take(n) {
        if *el != prev {
            let idx = char2idx(prev);
            v[idx] = v[idx].max(right - left);
            left = right;
            prev = *el;
        }
    }
    let idx = char2idx(prev);
    v[idx] = v[idx].max(n - left);

    let ans = v.iter().sum::<usize>();
    println!("{ans}")
}

fn char2idx(c: char) -> usize {
    c as usize - 'a' as usize
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
