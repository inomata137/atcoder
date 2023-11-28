use proconio::*;

fn main() {
    input! {
        a: usize
    };
    let msg = if a % 2 == 0 { "Even" } else { "Odd" };
    println!("{msg}");
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
