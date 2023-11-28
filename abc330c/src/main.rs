use std::fmt::Display;
use proconio::*;

fn main() {
    input! {
        d: f64
    };
    let mut x = nearest_root(d).0;
    let mut ans = d;
    while x >= 0f64 {
        let (y, res) = nearest_root(d - x.powi(2));
        if y > x {
            break;
        }
        ans = ans.min(res);
        x -= 1.0;

    }
    println!("{}", ans as usize)
}

fn nearest_root(n: f64) -> (f64, f64) {
    let root = n.sqrt();
    let f = root.floor();
    let c = root.ceil();
    let df = n - f.powi(2);
    let dc = c.powi(2) - n;
    if df > dc {
        (c, dc)
    } else {
        (f, df)
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
