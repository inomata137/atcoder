use std::fmt::Display;
use proconio::*;

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
fn print_vec<T: Display>(v: &Vec<T>) {
    if v.len() == 0 {
        return
    }
    print!("{}", v[0]);
    for e in &v[1..] {
        print!(" {}", e)
    }
    println!()
}
