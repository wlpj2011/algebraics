//! Integration tests for [`SimpleExtension`] using GF(4) as the concrete case.
//!
//! GF(4) = F₂[x]/(x²+x+1) has four elements: {0, 1, α, α+1},
//! where α = generator() is a root of x²+x+1. All arithmetic is mod 2.
//!
//! Key relations (x² ≡ x+1 mod x²+x+1):
//!   α²      = α+1
//!   α(α+1)  = 1
//!   (α+1)²  = α
//!
//! Frobenius (squaring in char 2):   0↦0,  1↦1,  α↦α+1,  α+1↦α
//! Trace (Tr = id + Frob):           Tr(0)=0,  Tr(1)=0,  Tr(α)=1,  Tr(α+1)=1
//! Norm (N = id · Frob):             N(0)=0,   N(1)=1,   N(α)=1,   N(α+1)=1

use algebraics::field::{FiniteSimpleExtension, Fp};
use algebraics::poly::Poly;
use algebraics::traits::*;

// ---- GF(4) setup ----

/// Conway polynomial for GF(4): x² + x + 1 over F₂.
///
/// This is the lexicographically minimal primitive irreducible polynomial of
/// degree 2 over F₂, making it compatible with Conway polynomial tables.
struct ConwayGF4;

impl IrreduciblePoly<Fp<2>> for ConwayGF4 {
    fn modulus() -> Poly<Fp<2>> {
        // 1 + x + x², in ascending degree order
        Poly::new(vec![Fp::<2>::one(), Fp::<2>::one(), Fp::<2>::one()])
    }
}

type GF4 = FiniteSimpleExtension<Fp<2>, ConwayGF4>;

/// All four elements of GF(4).
fn gf4_all() -> [GF4; 4] {
    [
        GF4::zero(),
        GF4::one(),
        GF4::generator(),
        GF4::generator() + GF4::one(),
    ]
}

/// α = generator(), a root of x²+x+1.
fn alpha() -> GF4 {
    GF4::generator()
}

/// α+1, the other non-base-field element.
fn alpha_plus_one() -> GF4 {
    GF4::generator() + GF4::one()
}

// ---- Modulus sanity ----

#[test]
fn test_conway_gf4_modulus_degree() {
    assert_eq!(ConwayGF4::degree(), 2);
}

// ---- Extension degree and characteristic ----

#[test]
fn test_gf4_degree() {
    assert_eq!(<GF4 as FiniteExtension>::degree(), 2);
}

#[test]
fn test_gf4_characteristic() {
    assert_eq!(GF4::characteristic(), 2);
}

// ---- Field axioms (exhaustive over all 4 elements) ----

#[test]
fn test_gf4_additive_identity() {
    for a in gf4_all() {
        assert_eq!(a.clone() + GF4::zero(), a.clone());
        assert_eq!(GF4::zero() + a.clone(), a.clone());
    }
}

#[test]
fn test_gf4_additive_inverse() {
    for a in gf4_all() {
        assert_eq!(a.clone() + (-a.clone()), GF4::zero());
    }
}

#[test]
fn test_gf4_add_commutativity() {
    for a in gf4_all() {
        for b in gf4_all() {
            assert_eq!(a.clone() + b.clone(), b.clone() + a.clone());
        }
    }
}

#[test]
fn test_gf4_add_associativity() {
    for a in gf4_all() {
        for b in gf4_all() {
            for c in gf4_all() {
                assert_eq!(
                    (a.clone() + b.clone()) + c.clone(),
                    a.clone() + (b.clone() + c.clone())
                );
            }
        }
    }
}

#[test]
fn test_gf4_multiplicative_identity() {
    for a in gf4_all() {
        assert_eq!(a.clone() * GF4::one(), a.clone());
        assert_eq!(GF4::one() * a.clone(), a.clone());
    }
}

#[test]
fn test_gf4_multiplicative_inverse() {
    for a in gf4_all() {
        if a == GF4::zero() {
            assert_eq!(a.inv(), None);
        } else {
            let inv = a.inv().expect("nonzero element must have inverse");
            assert_eq!(a.clone() * inv, GF4::one());
        }
    }
}

