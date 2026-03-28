use algebraics::prime_field::Fp;
use algebraics::traits::{Field, One, Zero};

// Edge case worth having for characteristic 2
#[test]
fn test_fp2_negation() {
    assert_eq!(-Fp::<2>::new(1), Fp::<2>::new(1));
}

// Checks additive identity axiom exhaustively for a small field
#[test]
fn test_fp7_additive_identity() {
    for n in 0..7u64 {
        let a = Fp::<7>::new(n);
        assert_eq!(a + Fp::<7>::zero(), a);
    }
}

// Checks additive inverse axiom exhaustively
#[test]
fn test_fp7_additive_inverse() {
    for n in 0..7u64 {
        let a = Fp::<7>::new(n);
        assert_eq!(a + (-a), Fp::<7>::zero());
    }
}

// Checks multiplicative inverse for all nonzero elements
#[test]
fn test_fp7_multiplicative_inverse() {
    for n in 1..7u64 {
        let a = Fp::<7>::new(n);
        assert_eq!(a * a.inv().unwrap(), Fp::<7>::one());
    }
}

// Checks distributivity for all triples — F_7 is small enough to be exhaustive
#[test]
fn test_fp7_distributivity() {
    for a in 0..7u64 {
        for b in 0..7u64 {
            for c in 0..7u64 {
                let (a, b, c) = (Fp::<7>::new(a), Fp::<7>::new(b), Fp::<7>::new(c));
                assert_eq!(a * (b + c), a * b + a * c);
            }
        }
    }
}

// Checks a known mathematical fact using a larger prime
#[test]
fn test_fp53_fermat_little_theorem() {
    for n in [1u64, 2, 10, 37, 52] {
        let a = Fp::<53>::new(n);
        let mut result = Fp::<53>::one();
        for _ in 0..52 {
            result = result * a;
        }
        assert_eq!(result, Fp::<53>::one());
    }
}

#[test]
fn test_reduction() {
    assert_eq!(Fp::<7>::new(10), Fp::<7>::new(3));
    assert_eq!(Fp::<53>::new(100), Fp::<53>::new(47));
}
