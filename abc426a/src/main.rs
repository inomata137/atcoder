#[allow(unused)]
use proconio::{input, marker::*};

fn main() {
    input! {
        x: String,
        y: String
    };

    let x = match x.as_str() {
        "Ocelot" => 0,
        "Serval" => 1,
        "Lynx" => 2,
        _ => unreachable!(),
    };

    let y = match y.as_str() {
        "Ocelot" => 0,
        "Serval" => 1,
        "Lynx" => 2,
        _ => unreachable!(),
    };

    let ans = if x >= y { "Yes" } else { "No" };
    println!("{ans}")
}

/// Returns the largest usize that satisfies the predicate.
#[allow(unused)]
fn binary_search(mut left: usize, right: usize, predicate: impl Fn(usize) -> bool) -> usize {
    debug_assert!(predicate(left));
    debug_assert!(!predicate(right));

    let mut size = right - left;
    while size > 1 {
        let mid = left + size / 2;
        if predicate(mid) {
            left = mid;
            size -= size / 2;
        } else {
            size /= 2;
        }
    }
    left
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn test_binary_search() {
        let predicate = |x: usize| x < 5;
        assert_eq!(binary_search(0, 10, predicate), 4);
        assert_eq!(binary_search(0, 5, predicate), 4);
        assert_eq!(binary_search(3, 10, predicate), 4);
    }
}

#[allow(unused)]
fn print_vec<T: std::fmt::Display>(v: &[T]) {
    if !v.is_empty() {
        print!("{}", v[0]);
        for e in &v[1..] {
            print!(" {}", e);
        }
    }
    println!();
}