#[test]
fn test_gf4_mul_commutativity() {
    for a in gf4_all() {
        for b in gf4_all() {
            assert_eq!(a.clone() * b.clone(), b.clone() * a.clone());
        }
    }
}

#[test]
fn test_gf4_mul_zero() {
    for a in gf4_all() {
        assert_eq!(a.clone() * GF4::zero(), GF4::zero());
        assert_eq!(GF4::zero() * a.clone(), GF4::zero());
    }
}

#[test]
fn test_gf4_distributivity() {
    for a in gf4_all() {
        for b in gf4_all() {
            for c in gf4_all() {
                assert_eq!(
                    a.clone() * (b.clone() + c.clone()),
                    a.clone() * b.clone() + a.clone() * c.clone()
                );
            }
        }
    }
}

// ---- Multiplication table ----

#[test]
fn test_gf4_alpha_squared() {
    // α² = α+1, since x² ≡ x+1 mod (x²+x+1)
    assert_eq!(alpha() * alpha(), alpha_plus_one());
}

#[test]
fn test_gf4_alpha_times_alpha_plus_one() {
    // α(α+1) = α²+α = (α+1)+α = 1 in F₂
    assert_eq!(alpha() * alpha_plus_one(), GF4::one());
}

#[test]
fn test_gf4_alpha_plus_one_squared() {
    // (α+1)² = α²+2α+1 = (α+1)+0+1 = α in F₂
    assert_eq!(alpha_plus_one() * alpha_plus_one(), alpha());
}

#[test]
fn test_gf4_generator_is_root_of_modulus() {
    // M(α) = α²+α+1 = (α+1)+α+1 = 0 in F₂
    let a = alpha();
    let m_of_a = a.clone() * a.clone() + a.clone() + GF4::one();
    assert_eq!(m_of_a, GF4::zero());
}

// ---- embed ----

#[test]
fn test_gf4_embed_zero() {
    assert_eq!(GF4::embed(Fp::<2>::zero()), GF4::zero());
}

#[test]
fn test_gf4_embed_one() {
    assert_eq!(GF4::embed(Fp::<2>::one()), GF4::one());
}

#[test]
fn test_gf4_embed_preserves_addition() {
    for a in Fp::<2>::enumerate() {
        for b in Fp::<2>::enumerate() {
            assert_eq!(GF4::embed(a + b), GF4::embed(a) + GF4::embed(b));
        }
    }
}

#[test]
fn test_gf4_embed_preserves_multiplication() {
    for a in Fp::<2>::enumerate() {
        for b in Fp::<2>::enumerate() {
            assert_eq!(GF4::embed(a * b), GF4::embed(a) * GF4::embed(b));
        }
    }
}

#[test]
fn test_gf4_project_to_base() {
    assert_eq!(GF4::zero().project_to_base(), Some(Fp::<2>::zero()));
    assert_eq!(GF4::one().project_to_base(), Some(Fp::<2>::one()));
    assert_eq!(alpha().project_to_base(), None);
    assert_eq!(alpha_plus_one().project_to_base(), None);
}

#[test]
fn test_gf4_project_is_left_inverse_of_embed() {
    // For all k in F_2, project_to_base(embed(k)) = Some(k)
    for k in Fp::<2>::enumerate() {
        assert_eq!(GF4::embed(k).project_to_base(), Some(k));
    }
}

// ---- Frobenius ----

#[test]
fn test_gf4_frobenius_values() {
    // Frobenius is squaring in characteristic 2
    assert_eq!(GF4::zero().frobenius(), GF4::zero());
    assert_eq!(GF4::one().frobenius(), GF4::one());
    assert_eq!(alpha().frobenius(), alpha_plus_one()); // α² = α+1
    assert_eq!(alpha_plus_one().frobenius(), alpha()); // (α+1)² = α
}

