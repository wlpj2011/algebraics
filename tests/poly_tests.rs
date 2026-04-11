use algebraics::field::Fp;
use algebraics::poly::Poly;
use algebraics::poly::PolyIter;
use algebraics::traits::EuclideanDomain;
use algebraics::traits::{Finite, One, Zero};

#[test]
fn test_poly_normalization_equality() {
    type F = Fp<7u64>;

    let p1 = Poly::new(vec![F::new(1), F::new(2), F::zero(), F::zero()]);
    let p2 = Poly::new(vec![F::new(1), F::new(2)]);

    assert_eq!(p1, p2);
}

#[test]
fn test_poly_fp7_deg2_add_identity() {
    type CoeffField = Fp<7u64>;
    let n = 2;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        assert_eq!(&p + &Poly::zero(), p);
    }
}

#[test]
fn test_poly_fp7_deg2_mul_identity() {
    type CoeffField = Fp<7u64>;
    let n = 2;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        assert_eq!(&p * &Poly::one(), p);
    }
}

#[test]
fn test_poly_fp7_deg2_additive_inverse() {
    type CoeffField = Fp<7u64>;
    let n = 2;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        assert_eq!(&p + &-(&p), Poly::zero());
    }
}

#[test]
fn test_poly_fp7_deg2_negation_double() {
    type CoeffField = Fp<7u64>;
    let n = 2;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        assert_eq!(-&(-&p), p);
    }
}

#[test]
fn test_poly_fp7_deg2_subtraction() {
    type CoeffField = Fp<7u64>;
    let n = 2;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        for q in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
            assert_eq!(&p - &q, &p + &(-&q));
        }
    }
}

#[test]
fn test_poly_fp7_deg2_add_commutativity() {
    type CoeffField = Fp<7u64>;
    let n = 2;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        for q in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
            assert_eq!(&p + &q, &q + &p);
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
                assert_eq!(&(&p + &q) + &r, &p + &(&q + &r));
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
            assert_eq!(&p * &q, &q * &p);
        }
    }
}

#[test]
fn test_poly_fp7_deg2_mul_zero() {
    type CoeffField = Fp<7u64>;
    let n = 2;
    for p in PolyIter::<CoeffField>::all_of_bounded_degree(n) {
        assert_eq!(&p * &Poly::zero(), Poly::zero());
        assert_eq!(&Poly::zero() * &p, Poly::zero());
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
            let sum_deg = (&p1 + &p2).degree();
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
            let prod_deg = (&p1 * &p2).degree().unwrap();
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

#[test]
fn test_poly_fp5_div_rem() {
    type CoeffField = Fp<5u64>;
    let f = Poly::new(vec![
        CoeffField::new(1),
        CoeffField::new(3),
        CoeffField::new(4),
        CoeffField::new(2),
    ]); // Creates 1 + 3*x + 4*x^2 + 2*x^3
    let g = Poly::new(vec![CoeffField::new(2), CoeffField::new(1)]);
    let (q, r) = f.div_rem(g);
    assert_eq!(
        (q, r),
        (
            Poly::new(vec![
                CoeffField::new(3),
                CoeffField::zero(),
                CoeffField::new(2)
            ]),
            Poly::zero()
        )
    );
}

#[test]
fn test_poly_fp7_div_rem_all_bounded() {
    type F = Fp<7u64>;

    for f in PolyIter::<F>::all_of_bounded_degree(3) {
        for g in PolyIter::<F>::all_of_bounded_degree(2) {
            if g.is_zero() {
                continue;
            }

            let (q, r) = f.clone().div_rem(g.clone());

            // Reconstruct f from quotient and remainder
            let reconstructed = &(&q * &g) + &r;
            assert_eq!(reconstructed, f, "Failed for f={:?}, g={:?}", f, g);

            // Check remainder degree
            if !r.is_zero() {
                assert!(r.degree().unwrap() < g.degree().unwrap());
            }
        }
    }
}

#[test]
fn test_poly_fp7_div_non_monic() {
    type F = Fp<7u64>;

    let f = Poly::new(vec![F::new(1), F::new(2), F::new(3)]);
    let g = Poly::new(vec![F::new(2), F::new(2)]); // not monic

    let (q, r) = f.clone().div_rem(g.clone());

    let reconstructed = &(&q * &g) + &r;
    assert_eq!(reconstructed, f);
}

#[test]
fn test_poly_fp7_high_degree_mul() {
    type F = Fp<7u64>;

    let p = Poly::new((0..20).map(|i| F::new(i % 7)).collect());
    let q = Poly::new((0..20).map(|i| F::new((2 * i) % 7)).collect());

    let prod = &p * &q;

    // Basic sanity: degree check
    assert_eq!(
        prod.degree().unwrap(),
        p.degree().unwrap() + q.degree().unwrap()
    );
}

#[test]
fn test_poly_fp11_sampled_arithmetic() {
    type F = Fp<11u64>;

    for i in 0..50 {
        let p = Poly::new((0..10).map(|j| F::new((i + j * 3) % 11)).collect());
        let q = Poly::new((0..10).map(|j| F::new((i * 2 + j) % 11)).collect());
        let r = Poly::new((0..10).map(|j| F::new((i + j * 5) % 11)).collect());

        // Distributivity
        assert_eq!(&p * &(&q + &r), &(&p * &q) + &(&p * &r));
    }
}

#[test]
fn test_poly_fp7_high_degree_division() {
    type F = Fp<7u64>;

    let f = Poly::new((0..15).map(|i| F::new(i % 7)).collect());
    let g = Poly::new((0..5).map(|i| F::new((i + 1) % 7)).collect());

    let (q, r) = f.clone().div_rem(g.clone());

    let reconstructed = &(&q * &g) + &r;
    assert_eq!(reconstructed, f);

    if !r.is_zero() {
        assert!(r.degree().unwrap() < g.degree().unwrap());
    }
}

#[test]
fn test_poly_fp97_basic_properties() {
    type F = Fp<97u64>;

    let p = Poly::new(vec![F::new(3), F::new(45), F::new(2)]);
    let q = Poly::new(vec![F::new(10), F::new(1)]);

    // Check distributivity
    let lhs = &p * &(&q + &Poly::one());
    let rhs = &(&p * &q) + &p;

    assert_eq!(lhs, rhs);
}
