use proconio::*;
use std::collections::VecDeque;
use std::{cmp::Ordering, fmt::Display};

// left, right, height
type E = (usize, usize, usize);

struct State {
    v: VecDeque<E>,
    area: usize,
}

impl State {
    fn merge(&mut self, ne: E) {
        if self.v.is_empty() {
            self.v.push_back((0, ne.1, ne.2));
            self.area = ne.1 * ne.2;
            return;
        }
        let last = self.v.back_mut().unwrap();
        match last.2.cmp(&ne.2) {
            Ordering::Greater => {
                self.area += (ne.1 - ne.0) * ne.2;
                self.v.push_back(ne);
            }
            Ordering::Equal => {
                self.area += (ne.1 - last.1) * last.2;
                last.1 = ne.1;
            }
            Ordering::Less => {
                let last = self.v.pop_back().unwrap();
                self.area -= (last.1 - last.0) * last.2;
                self.merge((last.0, ne.1, ne.2));
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    };
    let mut ans = vec![0usize; n];
    ans[0] = h[0];

    let mut state = State {
        v: VecDeque::new(),
        area: h[0],
    };
    for (i, h) in h.iter().enumerate().take(n) {
        state.merge((i, i + 1, *h));
        if i > 0 {
            print!(" ")
        }
        print!("{}", state.area + 1)
    }
    println!()
}

#[allow(unused)]
fn print_vec<T: Display>(v: &[T]) {
    if v.is_empty() {
        return;
    }
    print!("{}", v[0]);
    for e in &v[1..] {
        print!(" {}", e);
    }
    println!();
}