#[test]
fn test_gf4_frobenius_iter_2_is_identity() {
    // Frobenius^[GF4:F2] = id
    for a in gf4_all() {
        assert_eq!(a.frobenius_iter(2), a.clone());
    }
}

#[test]
fn test_gf4_frobenius_iter_0_is_identity() {
    for a in gf4_all() {
        assert_eq!(a.frobenius_iter(0), a.clone());
    }
}

#[test]
fn test_gf4_frobenius_preserves_addition() {
    for a in gf4_all() {
        for b in gf4_all() {
            assert_eq!(
                (a.clone() + b.clone()).frobenius(),
                a.frobenius() + b.frobenius()
            );
        }
    }
}

#[test]
fn test_gf4_frobenius_preserves_multiplication() {
    for a in gf4_all() {
        for b in gf4_all() {
            assert_eq!(
                (a.clone() * b.clone()).frobenius(),
                a.frobenius() * b.frobenius()
            );
        }
    }
}

// ---- Trace ----

#[test]
fn test_gf4_trace_values() {
    // Tr(x) = x + Frob(x) since [GF4:F2] = 2
    assert_eq!(GF4::zero().trace(), Fp::<2>::zero());
    assert_eq!(GF4::one().trace(), Fp::<2>::zero()); // 1+1=0 in F₂
    assert_eq!(alpha().trace(), Fp::<2>::one()); // α+(α+1)=1
    assert_eq!(alpha_plus_one().trace(), Fp::<2>::one()); // (α+1)+α=1
}

#[test]
fn test_gf4_trace_is_additive() {
    for a in gf4_all() {
        for b in gf4_all() {
            assert_eq!((a.clone() + b.clone()).trace(), a.trace() + b.trace());
        }
    }
}

#[test]
fn test_gf4_trace_of_embed() {
    // Tr(embed(k)) = k * [GF4:F2] = 2k = 0 for all k in F₂ (char 2)
    for k in Fp::<2>::enumerate() {
        assert_eq!(GF4::embed(k).trace(), Fp::<2>::zero());
    }
}

#[test]
fn test_gf4_trace_via_frobenius_agrees_with_trace() {
    for a in gf4_all() {
        assert_eq!(a.trace_via_frobenius(), a.trace());
    }
}

// ---- Norm ----
//
// FiniteExtension::norm and FieldExtension::norm have the same name but
// different return types. UFCS is required to avoid ambiguity.

#[test]
fn test_gf4_norm_values() {
    // N(x) = x · Frob(x) since [GF4:F2] = 2
    assert_eq!(FiniteExtension::norm(&GF4::zero()), Fp::<2>::zero());
    assert_eq!(FiniteExtension::norm(&GF4::one()), Fp::<2>::one());
    assert_eq!(FiniteExtension::norm(&alpha()), Fp::<2>::one()); // α·(α+1)=1
    assert_eq!(FiniteExtension::norm(&alpha_plus_one()), Fp::<2>::one()); // (α+1)·α=1
}

#[test]
fn test_gf4_norm_is_multiplicative() {
    // N(ab) = N(a)·N(b) for all a, b
    for a in gf4_all() {
        for b in gf4_all() {
            assert_eq!(
                FiniteExtension::norm(&(a.clone() * b.clone())),
                FiniteExtension::norm(&a) * FiniteExtension::norm(&b)
            );
        }
    }
}

#[test]
fn test_gf4_norm_of_embed() {
    // N(embed(k)) = k^[GF4:F2] = k^2 = k for all k in F₂ (since k^2=k in F₂)
    for k in Fp::<2>::enumerate() {
        assert_eq!(FiniteExtension::norm(&GF4::embed(k)), k * k);
    }
}

#[test]
fn test_gf4_norm_zero_iff_zero() {
    // N(a) = 0 ⟺ a = 0, since the multiplicative group has no zero divisors
    for a in gf4_all() {
        assert_eq!(
            FiniteExtension::norm(&a) == Fp::<2>::zero(),
            a == GF4::zero()
        );
    }
}
