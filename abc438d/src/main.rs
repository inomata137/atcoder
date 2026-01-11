#[allow(unused)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
        mut b: [isize; n],
        mut c: [isize; n],
    };
    let bsum: isize = b.iter().sum();
    a[0] -= b[0];
    for i in 1..n {
        a[i] += a[i - 1] - b[i];
    }
    c[n - 1] -= b[n - 1];
    for i in (0..n - 1).rev() {
        c[i] += c[i + 1] - b[i];
    }
    let f = |x: usize, y: usize| bsum + a[x] + c[y + 1];

    let mut ans_x = 0;
    let mut ans = f(0, 1);
    for y in 2..n - 1 {
        if a[y - 1] > a[ans_x] {
            ans_x = y - 1;
        }
        ans = ans.max(f(ans_x, y));
    }
    println!("{ans}")
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
