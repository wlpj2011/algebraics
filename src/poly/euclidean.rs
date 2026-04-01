//! Implementation of [`EuclideanDomain`] for `Poly<T: Field>`
use super::core::Poly;
use crate::traits::*;

/// Implements polynomial division for polynomials over fields
/// 
/// # Panics
/// Panics on division by zero
impl<T: Field> EuclideanDomain for Poly<T> {
    fn div_rem(self, denominator: Self) -> (Self, Self) {
        assert!(!denominator.is_zero(), "Cannot divide by zero polynomial");
        if self.is_zero() {
            return (Poly::<T>::zero(), Poly::<T>::zero());
        }
        let mut quotient = Poly::<T>::zero();
        let mut remainder = self.clone();

        while !remainder.is_zero() && remainder.degree() >= denominator.degree(){
            let tmp_coeff = remainder.lead_coeff() * denominator.lead_coeff().inv().unwrap();
            let tmp_degree = remainder.degree().unwrap() - denominator.degree().unwrap(); //  Guaranteed non-negative
            let mut tmp_coeffs = vec![T::zero(); tmp_degree];
            tmp_coeffs.push(tmp_coeff);
            let tmp = Poly::new(tmp_coeffs);
            quotient = &quotient + &tmp;
            remainder = &remainder - &(&tmp * &denominator);
        }

        (quotient, remainder)
    }
}