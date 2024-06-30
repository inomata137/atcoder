use std::fmt::Display;
use proconio::*;
use mod998244353::Mod;

fn main() {
    input! {
        n: i128,
        k: usize
    };
    let mut ans = Mod::new(n - 2) / n;
    ans = ans.pow(k);
    ans *= 1 - n;
    ans += n + 1;
    ans /= 2;
    println!("{}", ans.get());
}

mod mod998244353 {
    use std::ops::*;
    use std::collections::VecDeque;
    use std::mem;

    const MOD: i128 = 998244353;

    #[derive(Clone, Copy, Debug)]
    pub struct Mod {
        inner: i128
    }

    impl Neg for Mod {
        type Output = Self;

        fn neg(self) -> Self {
            Mod {
                inner: (MOD - self.inner) % MOD
            }
        }
    }

    impl Add for Mod {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Mod {
                inner: (self.inner + rhs.inner) % MOD
            }
        }
    }

    impl AddAssign for Mod {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }

    impl Add<i128> for Mod {
        type Output = Self;

        fn add(self, rhs: i128) -> Self {
            Mod {
                inner: (self.inner + (rhs % MOD)) % MOD
            }
        }
    }

    impl AddAssign<i128> for Mod {
        fn add_assign(&mut self, rhs: i128) {
            *self = *self + rhs;
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

    impl SubAssign for Mod {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }

    impl Sub<i128> for Mod {
        type Output = Self;

        fn sub(self, rhs: i128) -> Self {
            Mod {
                inner: (self.inner + MOD - (rhs % MOD)) % MOD
            }
        }
    }

    impl SubAssign<i128> for Mod {
        fn sub_assign(&mut self, rhs: i128) {
            *self = *self - rhs;
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

    impl MulAssign for Mod {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs;
        }
    }

    impl Mul<i128> for Mod {
        type Output = Self;

        fn mul(self, rhs: i128) -> Self {
            Mod {
                inner: (self.inner * (rhs % MOD)) % MOD
            }
        }
    }

    impl MulAssign<i128> for Mod {
        fn mul_assign(&mut self, rhs: i128) {
            *self = *self * rhs;
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

    impl DivAssign for Mod {
        fn div_assign(&mut self, rhs: Self) {
            *self = *self / rhs;
        }
    }

    impl Div<i128> for Mod {
        type Output = Self;

        fn div(self, rhs: i128) -> Self {
            self * Mod {
                inner: moddiv(rhs)
            }
        }
    }

    impl DivAssign<i128> for Mod {
        fn div_assign(&mut self, rhs: i128) {
            *self = *self / rhs;
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
        pub fn get(&self) -> i128 {
            (self.inner + MOD) % MOD
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
            mem::swap(&mut a, &mut b);
            mem::swap(&mut u, &mut v);
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
