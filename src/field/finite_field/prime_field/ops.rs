use crate::traits::{Field, Zero};

use super::core::Fp;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Adds two field elements, reducing modulo `P`.
impl<const P: u64> Add for Fp<P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Fp::new(((self.value() as u128 + rhs.value() as u128) % P as u128) as u64)
    }
}

/// Negates a field element: computes `P - self.0` for nonzero elements.
///
/// Note: `Fp(0)` negates to `Fp(0)` because `Fp::new` reduces `P mod P = 0`
impl<const P: u64> Neg for Fp<P> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Fp::new((P as u128 - self.value() as u128) as u64)
    }
}

/// Subtracts two field elements modulo `P`.
impl<const P: u64> Sub for Fp<P> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

/// Multiplies two field elements modulo `P`.
///
/// Uses 128-bit intermediate arithmetic to avoid overflow for large `P`.
impl<const P: u64> Mul for Fp<P> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Fp::new(((self.value() as u128 * rhs.value() as u128) % P as u128) as u64)
    }
}

/// Divides two field elements modulo `P`.
///
/// # Panics
/// Panics on division by zero
impl<const P: u64> Div for Fp<P> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        assert!(!rhs.is_zero(), "Cannot divide by zero");
        self * rhs.inv().unwrap()
    }
}
