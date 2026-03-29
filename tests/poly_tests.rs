use algebraics::poly::Poly;
use algebraics::prime_field::Fp;
use algebraics::traits::{Field, One, Ring, Zero};

// Test Ring Behaviour
#[test]
fn test_poly_fp7_deg2_add_identity() {
    for coeff0 in 0..7u64 {
        for coeff1 in 0..7u64 {
            for coeff2 in 0..7u64 {
                type CoeffField = Fp<7u64>;
                let p: Poly<CoeffField> = Poly::new(vec![
                    CoeffField::new(coeff0),
                    CoeffField::new(coeff1),
                    CoeffField::new(coeff2),
                ]);
                assert_eq!(p.clone() + Poly::zero(), p);
            }
        }
    }
}

#[test]
fn test_poly_fp7_deg2_mul_identity() {
    for coeff0 in 0..7u64 {
        for coeff1 in 0..7u64 {
            for coeff2 in 0..7u64 {
                type CoeffField = Fp<7u64>;
                let p: Poly<CoeffField> = Poly::new(vec![
                    CoeffField::new(coeff0),
                    CoeffField::new(coeff1),
                    CoeffField::new(coeff2),
                ]);
                assert_eq!(p.clone() * Poly::one(), p);
            }
        }
    }
}

// Test degree behavior
#[test]
fn test_degree_zero_poly() {
    type CoeffField = Fp<7u64>;
    let zero: Poly<CoeffField> = Poly::zero();
    assert!(zero.degree().is_none());
}

#[test]
fn test_degree_constant() {
    type CoeffField = Fp<7u64>;
    for a in 1..7u64 {
        let p: Poly<CoeffField> = Poly::new(vec![CoeffField::new(a)]);
        assert_eq!(p.degree().unwrap(), 0);
    }
}

// This only works for Coefficients in a IntegralDomain
#[test]
fn test_degree_sum() {
    type CoeffField = Fp<7u64>;
    for a0 in 0..7u64 {
        for a1 in 0..7u64 {
            for b0 in 0..7u64 {
                for b1 in 0..7u64 {
                    if (a0, a1) == (0, 0) || (b0, b1) == (0, 0) {
                        continue;
                    }
                    let a: Poly<CoeffField> =
                        Poly::new(vec![CoeffField::new(a0), CoeffField::new(a1)]);
                    let b: Poly<CoeffField> =
                        Poly::new(vec![CoeffField::new(b0), CoeffField::new(b1)]);

                    let sum_deg = (a.clone() + b.clone()).degree();
                    let max_deg = a.degree().unwrap().max(b.degree().unwrap());
                    assert!(sum_deg.map_or(true, |d| d <= max_deg));
                }
            }
        }
    }
}

#[test]
fn test_degree_mul() {
    type F = Fp<7u64>;
    for a1 in 1..7u64 {
        for b1 in 1..7u64 {
            for a0 in 0..7u64 {
                for b0 in 0..7u64 {
                    let a = Poly::new(vec![F::new(a0), F::new(a1)]);
                    let b = Poly::new(vec![F::new(b0), F::new(b1)]);
                    let prod_deg = (a.clone() * b.clone()).degree().unwrap();
                    let sum_of_deg = a.degree().unwrap() + b.degree().unwrap();
                    assert_eq!(prod_deg, sum_of_deg);
                }
            }
        }
    }
}
