#[allow(unused)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize
    };
    let ans = x.checked_sub(y * z).is_some_and(|a| a % (z - 1) == 0);
    if ans {
        println!("Yes")
    } else {
        println!("No")
    }
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
