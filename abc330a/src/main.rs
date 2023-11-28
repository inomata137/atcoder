use proconio::*;

fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize; n]
    };
    let mut count = 0;
    for p in a {
        if p >= l {
            count += 1;
        }
    }
    println!("{count}")
}
