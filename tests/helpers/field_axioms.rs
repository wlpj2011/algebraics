use super::ring_axioms::check_commutative_ring_axioms;
use algebraics::traits::*;
use std::fmt::Debug;

pub fn check_field_axioms<F: Field + Debug>(elems: &[F]) {
    check_commutative_ring_axioms(elems);
    check_multiplicative_inverse_identity(elems);
}

pub fn check_multiplicative_inverse_identity<F: Field + Debug>(elems: &[F]) {
    for a in elems {
        assert_eq!(a.clone() * F::one(), a.clone());
        if !a.is_zero() {
            assert_eq!(a.clone() * (a.inv().unwrap()), F::one());
        }
    }
}
