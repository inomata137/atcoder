#[allow(unused)]
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    };

    let mut a_counts = vec![0; n + 1];
    let mut b_counts = vec![0; n + 1];
    for i in 0..n {
        match &s[i] {
            'a' => {
                a_counts[i + 1] = a_counts[i] + 1;
                b_counts[i + 1] = b_counts[i];
            }
            'b' => {
                a_counts[i + 1] = a_counts[i];
                b_counts[i + 1] = b_counts[i] + 1;
            }
            _ => unreachable!(),
        }
    }

    let mut ans = 0;
    for l in 1..=n {
        if a_counts[n] - a_counts[l - 1] < a {
            continue;
        }
        let rl = lower_bound(l, n, |r| a_counts[r] - a_counts[l - 1] >= a);
        if b_counts[rl] - b_counts[l - 1] >= b {
            continue;
        }
        let rr = upper_bound(rl, n, |r| b_counts[r] - b_counts[l - 1] < b);
        ans += rr + 1 - rl;
    }

    println!("{ans}")
}

/// Returns the largest usize that satisfies the predicate.
#[allow(unused)]
fn upper_bound(mut left: usize, right: usize, predicate: impl Fn(usize) -> bool) -> usize {
    debug_assert!(predicate(left));

    if predicate(right) {
        return right;
    }

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

/// Returns the smallest usize that satisfies the predicate.
#[allow(unused)]
fn lower_bound(mut left: usize, right: usize, predicate: impl Fn(usize) -> bool) -> usize {
    debug_assert!(predicate(right));

    if predicate(left) {
        return left;
    }

    let mut size = right - left;
    while size > 1 {
        let mid = left + size / 2;
        if predicate(mid) {
            size /= 2;
        } else {
            left = mid;
            size -= size / 2;
        }
    }
    left + size
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
    use super::upper_bound;

    #[test]
    fn test_binary_search() {
        let predicate = |x: usize| x < 5;
        assert_eq!(upper_bound(0, 10, predicate), 4);
        assert_eq!(upper_bound(0, 5, predicate), 4);
        assert_eq!(upper_bound(3, 10, predicate), 4);
    }
}
