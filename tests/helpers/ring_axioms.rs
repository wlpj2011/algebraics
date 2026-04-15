use algebraics::traits::*;
use std::fmt::Debug;

pub fn check_ring_axioms<F: Ring + Debug>(elems: &[F]) {
    check_addition_associative(elems);
    check_addition_commutative(elems);
    check_additive_inverse_identity(elems);
    check_multiplication_associative(elems);
    check_multiplication_zero(elems);
}

pub fn check_commutative_ring_axioms<F: CommutativeRing + Debug>(elems: &[F]) {
    check_ring_axioms(elems);
    check_multiplication_commutative(elems);
}

pub fn check_addition_associative<F: Semigroup + Debug>(elems: &[F]) {
    for a in elems {
        for b in elems {
            for c in elems {
                assert_eq!(
                    (a.clone() + b.clone()) + c.clone(),
                    a.clone() + (b.clone() + c.clone())
                );
            }
        }
    }
}

pub fn check_additive_inverse_identity<F: Group + Debug>(elems: &[F]) {
    for a in elems {
        assert_eq!(a.clone() + F::zero(), a.clone());
        assert_eq!(a.clone() + (-a.clone()), F::zero());
    }
}

pub fn check_addition_commutative<F: AbelianGroup + Debug>(elems: &[F]) {
    for a in elems {
        for b in elems {
            assert_eq!(a.clone() + b.clone(), b.clone() + a.clone());
        }
    }
}

pub fn check_multiplication_associative<F: Ring + Debug>(elems: &[F]) {
    for a in elems {
        for b in elems {
            for c in elems {
                assert_eq!(
                    (a.clone() * b.clone()) * c.clone(),
                    a.clone() * (b.clone() * c.clone())
                );
            }
        }
    }
}

pub fn check_multiplication_commutative<F: CommutativeRing + Debug>(elems: &[F]) {
    for a in elems {
        for b in elems {
            assert_eq!(a.clone() * b.clone(), b.clone() * a.clone());
        }
    }
}

pub fn check_multiplication_zero< F: Ring + Debug>(elems: &[F]) {
    for a in elems {
        assert_eq!(a.clone() * F::zero(), F::zero())
    }
}
