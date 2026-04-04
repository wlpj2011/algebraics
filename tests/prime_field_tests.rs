use algebraics::field::Fp;
use algebraics::traits::{Field, Finite, FiniteField, One, Zero};

// Edge case worth having for characteristic 2
#[test]
fn test_fp2_negation() {
    assert_eq!(-Fp::<2>::new(1), Fp::<2>::new(1));
}

// Checks additive identity axiom exhaustively for a small field
#[test]
fn test_fp7_additive_identity() {
    for a in Fp::<7>::enumerate() {
        assert_eq!(a + Fp::<7>::zero(), a);
    }
}

// Checks additive inverse axiom exhaustively
#[test]
fn test_fp7_additive_inverse() {
    for a in Fp::<7>::enumerate() {
        assert_eq!(a + (-a), Fp::<7>::zero());
    }
}

// Checks multiplicative inverse for all nonzero elements
#[test]
fn test_fp7_multiplicative_inverse() {
    for a in Fp::<7>::multiplicative_group() {
        assert_eq!(a * a.inv().unwrap(), Fp::<7>::one());
    }
}

// Checks distributivity for all triples — F_7 is small enough to be exhaustive
#[test]
fn test_fp7_distributivity() {
    for a in Fp::<7>::enumerate() {
        for b in Fp::<7>::enumerate() {
            for c in Fp::<7>::enumerate() {
                assert_eq!(a * (b + c), a * b + a * c);
            }
        }
    }
}

// Checks a known mathematical fact using a larger prime
#[test]
fn test_fp53_fermat_little_theorem() {
    for a in Fp::<53>::multiplicative_group() {
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

#[test]
fn test_large_prime_overflow_mul() {
    const P: u64 = 18_446_744_073_709_551_557; // 2^64 - 59
    let a = Fp::<P>::new(18_446_744_073_709_551_556);
    let b = Fp::<P>::new(18_446_744_073_709_551_556);
    println!("{} * {} = 1", a, b);
    assert_eq!(a * b, Fp::<P>::one());
}
