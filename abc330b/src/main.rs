use proconio::*;
use std::fmt::Display;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        a: [usize; n]
    }
    let result = a.iter().map(|&x| x.clamp(l, r)).collect::<Vec<usize>>();
    print_vec(&result)
}

#[allow(unused)]
fn print_vec<T: Display>(v: &[T]) {
    if v.is_empty() {
        return;
    }
    print!("{}", v[0]);
    for e in &v[1..] {
        print!(" {}", e)
    }
    println!()
}
