#[allow(unused)]
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    let mut a = Vec::<usize>::with_capacity(n + 1);
    let mut fa = Vec::<usize>::with_capacity(n + 1);

    a.push(1);
    fa.push(1);

    for _ in 0..n {
        let a_i = fa.iter().sum();
        a.push(a_i);
        fa.push(f(a_i));
    }

    println!("{}", a[n]);
}

fn f(mut x: usize) -> usize {
    let mut ans = 0;
    while x > 0 {
        ans += x % 10;
        x /= 10;
    }
    ans
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
