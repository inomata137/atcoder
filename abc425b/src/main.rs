#[allow(unused)]
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        a: [isize; n]
    };

    let mut p = a.clone();

    let mut placeholders = a
        .iter()
        .enumerate()
        .filter_map(|(idx, val)| if *val == -1 { Some(idx) } else { None })
        .collect::<std::collections::VecDeque<_>>();

    for val in 1..=n as isize {
        let pos = a.iter().enumerate().find_map(
            |(idx, a_val)| {
                if *a_val == val {
                    Some(idx)
                } else {
                    None
                }
            },
        );
        if pos.is_none() {
            if let Some(pos) = placeholders.pop_front() {
                p[pos] = val
            } else {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
    print_vec(&p);
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
