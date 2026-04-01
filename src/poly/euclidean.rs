//! Implementation of [`EuclideanDomain`] for `Poly<T: Field>`
use super::core::Poly;
use crate::traits::*;

/// Implements polynomial division for polynomials over fields
impl<T: Field> EuclideanDomain for Poly<T> {
    fn div_rem(self, _other: Self) -> (Self, Self) {
        todo!()
    }
}