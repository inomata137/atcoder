#[allow(unused)]
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
    };
    // 累積和
    let mut sum_a = Vec::with_capacity(n + 1);
    sum_a.push(0);
    let mut sum_ai = Vec::with_capacity(n + 1);
    sum_ai.push(0);
    let mut sum_aii = Vec::with_capacity(n + 1);
    sum_aii.push(0);
    for (i, a) in a.iter().enumerate() {
        let i = i as isize;
        sum_a.push(sum_a.last().unwrap() + a);
        sum_ai.push(sum_ai.last().unwrap() + a * i);
        sum_aii.push(sum_aii.last().unwrap() + a * i * i);
    }

    for _ in 0..q {
        solve(&sum_a, &sum_ai, &sum_aii);
    }
}

fn solve(sum_a: &[isize], sum_ai: &[isize], sum_aii: &[isize]) {
    input! {
        mut l: Isize1,
        mut r: Isize1,
    }
    let s1 = (r + 1) * (l - 1) * (sum_a[(r + 1) as usize] - sum_a[l as usize]);
    let s2 = (r + l) * (sum_ai[(r + 1) as usize] - sum_ai[l as usize]);
    let s3 = sum_aii[(r + 1) as usize] - sum_aii[l as usize];

    let ans = -s1 + s2 - s3;

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
