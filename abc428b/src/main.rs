#[allow(unused)]
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: String
    };

    let mut count = std::collections::BTreeMap::<&str, usize>::new();
    for i in 0..n + 1 - k {
        let pat = &s[i..i + k];
        *count.entry(pat).or_default() += 1;
    }
    let max = *count.values().max().unwrap();
    println!("{max}");
    let vals = count
        .into_iter()
        .filter_map(|(k, v)| if v == max { Some(k) } else { None })
        .collect::<Vec<_>>();
    print_vec(&vals);
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
