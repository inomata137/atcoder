#[allow(unused)]
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n]
    };

    let mut sum1 = a.clone();
    for i in 1..n {
        sum1[i] += sum1[i - 1];
    }
    let mut sum2 = a.clone();
    for i in (0..n - 1).rev() {
        sum2[i] += sum2[i + 1];
    }

    let mut head = 0;

    for _ in 0..q {
        input! {
            query_type: usize
        }
        if query_type == 1 {
            input! {
                c: usize
            }
            head = (head + c) % n;
        } else {
            input! {
                l: Usize1,
                r: Usize1
            }
            let l = (l + head) % n;
            let r = (r + head) % n;

            if r >= l {
                println!("{}", sum1[r] - sum1[l] + a[l])
            } else {
                println!("{}", sum2[l] + sum1[r])
            }
        }
    }
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
