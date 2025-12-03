#[allow(unused)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

fn main() {
    input! {
        t: usize
    };
    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        m: usize,
        x: [usize; n],
        y: [usize; m]
    }
    let x_to_i = x
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect::<std::collections::BTreeMap<_, _>>();
    if x_to_i.len() < n {
        println!("No");
        return;
    }
    let y_to_i = y
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect::<std::collections::BTreeMap<_, _>>();
    if y_to_i.len() < m {
        println!("No");
        return;
    }
    let mut left = x
        .iter()
        .flat_map(|xv| y.iter().map(move |yv| (*xv.min(yv), *xv, *yv)))
        .collect::<Vec<_>>();
    left.sort_by_key(|v| v.0);

    let mut a = vec![vec![0usize; m]; n];
    let mut pos = Vec::with_capacity(n * m);
    for (i, (m, x, y)) in left.into_iter().enumerate().rev() {
        let v = i + 1;
        if m < v {
            println!("No");
            return;
        }
        let xi = x_to_i[&x];
        let yi = y_to_i[&y];
        a[xi][yi] = v;
        pos.push((xi, yi));
    }
    // ここまでで
    // A_ij <= X_i
    // A_ij <= Y_j
    // が満たされた。あとは
    // ∃j A_ij == X_i
    // ∃i A_ij == Y_j
    // を満たせば良い
    let bounds = std::collections::BTreeSet::from_iter(x.iter().chain(y.iter()));
    for b in bounds.into_iter().rev() {
        if let Some(xi) = x_to_i.get(b) {
            let m = a[*xi].iter().max().unwrap();
            if m < b {
                let p1 = pos[b - 1];
                let p2 = (*xi, p1.1);
                (a[p1.0][p1.1], a[p2.0][p2.1]) = (a[p2.0][p2.1], a[p1.0][p1.1]);
                pos[a[p1.0][p1.1] - 1] = p1;
                pos[a[p2.0][p2.1] - 1] = p2;
            }
        }
        if let Some(yi) = y_to_i.get(b) {
            let m = a.iter().map(|row| row[*yi]).max().unwrap();
            if m < *b {
                let p1 = pos[b - 1];
                let p2 = (p1.0, *yi);
                (a[p1.0][p1.1], a[p2.0][p2.1]) = (a[p2.0][p2.1], a[p1.0][p1.1]);
                pos[a[p1.0][p1.1] - 1] = p1;
                pos[a[p2.0][p2.1] - 1] = p2;
            }
        }
    }

    println!("Yes");
    a.iter().for_each(|r| print_vec(r));
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
