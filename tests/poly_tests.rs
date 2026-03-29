use algebraics::poly::Poly;
use algebraics::poly::PolyIter;
use algebraics::prime_field::Fp;
use algebraics::traits::{Field, One, Ring, Zero};

// Test Ring Behaviour
#[test]
fn test_poly_fp7_deg2_add_identity() {
    type CoeffField = Fp<7u64>;
    let polys = PolyIter::<CoeffField>::all_of_bounded_degree(3);
    for p in polys {
        assert_eq!(p.clone() + Poly::zero(), p);
    }
}

#[test]
fn test_poly_fp7_deg2_mul_identity() {
    type CoeffField = Fp<7u64>;
    let polys = PolyIter::<CoeffField>::all_of_bounded_degree(3);
    for p in polys {
        assert_eq!(p.clone() * Poly::one(), p);
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
    let polys = PolyIter::<CoeffField>::all_of_exact_degree(0);
    for p in polys {
        assert_eq!(p.degree().unwrap(), 0);
    }
}

// This only works for Coefficients in a IntegralDomain
#[test]
fn test_degree_sum() {
    type CoeffField = Fp<7u64>;
    let polys1 = PolyIter::<CoeffField>::all_of_bounded_degree(1);
    for p1 in polys1 {
        let polys2 = PolyIter::<CoeffField>::all_of_bounded_degree(1);
        for p2 in polys2 {
            if p1 == Poly::zero() || p2 == Poly::zero() {
                continue;
            }
            let sum_deg = (p1.clone() + p2.clone()).degree();
            let max_deg = p1.degree().unwrap().max(p2.degree().unwrap());
            assert!(sum_deg.map_or(true, |d| d <= max_deg));
        }
    }
}

#[test]
fn test_degree_mul() {
    type CoeffField = Fp<7u64>;
    let polys1 = PolyIter::<CoeffField>::all_of_bounded_degree(1);
    for p1 in polys1 {
        let polys2 = PolyIter::<CoeffField>::all_of_bounded_degree(1);
        for p2 in polys2 {
            if p1 == Poly::zero() || p2 == Poly::zero() {
                continue;
            }
            let prod_deg = (p1.clone() * p2.clone()).degree().unwrap();
            let sum_of_deg = p1.degree().unwrap() + p2.degree().unwrap();
            assert_eq!(prod_deg, sum_of_deg);
        }
    }
}
