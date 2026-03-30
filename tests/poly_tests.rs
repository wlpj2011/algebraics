use algebraics::finite_field::Fp;
use algebraics::poly::Poly;
use algebraics::poly::PolyIter;
use algebraics::traits::{Finite, One, Zero};

#[test]
fn test_poly_fp7_deg2_add_identity() {
    type CoeffField = Fp<7u64>;
    let n = 2;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        assert_eq!(p.clone() + Poly::zero(), p);
    }
}

#[test]
fn test_poly_fp7_deg2_mul_identity() {
    type CoeffField = Fp<7u64>;
    let n = 2;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        assert_eq!(p.clone() * Poly::one(), p);
    }
}

#[test]
fn test_poly_fp7_deg2_additive_inverse() {
    type CoeffField = Fp<7u64>;
    let n = 2;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        assert_eq!(p.clone() + (-p.clone()), Poly::zero());
    }
}

#[test]
fn test_poly_fp7_deg2_negation_double() {
    type CoeffField = Fp<7u64>;
    let n = 2;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        assert_eq!(-(-p.clone()), p);
    }
}

#[test]
fn test_poly_fp7_deg2_subtraction() {
    type CoeffField = Fp<7u64>;
    let n = 2;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        for q in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
            assert_eq!(p.clone() - q.clone(), p.clone() + (-q.clone()));
        }
    }
}

#[test]
fn test_poly_fp7_deg2_add_commutativity() {
    type CoeffField = Fp<7u64>;
    let n = 2;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        for q in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
            assert_eq!(p.clone() + q.clone(), q.clone() + p.clone());
        }
    }
}

#[test]
fn test_poly_fp7_deg1_add_associativity() {
    type CoeffField = Fp<7u64>;
    let n = 1;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        for q in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
            for r in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
                assert_eq!(
                    (p.clone() + q.clone()) + r.clone(),
                    p.clone() + (q.clone() + r.clone())
                );
            }
        }
    }
}

#[test]
fn test_poly_fp7_deg1_mul_commutativity() {
    type CoeffField = Fp<7u64>;
    let n = 1;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        for q in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
            assert_eq!(p.clone() * q.clone(), q.clone() * p.clone());
        }
    }
}

#[test]
fn test_poly_fp7_deg2_mul_zero() {
    type CoeffField = Fp<7u64>;
    let n = 2;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        assert_eq!(p.clone() * Poly::zero(), Poly::zero());
        assert_eq!(Poly::zero() * p.clone(), Poly::zero());
    }
}

#[test]
fn test_poly_fp7_zero_degree() {
    type CoeffField = Fp<7u64>;
    let zero: Poly<CoeffField> = Poly::zero();
    assert!(zero.degree().is_none());
}

#[test]
fn test_poly_fp7_deg0_exact_degree() {
    type CoeffField = Fp<7u64>;
    for p in PolyIter::<CoeffField>::all_of_exact_degree(0) {
        assert_eq!(p.degree().unwrap(), 0);
    }
}

#[test]
fn test_poly_fp7_deg1_degree_sum() {
    type CoeffField = Fp<7u64>;
    let n = 1;
    for p1 in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        for p2 in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
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
fn test_poly_fp7_deg1_degree_mul() {
    type CoeffField = Fp<7u64>;
    let n = 1;
    for p1 in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        for p2 in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
            if p1 == Poly::zero() || p2 == Poly::zero() {
                continue;
            }
            let prod_deg = (p1.clone() * p2.clone()).degree().unwrap();
            let sum_deg = p1.degree().unwrap() + p2.degree().unwrap();
            assert_eq!(prod_deg, sum_deg);
        }
    }
}

#[test]
fn test_poly_fp7_deg2_normalize_trailing_zeros() {
    type CoeffField = Fp<7u64>;
    let p = Poly::new(vec![
        CoeffField::zero(),
        CoeffField::zero(),
        CoeffField::one(),
    ]);
    assert_eq!(p.degree().unwrap(), 2);

    let p2 = Poly::new(vec![
        CoeffField::zero(),
        CoeffField::zero(),
        CoeffField::zero(),
    ]);
    assert!(p2.degree().is_none());
}

#[test]
fn test_poly_fp7_deg2_iterator_len_tracking() {
    type CoeffField = Fp<7>;
    let n = 2;
    let mut iter = PolyIter::<CoeffField>::all_of_bounded_degree(n);

    let total = CoeffField::size().pow((n + 1) as u32);
    assert_eq!(iter.len(), total);

    // Consume a few elements
    iter.next();
    iter.next();
    assert_eq!(iter.len(), total - 2);

    // Consume the rest
    let _ = iter.by_ref().count();
    assert_eq!(iter.len(), 0);
}

#[test]
fn test_poly_fp7_deg1_exact_degree_iterator() {
    type CoeffField = Fp<7u64>;
    let n = 1;
    for p in PolyIter::<CoeffField>::all_of_exact_degree(n) {
        assert_eq!(p.degree().unwrap(), n);
    }
}
