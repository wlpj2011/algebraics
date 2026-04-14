//! Integration tests for `FpN` — finite field extensions via Conway polynomials.
//!
//! Four concrete cases:
//!   FpN<2, 2>  = GF(4)     — Conway poly x²+x+1,      so α² = α+1
//!   FpN<2, 3>  = GF(8)     — Conway poly x³+x+1,      so α³ = α+1
//!   FpN<3, 2>  = GF(9)     — Conway poly x²+2x+2,     so α² = α+1  (in F_3)
//!   FpN<13, 6> = GF(13^6)  — primitivity and Fermat checks only

use algebraics::arithmetic::pow;
use algebraics::field::{Fp, FpN};
use algebraics::traits::*;

// ══════════════════════════════════════════════════════
// GF(4) = FpN<2, 2>
// Conway poly: x²+x+1  →  α² = α+1
// Elements: 0, 1, α, α+1   (multiplicative order of α: 3)
// ══════════════════════════════════════════════════════

type GF4 = FpN<2, 2>;

#[test]
fn test_gf4_size_char_degree() {
    assert_eq!(GF4::size(), 4);
    assert_eq!(GF4::characteristic(), 2);
    assert_eq!(<GF4 as FiniteExtension>::degree(), 2);
}

#[test]
fn test_gf4_alpha_squared() {
    let a = GF4::generator();
    assert_eq!(a.clone() * a.clone(), a + GF4::one());
}

#[test]
fn test_gf4_fermat() {
    for x in GF4::multiplicative_group() {
        assert_eq!(pow(x, 3), GF4::one());
    }
}

#[test]
fn test_gf4_field_axioms_exhaustive() {
    let elems: Vec<GF4> = GF4::enumerate().collect();
    for a in &elems {
        assert_eq!(a.clone() + GF4::zero(), a.clone());
        assert_eq!(a.clone() * GF4::one(), a.clone());
        assert_eq!(a.clone() + (-a.clone()), GF4::zero());
        if !a.is_zero() {
            assert_eq!(a.clone() * a.inv().unwrap(), GF4::one());
        }
    }
    for a in &elems {
        for b in &elems {
            for c in &elems {
                assert_eq!(
                    a.clone() * (b.clone() + c.clone()),
                    a.clone() * b.clone() + a.clone() * c.clone()
                );
            }
        }
    }
}

// ══════════════════════════════════════════════════════
// GF(8) = FpN<2, 3>
// Conway poly: x³+x+1  →  α³ = α+1
// Frobenius: x ↦ x²
// Trace = x + x² + x⁴  (sum of Frobenius orbit)
// Norm  = x^7 = 1 for all nonzero x  (since |F×| = 7)
// ══════════════════════════════════════════════════════

type GF8 = FpN<2, 3>;

#[test]
fn test_gf8_size_char_degree() {
    assert_eq!(GF8::size(), 8);
    assert_eq!(GF8::characteristic(), 2);
    assert_eq!(<GF8 as FiniteExtension>::degree(), 3);
}

#[test]
fn test_gf8_alpha_cubed() {
    // α³+α+1 = 0  →  α³ = α+1
    let a = GF8::generator();
    assert_eq!(a.clone() * a.clone() * a.clone(), a + GF8::one());
}

#[test]
fn test_gf8_fermat() {
    for x in GF8::multiplicative_group() {
        assert_eq!(pow(x, 7), GF8::one());
    }
}

#[test]
fn test_gf8_trace_values() {
    // Tr(0)=0, Tr(1)=1+1+1=1, Tr(α)=α+α²+(α²+α)=0, Tr(α+1)=1
    let a = GF8::generator();
    assert_eq!(GF8::zero().trace(), Fp::<2>::zero());
    assert_eq!(GF8::one().trace(), Fp::<2>::one());
    assert_eq!(a.clone().trace(), Fp::<2>::zero());
    assert_eq!((a + GF8::one()).trace(), Fp::<2>::one());
}

#[test]
fn test_gf8_trace_additive() {
    let elems: Vec<GF8> = GF8::enumerate().collect();
    for a in &elems {
        for b in &elems {
            assert_eq!((a.clone() + b.clone()).trace(), a.trace() + b.trace());
        }
    }
}

#[test]
fn test_gf8_norm_nonzero_is_one() {
    // N(x) = x^(1+2+4) = x^7 = 1 for all x in GF(8)×
    for x in GF8::multiplicative_group() {
        assert_eq!(FiniteExtension::norm(&x), Fp::<2>::one());
    }
}

#[test]
fn test_gf8_field_axioms_exhaustive() {
    let elems: Vec<GF8> = GF8::enumerate().collect();
    for a in &elems {
        assert_eq!(a.clone() + GF8::zero(), a.clone());
        assert_eq!(a.clone() * GF8::one(), a.clone());
        assert_eq!(a.clone() + (-a.clone()), GF8::zero());
        if !a.is_zero() {
            assert_eq!(a.clone() * a.inv().unwrap(), GF8::one());
        }
    }
    for a in &elems {
        for b in &elems {
            for c in &elems {
                assert_eq!(
                    a.clone() * (b.clone() + c.clone()),
                    a.clone() * b.clone() + a.clone() * c.clone()
                );
            }
        }
    }
}

