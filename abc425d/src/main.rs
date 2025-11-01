#[allow(unused)]
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    };

    let painted = s
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut blacks =
        std::collections::VecDeque::from_iter(painted.iter().enumerate().flat_map(|(li, l)| {
            l.iter()
                .enumerate()
                .filter_map(move |(ci, p)| if *p { Some((li, ci)) } else { None })
        }));

    enum Tile {
        Reachable { dist: usize },
        Unreachable,
        Unknown,
    }

    let mut tiles = (0..h)
        .map(|li| {
            (0..w)
                .map(|ci| {
                    if painted[li][ci] {
                        Tile::Reachable { dist: 0 }
                    } else {
                        Tile::Unknown
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    while let Some((li, ci)) = blacks.pop_front() {
        let mut dst = vec![];
        if li > 0 {
            dst.push((li - 1, ci))
        }
        if li < h - 1 {
            dst.push((li + 1, ci))
        }
        if ci > 0 {
            dst.push((li, ci - 1))
        }
        if ci < w - 1 {
            dst.push((li, ci + 1))
        }
        let Tile::Reachable { dist: dist_from } = tiles[li][ci] else {
            panic!()
        };
        for (dst_li, dst_ci) in dst {
            match tiles[dst_li][dst_ci] {
                Tile::Reachable { dist, .. } if dist == dist_from + 1 => {
                    tiles[dst_li][dst_ci] = Tile::Unreachable
                }
                Tile::Unknown => {
                    let mut f = true;
                    if dst_li > 0 {
                        f = f && !matches!(tiles[dst_li - 1][dst_ci], Tile::Reachable { .. })
                    }
                    if dst_li < h - 1 {
                        f = f && !matches!(tiles[dst_li + 1][dst_ci], Tile::Reachable { .. })
                    }
                    if dst_ci > 0 {
                        f = f && !matches!(tiles[dst_li][dst_ci - 1], Tile::Reachable { .. })
                    }
                    if dst_ci < w - 1 {
                        f = f && !matches!(tiles[dst_li][dst_ci + 1], Tile::Reachable { .. })
                    }
                    if f {
                        tiles[dst_li][dst_ci] = Tile::Reachable {
                            dist: dist_from + 1,
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let ans = tiles
        .into_iter()
        .flat_map(|l| {
            l.into_iter()
                .filter(|p| matches!(p, Tile::Reachable { .. }))
        })
        .count();

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
