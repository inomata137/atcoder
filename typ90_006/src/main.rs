#[allow(unused)]
use proconio::{input, marker::*};

const OFFSET: usize = 'a' as usize;

fn main() {
    input! {
        n: usize, k: usize,
        s: Chars
    };
    let mut chars = vec![std::collections::VecDeque::<usize>::new(); 26];
    for (i, &c) in s[..n - k].iter().enumerate() {
        chars[c as usize - OFFSET].push_back(i);
    }

    let mut idx;
    for i in 0..k {
        chars[s[n - k + i] as usize - OFFSET].push_back(n - k + i);
        idx = chars.iter().flatten().copied().next().unwrap();
        chars.iter_mut().for_each(|indices| loop {
            match indices.front() {
                Some(&i) if i <= idx => {
                    indices.pop_front();
                }
                _ => break,
            }
        });
        print!("{}", s[idx]);
    }

    println!()
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
