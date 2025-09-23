#[allow(unused)]
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize, // 団体数
        k: usize, // キャパシティ
        visit: [(usize, usize, usize); n],
    };

    let mut empty = k;
    let mut time = 0;

    let mut pool = std::collections::BTreeMap::<usize, usize>::new();

    // (来訪時刻, 滞在時間, 人数)
    for (arrival, stay, num) in visit {
        while empty < num {
            let (leave_time, leave_num) = pool.pop_first().unwrap();
            empty += leave_num;
            time = time.max(leave_time);
        }
        time = time.max(arrival);
        empty -= num;
        println!("{time}");
        *pool.entry(time + stay).or_default() += num;
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