// ══════════════════════════════════════════════════════
// GF(9) = FpN<3, 2>
// Conway poly: x²+2x+2  →  α² = -2α-2 = α+1  (in F_3)
// Powers: α²=α+1, α³=2α+1, α⁴=2, α⁵=2α, α⁶=2α+2, α⁷=α+2, α⁸=1
// Trace = α + α³ = α+(2α+1) = 1  (in F_3)
// Norm  = α · α³ = α⁴ = 2         (in F_3)
// ══════════════════════════════════════════════════════

type GF9 = FpN<3, 2>;

#[test]
fn test_gf9_size_char_degree() {
    assert_eq!(GF9::size(), 9);
    assert_eq!(GF9::characteristic(), 3);
    assert_eq!(<GF9 as FiniteExtension>::degree(), 2);
}

#[test]
fn test_gf9_alpha_squared() {
    // α²+2α+2 = 0  →  α² = -2α-2 = α+1 in F_3
    let a = GF9::generator();
    assert_eq!(a.clone() * a.clone(), a + GF9::one());
}

#[test]
fn test_gf9_alpha_fourth() {
    // α⁴ = 2  (the scalar 2 ∈ F_3 embedded in GF(9))
    let two = GF9::embed(Fp::<3>::new(2));
    assert_eq!(pow(GF9::generator(), 4), two);
}

#[test]
fn test_gf9_fermat() {
    for x in GF9::multiplicative_group() {
        assert_eq!(pow(x, 8), GF9::one());
    }
}

#[test]
fn test_gf9_trace_alpha() {
    assert_eq!(GF9::generator().trace(), Fp::<3>::one());
}

#[test]
fn test_gf9_trace_additive() {
    let elems: Vec<GF9> = GF9::enumerate().collect();
    for a in &elems {
        for b in &elems {
            assert_eq!((a.clone() + b.clone()).trace(), a.trace() + b.trace());
        }
    }
}

#[test]
fn test_gf9_norm_alpha() {
    // N(α) = α · α³ = α⁴ = 2 ∈ F_3
    assert_eq!(FiniteExtension::norm(&GF9::generator()), Fp::<3>::new(2));
}

#[test]
fn test_gf9_norm_multiplicative() {
    let elems: Vec<GF9> = GF9::enumerate().collect();
    for a in &elems {
        for b in &elems {
            assert_eq!(
                FiniteExtension::norm(&(a.clone() * b.clone())),
                FiniteExtension::norm(a) * FiniteExtension::norm(b),
            );
        }
    }
}

#[test]
fn test_gf9_field_axioms_exhaustive() {
    let elems: Vec<GF9> = GF9::enumerate().collect();
    for a in &elems {
        assert_eq!(a.clone() + GF9::zero(), a.clone());
        assert_eq!(a.clone() * GF9::one(), a.clone());
        assert_eq!(a.clone() + (-a.clone()), GF9::zero());
        if !a.is_zero() {
            assert_eq!(a.clone() * a.inv().unwrap(), GF9::one());
        }
    }
    for a in &elems {
        for b in &elems {
            for c in &elems {
                assert_eq!(
                    a.clone() * (b.clone() + c.clone()),
                    a.clone() * b.clone() + a.clone() * c.clone()
                );
            }
        }
    }
}

// ══════════════════════════════════════════════════════
// GF(13^6) = FpN<13, 6>
// Too large for exhaustive tests (13^6 = 4,826,809 elements).
// Focus on: size/char/degree, Fermat for the generator,
// and primitivity (α should not lie in any proper subfield).
// ══════════════════════════════════════════════════════

type GF13_6 = FpN<13, 6>;

#[test]
fn test_gf13_6_size_char_degree() {
    assert_eq!(GF13_6::size(), 13usize.pow(6));
    assert_eq!(GF13_6::characteristic(), 13);
    assert_eq!(<GF13_6 as FiniteExtension>::degree(), 6);
}

#[test]
fn test_gf13_6_generator_fermat() {
    // α^(13^6 - 1) = 1
    assert_eq!(pow(GF13_6::generator(), 13u64.pow(6) - 1), GF13_6::one());
}

#[test]
fn test_gf13_6_generator_primitive() {
    // The Conway polynomial is primitive: α has order exactly 13^6-1,
    // so it must not satisfy x^(13^k - 1) = 1 for any proper divisor k | 6.
    // Proper divisors: 1, 2, 3.
    let a = GF13_6::generator();
    assert_ne!(pow(a.clone(), 13u64.pow(1) - 1), GF13_6::one());
    assert_ne!(pow(a.clone(), 13u64.pow(2) - 1), GF13_6::one());
    assert_ne!(pow(a.clone(), 13u64.pow(3) - 1), GF13_6::one());
}

#[test]
fn test_gf13_6_sampled_distributivity() {
    // Spot-check distributivity on a few elements derived from the generator.
    let a = GF13_6::generator();
    let b = pow(a.clone(), 2);
    let c = pow(a.clone(), 3);
    assert_eq!(
        a.clone() * (b.clone() + c.clone()),
        a.clone() * b + a.clone() * c,
    );
}
