use proconio::*;
use std::fmt::Display;

fn main() {
    input! {
        sx: usize,
        sy: usize,
        tx: usize,
        ty: usize,
    };
    let ans_y = sy.abs_diff(ty);
    let (start_left, start_right) = if (sx + sy) % 2 == 0 {
        (sx, sx + 1)
    } else {
        (sx - 1, sx)
    };
    let xmin = if start_left > ans_y {
        start_left - ans_y
    } else {
        0
    };
    let xmax = start_right + ans_y;
    let ans_x = if tx < xmin {
        (xmin - tx + 1) / 2
    } else if tx > xmax {
        (tx - xmax + 1) / 2
    } else {
        0
    };
    println!("{}", ans_x + ans_y);
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
