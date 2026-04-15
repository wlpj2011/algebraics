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

#[test]
fn test_fp7_commutativity() {
    type F = Fp<7u64>;

    for a in F::enumerate() {
        for b in F::enumerate() {
            assert_eq!(a + b, b + a);
            assert_eq!(a * b, b * a);
        }
    }
}

#[test]
fn test_fp7_associativity() {
    type F = Fp<7u64>;

    for a in F::enumerate() {
        for b in F::enumerate() {
            for c in F::enumerate() {
                assert_eq!((a + b) + c, a + (b + c));
                assert_eq!((a * b) * c, a * (b * c));
            }
        }
    }
}

#[test]
fn test_fp7_zero_multiplication() {
    type F = Fp<7u64>;

    let zero = F::zero();

    for a in F::enumerate() {
        assert_eq!(a * zero, zero);
        assert_eq!(zero * a, zero);
    }
}

#[test]
fn test_fp7_enumeration_size() {
    type F = Fp<7u64>;

    let elems: Vec<_> = F::enumerate().collect();
    assert_eq!(elems.len(), 7);

    // Ensure all elements are distinct
    for i in 0..elems.len() {
        for j in 0..elems.len() {
            if i != j {
                assert_ne!(elems[i], elems[j]);
            }
        }
    }
}

// Checks multiplicative inverse for all nonzero elements
#[test]
fn test_fp7_multiplicative_inverse() {
    assert_eq!(Fp::<7>::zero().inv(), None);
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
fn test_large_prime_overflow_add() {
    const P: u64 = 18_446_744_073_709_551_557;
    let a = Fp::<P>::new(P - 1);
    let b = Fp::<P>::new(P - 1);
    assert_eq!(a + b, Fp::<P>::new(P - 2));
}

#[test]
fn test_large_prime_overflow_mul() {
    const P: u64 = 18_446_744_073_709_551_557; // 2^64 - 59
    let a = Fp::<P>::new(P - 1);
    let b = Fp::<P>::new(P - 1);
    assert_eq!(a * b, Fp::<P>::one());
}

