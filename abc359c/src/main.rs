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
    let xmin = start_left.saturating_sub(ans_y);
    let xmax = start_right + ans_y;
    let ans_x = if tx < xmin {
        (xmin - tx).div_ceil(2)
    } else if tx > xmax {
        (tx - xmax).div_ceil(2)
    } else {
        0
    };
    println!("{}", ans_x + ans_y);
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
