pub use mod998244353::Mod;
use proconio::*;
use std::fmt::Display;

fn main() {
    input! {
        n: u32
    };
    println!("{}", solve(n))
}

fn solve(n: u32) -> usize {
    let mut a = 1;
    while a * (a + 1) * (a + 2) / 6 < n {
        a += 1;
    }
    let n = n - (a - 1) * a * (a + 1) / 6;
    let mut b = 1;
    while b * (b + 1) / 2 < n {
        b += 1;
    }
    let c = n - (b - 1) * b / 2;
    repunit(a) + repunit(b) + repunit(c)
}

fn repunit(n: u32) -> usize {
    (10usize.pow(n) - 1) / 9
}

#[allow(unused)]
fn binary_search<F>(mut left: usize, mut right: usize, predicate: F) -> usize
where
    F: Fn(usize) -> bool,
{
    debug_assert!(predicate(left));
    debug_assert!(!predicate(right));
    while right - left > 1 {
        let m = (left + right) / 2;
        if predicate(m) {
            left = m;
        } else {
            right = m;
        }
    }
    left
}

mod mod998244353 {
    use std::collections::VecDeque;
    use std::mem;
    use std::ops::*;

    const MOD: u128 = 998244353;

    #[derive(Clone, Copy, Debug)]
    pub struct Mod {
        inner: u128,
    }

    impl Neg for Mod {
        type Output = Self;

        fn neg(self) -> Self {
            Mod {
                inner: (MOD - self.inner) % MOD,
            }
        }
    }

    impl Add for Mod {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Mod {
                inner: (self.inner + rhs.inner) % MOD,
            }
        }
    }

    impl AddAssign for Mod {
        fn add_assign(&mut self, rhs: Self) {
            self.inner = (self.inner + rhs.inner) % MOD;
        }
    }

    impl Add<u128> for Mod {
        type Output = Self;

        fn add(self, rhs: u128) -> Self {
            Mod {
                inner: (self.inner + (rhs % MOD)) % MOD,
            }
        }
    }

    impl AddAssign<u128> for Mod {
        fn add_assign(&mut self, rhs: u128) {
            self.inner = (self.inner + (rhs % MOD)) % MOD;
        }
    }

    impl Add<Mod> for u128 {
        type Output = Mod;

        fn add(self, rhs: Mod) -> Mod {
            Mod {
                inner: (self + rhs.inner) % MOD,
            }
        }
    }

    impl Sub for Mod {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
            Mod {
                inner: (self.inner + MOD - rhs.inner) % MOD,
            }
        }
    }

    impl SubAssign for Mod {
        fn sub_assign(&mut self, rhs: Self) {
            self.inner = (self.inner + MOD - rhs.inner) % MOD;
        }
    }

    impl Sub<u128> for Mod {
        type Output = Self;

        fn sub(self, rhs: u128) -> Self {
            Mod {
                inner: (self.inner + MOD - (rhs % MOD)) % MOD,
            }
        }
    }

    impl SubAssign<u128> for Mod {
        fn sub_assign(&mut self, rhs: u128) {
            self.inner = (self.inner + MOD - (rhs % MOD)) % MOD;
        }
    }

    impl Sub<Mod> for u128 {
        type Output = Mod;

        fn sub(self, rhs: Mod) -> Mod {
            Mod {
                inner: ((self % MOD) + MOD - rhs.inner) % MOD,
            }
        }
    }

    impl Mul for Mod {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            Mod {
                inner: (self.inner * rhs.inner) % MOD,
            }
        }
    }

    impl MulAssign for Mod {
        fn mul_assign(&mut self, rhs: Self) {
            self.inner = (self.inner * rhs.inner) % MOD;
        }
    }

    impl Mul<u128> for Mod {
        type Output = Self;

        fn mul(self, rhs: u128) -> Self {
            Mod {
                inner: (self.inner * (rhs % MOD)) % MOD,
            }
        }
    }

    impl MulAssign<u128> for Mod {
        fn mul_assign(&mut self, rhs: u128) {
            self.inner = (self.inner * (rhs % MOD)) % MOD;
        }
    }

    impl Mul<Mod> for u128 {
        type Output = Mod;

        fn mul(self, rhs: Mod) -> Mod {
            Mod {
                inner: ((self % MOD) * rhs.inner) % MOD,
            }
        }
    }

    impl Div for Mod {
        type Output = Self;

        fn div(self, rhs: Self) -> Self {
            self.mul(Mod {
                inner: moddiv(rhs.inner),
            })
        }
    }

    impl DivAssign for Mod {
        fn div_assign(&mut self, rhs: Self) {
            self.inner = (self.inner * moddiv(rhs.inner)) % MOD;
        }
    }

    impl Div<u128> for Mod {
        type Output = Self;

        fn div(self, rhs: u128) -> Self {
            self.mul(Mod { inner: moddiv(rhs) })
        }
    }

    impl DivAssign<u128> for Mod {
        fn div_assign(&mut self, rhs: u128) {
            self.inner = (self.inner * moddiv(rhs)) % MOD;
        }
    }

    impl Div<Mod> for u128 {
        type Output = Mod;

        fn div(self, rhs: Mod) -> Mod {
            Mod::new((self % MOD) * moddiv(rhs.inner))
        }
    }

    impl Mod {
        pub fn new(n: u128) -> Self {
            Mod { inner: n % MOD }
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
                    res *= *self;
                }
            }
            res
        }
        pub fn get(&self) -> u128 {
            self.inner
        }
    }

    fn moddiv(mut a: u128) -> u128 {
        let mut b = MOD;
        let mut u = 1;
        let mut v = 0;
        while b != 0 {
            let t = a / b;
            a -= t * b;
            u += MOD - ((t * v) % MOD);
            mem::swap(&mut a, &mut b);
            mem::swap(&mut u, &mut v);
        }
        u % MOD
    }

    #[test]
    fn test_moddiv() {
        assert_eq!(moddiv(1), 1);
        assert_eq!(moddiv(2), 499122177);
        assert_eq!(moddiv(3), 332748118);
        assert_eq!(moddiv(4), 748683265);
        assert_eq!(moddiv(5), 598946612);
        assert_eq!(moddiv(6), 166374059);
    }
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
