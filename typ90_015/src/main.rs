#[allow(unused)]
use ac_library::ModInt1000000007 as Mint;
#[allow(unused)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

fn main() {
    input! {
        n: usize
    };

    for k in 1..=n {
        solve(n, k);
    }
}

fn solve(n: usize, k: usize) {
    // a(m, k) = a(m - k, k) + a(m - 1, k)
    // a(m<=k, k) = m + 1

    // [a(k+1, k), a(k+2, k), ..., a(n, k)]
    // a[i] = a(i + k + 1, k)
    // a(m, k) = a[m - k - 1, k]
    let mut a = Vec::<Mint>::with_capacity(n - k);
    for i in 0..n - k {
        // a[i] = a(i + k + 1, k) = a(i + 1, k) + a(i + k, k)
        // a(i + 1, k) = a[i - k]
        // a(i + k, k) = a[i - 1]
        let v1 = if i >= k { a[i - k] } else { Mint::new(i + 2) };
        let v2 = if i >= 1 {
            a[i - 1]
        } else {
            Mint::new(i + k + 1)
        };
        a.push(v1 + v2);
    }

    let ans = a.last().copied().unwrap_or(Mint::new(n + 1));

    println!("{}", ans - 1)
}

/// Returns the largest usize that satisfies the predicate.
/// left and right bounds are inclusive.
#[allow(unused)]
fn upper_bound(mut left: usize, right: usize, predicate: impl Fn(usize) -> bool) -> Option<usize> {
    if !predicate(left) {
        return None;
    }

    if predicate(right) {
        return Some(right);
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
    Some(left)
}

/// Returns the smallest usize that satisfies the predicate.
/// left and right bounds are inclusive.
#[allow(unused)]
fn lower_bound(mut left: usize, right: usize, predicate: impl Fn(usize) -> bool) -> Option<usize> {
    if !predicate(right) {
        return None;
    }

    if predicate(left) {
        return Some(left);
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
    Some(left + size)
}

#[allow(unused)]
fn print_vec<T: std::fmt::Display>(v: &[T]) {
    if !v.is_empty() {
        print!("{}", v[0]);
        for e in &v[1..] {
            print!(" {e}");
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::{lower_bound, upper_bound};

    #[test]
    fn test_upper_bound() {
        let predicate = |x: usize| x < 5;

        assert_eq!(upper_bound(0, 3, predicate), Some(3));
        assert_eq!(upper_bound(0, 4, predicate), Some(4));
        assert_eq!(upper_bound(0, 5, predicate), Some(4));
        assert_eq!(upper_bound(0, 6, predicate), Some(4));

        assert_eq!(upper_bound(3, 10, predicate), Some(4));
        assert_eq!(upper_bound(4, 10, predicate), Some(4));
        assert_eq!(upper_bound(5, 10, predicate), None);
        assert_eq!(upper_bound(6, 10, predicate), None);
    }

    #[test]
    fn test_lower_bound() {
        let predicate = |x: usize| x >= 5;

        assert_eq!(lower_bound(0, 3, predicate), None);
        assert_eq!(lower_bound(0, 4, predicate), None);
        assert_eq!(lower_bound(0, 5, predicate), Some(5));
        assert_eq!(lower_bound(0, 6, predicate), Some(5));

        assert_eq!(lower_bound(3, 10, predicate), Some(5));
        assert_eq!(lower_bound(4, 10, predicate), Some(5));
        assert_eq!(lower_bound(5, 10, predicate), Some(5));
        assert_eq!(lower_bound(6, 10, predicate), Some(6));
    }
}
