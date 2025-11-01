#[allow(unused)]
use proconio::{input, marker::*};

fn main() {
    input! {
        q: usize
    };

    let mut levels = Vec::<(isize, bool)>::new();

    for _ in 0..q {
        input! {
            query: u8
        }
        match query {
            1 => solve1(&mut levels),
            2 => solve2(&mut levels),
            _ => unreachable!(),
        }
        let level = levels.last().copied().unwrap_or((0, true));
        println!("{}", if level == (0, true) { "Yes" } else { "No" })
    }
}

fn solve1(levels: &mut Vec<(isize, bool)>) {
    input! {
        c: char
    }
    let prev = levels.last().copied().unwrap_or((0, true));
    let next = match c {
        '(' => (prev.0 + 1, prev.1),
        ')' => (prev.0 - 1, prev.1 && prev.0 > 0),
        _ => unreachable!(),
    };
    levels.push(next);
}

fn solve2(levels: &mut Vec<(isize, bool)>) {
    levels.pop();
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
