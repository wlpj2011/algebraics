use super::core::Fp;
use std::ops::{Add, Mul, Neg, Sub};

impl<const P: u64> Add for Fp<P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Fp::new((self.value() + rhs.value()) % P)
    }
}

impl<const P: u64> Neg for Fp<P> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Fp::new((P - self.value()) % P)
    }
}

impl<const P: u64> Sub for Fp<P> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<const P: u64> Mul for Fp<P> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Fp::new(((self.value() as u128 * rhs.value() as u128) % P as u128) as u64)
    }
}
