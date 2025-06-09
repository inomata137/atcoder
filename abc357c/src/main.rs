use proconio::*;
use std::fmt::Display;

fn calc(x: usize, y: usize, level: u32) -> char {
    if level == 0 {
        return '#';
    }
    let block = 3usize.pow(level - 1);
    if x >= block && x < 2 * block && y >= block && y < 2 * block {
        return '.';
    }
    calc(x % block, y % block, level - 1)
}

fn main() {
    input! {
        n: u32
    };
    for i in 0..3usize.pow(n) {
        for j in 0..3usize.pow(n) {
            print!("{}", calc(j, i, n));
        }
        println!();
    }
}

#[allow(unused)]
fn print_vec<T: Display>(v: &Vec<T>) {
    if v.is_empty() {
        return;
    }
    print!("{}", v[0]);
    for e in &v[1..] {
        print!(" {}", e);
    }
    println!();
}
