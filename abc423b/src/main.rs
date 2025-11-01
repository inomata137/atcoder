#[allow(unused)]
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        l: [u8; n]
    };
    let mut ans = n - 1;
    for lock_state in &l {
        if *lock_state == 0 && ans > 0 {
            ans -= 1;
        } else {
            break;
        }
    }
    for lock_state in l.iter().rev() {
        if *lock_state == 0 && ans > 0 {
            ans -= 1;
        } else {
            break;
        }
    }
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
