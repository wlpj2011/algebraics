use algebraics::field::Fp;
use algebraics::traits::{Field, Finite, FiniteField, One, Zero};

mod helpers;
use helpers::*;

// Edge case worth having for characteristic 2
#[test]
fn test_fp2_negation() {
    assert_eq!(-Fp::<2>::new(1), Fp::<2>::new(1));
}

#[test]
fn test_fp7_axioms_exhaustive() {
    let elems = exhaustive_elements::<Fp<7>>();
    check_field_axioms(&elems);
}

#[test]
fn test_fp23_axioms_exhaustive() {
    let elems = exhaustive_elements::<Fp<23>>();
    check_field_axioms(&elems);
}

#[test]
fn test_fp101_axioms_exhaustive() {
    let elems = exhaustive_elements::<Fp<101>>();
    check_field_axioms(&elems);
}

#[cfg(feature = "expensive_tests")]
#[test]
fn test_fp547_axioms_exhaustive() {
    let elems = exhaustive_elements::<Fp<547>>();
    check_field_axioms(&elems);
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
