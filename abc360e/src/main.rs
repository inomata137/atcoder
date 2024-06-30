use std::fmt::Display;
use proconio::*;
use mod998244353::Mod;

fn main() {
    input! {
        n: i128,
        k: usize
    };
    let mut ans = Mod::new(n - 2) / Mod::new(n);
    ans = ans.pow(k);
    ans = ans * Mod::new(n - 1);
    ans = Mod::new(n + 1) - ans;
    ans = ans / Mod::new(2);
    println!("{}", ans.inner);
}

mod mod998244353 {
    use std::ops::{Add, Div, Mul, Sub};
    use std::collections::VecDeque;

    const MOD: i128 = 998244353;

    #[derive(Clone, Copy, Debug)]
    pub struct Mod {
        pub inner: i128
    }

    impl Add for Mod {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Mod {
                inner: (self.inner + rhs.inner) % MOD
            }
        }
    }

    impl Sub for Mod {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
            Mod {
                inner: (self.inner + MOD - rhs.inner) % MOD
            }
        }
    }

    impl Mul for Mod {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            Mod {
                inner: (self.inner * rhs.inner) % MOD
            }
        }
    }

    impl Div for Mod {
        type Output = Self;

        fn div(self, rhs: Self) -> Self {
            self * Mod {
                inner: moddiv(rhs.inner)
            }
        }
    }

    impl Mod {
        pub fn new(n: i128) -> Self {
            Mod {
                inner: (n % MOD + MOD) % MOD
            }
        }
        pub fn pow(&self, mut k: usize) -> Self {
            let mut vd = VecDeque::new();
            while k > 0 {
                vd.push_back(k % 2);
                k >>= 1;
            }
            let mut res = Mod { inner: 1 };
            while let Some(d) = vd.pop_back() {
                res = res * res;
                if d == 1 {
                    res = res * *self;
                }
            }
            res
        }
    }

    fn moddiv(mut a: i128) -> i128 {
        let mut b = MOD;
        let mut u = 1;
        let mut v = 0;
        while b != 0{
            let t = a / b;
            a -= t * b;
            u -= t * v;
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut u, &mut v);
        }
        u %= MOD;
        if u < 0 {
            u += MOD;
        };
        u
    }
}

#[allow(unused)]
fn print_vec<T: Display>(v: &Vec<T>) {
    if v.len() == 0 {
        return;
    }
    print!("{}", v[0]);
    for e in &v[1..] {
        print!(" {}", e);
    }
    println!();
}
