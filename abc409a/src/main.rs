#[allow(unused)]
use mod998244353::Mod;
use proconio::*;

fn main() {
    input! {
        _n: usize,
        t: proconio::marker::Chars,
        a: proconio::marker::Chars
    };
    let conflict = t
        .iter()
        .zip(a.iter())
        .any(|(tc, ac)| tc == &'o' && ac == &'o');
    let msg = if conflict { "Yes" } else { "No" };
    println!("{msg}");
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

mod mod998244353 {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    const MOD: usize = 998244353;

    #[derive(Clone, Copy, Debug)]
    pub struct Mod(usize);

    impl From<usize> for Mod {
        fn from(n: usize) -> Self {
            Mod(n % MOD)
        }
    }

    impl Neg for Mod {
        type Output = Self;

        fn neg(self) -> Self {
            match self.0 {
                0 => Mod(0),
                _ => Mod(MOD - self.0),
            }
        }
    }

    impl Add for Mod {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Mod((self.0 + rhs.0) % MOD)
        }
    }

    impl AddAssign for Mod {
        fn add_assign(&mut self, rhs: Self) {
            self.0 = (self.0 + rhs.0) % MOD;
        }
    }

    impl Add<usize> for Mod {
        type Output = Self;

        fn add(self, rhs: usize) -> Self {
            Mod((self.0 + (rhs % MOD)) % MOD)
        }
    }

    impl AddAssign<usize> for Mod {
        fn add_assign(&mut self, rhs: usize) {
            self.0 = (self.0 + (rhs % MOD)) % MOD;
        }
    }

    impl Add<Mod> for usize {
        type Output = Mod;

        fn add(self, rhs: Mod) -> Mod {
            Mod((self + rhs.0) % MOD)
        }
    }

    impl Sub for Mod {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
            match self.0.cmp(&rhs.0) {
                std::cmp::Ordering::Greater => Mod(self.0 - rhs.0),
                std::cmp::Ordering::Equal => Mod(0),
                std::cmp::Ordering::Less => Mod(self.0 + MOD - rhs.0),
            }
        }
    }

    impl SubAssign for Mod {
        fn sub_assign(&mut self, rhs: Self) {
            match self.0.cmp(&rhs.0) {
                std::cmp::Ordering::Greater => self.0 -= rhs.0,
                std::cmp::Ordering::Equal => self.0 = 0,
                std::cmp::Ordering::Less => self.0 += MOD - rhs.0,
            }
        }
    }

    impl Sub<usize> for Mod {
        type Output = Self;

        fn sub(self, rhs: usize) -> Self {
            let rhs = rhs % MOD;
            match self.0.cmp(&rhs) {
                std::cmp::Ordering::Greater => Mod(self.0 - rhs),
                std::cmp::Ordering::Equal => Mod(0),
                std::cmp::Ordering::Less => Mod(self.0 + MOD - rhs),
            }
        }
    }

    impl SubAssign<usize> for Mod {
        fn sub_assign(&mut self, rhs: usize) {
            let rhs = rhs % MOD;
            match self.0.cmp(&rhs) {
                std::cmp::Ordering::Greater => self.0 -= rhs,
                std::cmp::Ordering::Equal => self.0 = 0,
                std::cmp::Ordering::Less => self.0 += MOD - rhs,
            }
        }
    }

    impl Sub<Mod> for usize {
        type Output = Mod;

        fn sub(self, rhs: Mod) -> Mod {
            let self_mod = self % MOD;
            match self_mod.cmp(&rhs.0) {
                std::cmp::Ordering::Greater => Mod(self_mod - rhs.0),
                std::cmp::Ordering::Equal => Mod(0),
                std::cmp::Ordering::Less => Mod(self_mod + MOD - rhs.0),
            }
        }
    }

    impl Mul for Mod {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            Mod((self.0 * rhs.0) % MOD)
        }
    }

    impl MulAssign for Mod {
        fn mul_assign(&mut self, rhs: Self) {
            self.0 = (self.0 * rhs.0) % MOD;
        }
    }

    impl Mul<usize> for Mod {
        type Output = Self;

        fn mul(self, rhs: usize) -> Self {
            Mod((self.0 * (rhs % MOD)) % MOD)
        }
    }

    impl MulAssign<usize> for Mod {
        fn mul_assign(&mut self, rhs: usize) {
            self.0 = (self.0 * (rhs % MOD)) % MOD;
        }
    }

    impl Mul<Mod> for usize {
        type Output = Mod;

        fn mul(self, rhs: Mod) -> Mod {
            Mod(((self % MOD) * rhs.0) % MOD)
        }
    }

    impl Div for Mod {
        type Output = Self;

        fn div(self, rhs: Self) -> Self {
            Mod((self.0 * inverse(rhs.0)) % MOD)
        }
    }

    impl DivAssign for Mod {
        fn div_assign(&mut self, rhs: Self) {
            self.0 = (self.0 * inverse(rhs.0)) % MOD;
        }
    }

    impl Div<usize> for Mod {
        type Output = Self;

        fn div(self, rhs: usize) -> Self {
            Mod((self.0 * inverse(rhs)) % MOD)
        }
    }

    impl DivAssign<usize> for Mod {
        fn div_assign(&mut self, rhs: usize) {
            self.0 = (self.0 * inverse(rhs)) % MOD;
        }
    }

    impl Div<Mod> for usize {
        type Output = Mod;

        fn div(self, rhs: Mod) -> Mod {
            Mod(((self % MOD) * inverse(rhs.0)) % MOD)
        }
    }

    impl Mod {
        pub const fn new(n: usize) -> Self {
            Mod(n % MOD)
        }

        pub const fn pow(&self, k: usize) -> Self {
            Mod(modpow(self.0, k))
        }

        pub const fn inv(&self) -> Self {
            Mod(inverse(self.0))
        }

        pub const fn inner(&self) -> &usize {
            &self.0
        }
    }

    /// Computes the modular inverse of `a` under modulo `MOD`.
    ///
    /// It uses Fermat's Little Theorem.
    ///
    /// If `p` is prime and GCD(a, p) == 1, then a^(p-1) ≡ 1 (mod p).
    /// Thus, a^(p-2) ≡ a^(-1) (mod p).
    const fn inverse(a: usize) -> usize {
        let a = a % MOD;
        if a == 0 {
            panic!("Cannot compute inverse of zero");
        }
        modpow(a, MOD - 2)
    }

    const fn modpow(mut base: usize, mut pow: usize) -> usize {
        base %= MOD;
        let mut ans = 1;
        while pow > 0 {
            if pow & 1 == 1 {
                ans = (ans * base) % MOD;
            }
            base = (base * base) % MOD;
            pow >>= 1;
        }
        ans
    }

    #[cfg(test)]
    #[test]
    fn test_inverse() {
        assert_eq!(inverse(1), 1);
        assert_eq!(inverse(2), 499122177);
        assert_eq!(inverse(3), 332748118);
        assert_eq!(inverse(4), 748683265);
        assert_eq!(inverse(5), 598946612);
        assert_eq!(inverse(6), 166374059);
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
