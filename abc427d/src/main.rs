#[allow(unused)]
use proconio::{input, marker::*};

fn main() {
    input! {
        t: usize
    };
    for _ in 0..t {
        solve()
    }
}

fn solve() {
    input! {
        n: usize, // ノード数
        m: usize, // エッジ数
        k: usize, // 操作数
        s: Chars,
        uv: [(Usize1, Usize1); m]
    }

    let uv = uv.into_iter().fold(
        std::collections::BTreeMap::<usize, Vec<usize>>::new(),
        |mut m, (u, v)| {
            m.entry(u).or_default().push(v);
            m
        },
    );

    let mut dp = s;
    for _ in 1..=k {
        dp = (0..n)
            .map(|j| {
                let bob_wins = uv
                    .get(&j)
                    .unwrap()
                    .iter()
                    .all(|k| uv.get(k).unwrap().iter().any(|l| dp[*l] == 'B'));
                if bob_wins {
                    'B'
                } else {
                    'A'
                }
            })
            .collect();
    }
    let winner = match dp[0] {
        'A' => "Alice",
        'B' => "Bob",
        _ => unreachable!(),
    };
    println!("{winner}")
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
