#[allow(unused)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

fn main() {
    input! {
        n: usize,
    };
    // key: Coord, value: Dist
    let mut s = std::collections::BTreeMap::<usize, usize>::new();
    s.insert(0, usize::MAX);

    let mut total_dist = usize::MAX;
    for _ in 0..n {
        input! { x: usize };

        let (cl, dl) = s.range_mut(..x).last().unwrap();
        let d = x - *cl;
        if *dl > x - *cl {
            total_dist -= *dl - d;
            *dl = d;
        }

        let mut d = d;

        let r = s.range_mut(x..).next();
        if let Some((cr, dr)) = r {
            let new_dist = *cr - x;
            if *dr > *cr - x {
                total_dist -= *dr - new_dist;
                *dr = new_dist;
            }
            d = d.min(new_dist);
        }

        s.insert(x, d);
        total_dist += d;

        println!("{}", total_dist)
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
