#[allow(unused)]
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        skills: [(usize, usize); n],
    };

    let mut graph = std::collections::BTreeMap::<usize, std::collections::BTreeSet<usize>>::new();
    let mut q = std::collections::VecDeque::<usize>::new();
    let mut acquired = vec![false; n];

    for (s, (a, b)) in skills.iter().enumerate() {
        if a | b == 0 {
            acquired[s] = true;
            q.push_back(s);
        } else {
            graph.entry(a - 1).or_default().insert(s);
            graph.entry(b - 1).or_default().insert(s);
        }
    }

    while let Some(s) = q.pop_front() {
        let Some(a) = graph.get(&s) else {
            continue;
        };
        for s in a.iter() {
            if !acquired[*s] {
                q.push_back(*s)
            }
            acquired[*s] = true;
        }
    }

    let ans = acquired.iter().filter(|a| **a).count();
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
