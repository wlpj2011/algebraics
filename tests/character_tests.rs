use algebraics::characters::{
    stickelberger_data_for, stickelberger_element, AdditiveCharacter, GaussSum,
    MultiplicativeCharacter,
};
use algebraics::field::{Fp, FpN};
use algebraics::traits::{Finite, FiniteField, HasMultiplicativeGenerator, One, Zero};

// ── HasMultiplicativeGenerator for Fp<P> ─────────────────────────────────────

#[test]
fn primitive_root_fp7_has_order_six() {
    let g = Fp::<7>::multiplicative_generator();
    // g^6 = 1  (Fermat)
    let mut cur = Fp::<7>::one();
    for _ in 0..6 {
        cur = cur * g;
    }
    assert_eq!(cur, Fp::<7>::one());
    // g^k ≠ 1 for k ∈ {1,2,3}  (order exactly 6)
    let mut cur = g;
    for _ in 1..6 {
        assert_ne!(cur, Fp::<7>::one());
        cur = cur * g;
    }
}

#[test]
fn primitive_root_fp5() {
    let g = Fp::<5>::multiplicative_generator();
    // order divides 4; check all proper divisors
    use algebraics::arithmetic::mod_pow;
    let gv = g.value();
    assert_ne!(mod_pow(gv, 1, 5), 1);
    assert_ne!(mod_pow(gv, 2, 5), 1);
    assert_eq!(mod_pow(gv, 4, 5), 1);
}

#[test]
fn primitive_root_fp13() {
    let g = Fp::<13>::multiplicative_generator();
    use algebraics::arithmetic::mod_pow;
    let gv = g.value();
    // order 12; check that g^k != 1 for k in {1,2,3,4,6}
    for k in [1u64, 2, 3, 4, 6] {
        assert_ne!(mod_pow(gv, k, 13), 1, "g^{k} should not be 1");
    }
    assert_eq!(mod_pow(gv, 12, 13), 1);
}

// ── AdditiveCharacter on prime fields ────────────────────────────────────────

#[test]
fn additive_char_trivial_is_always_zero() {
    let psi = AdditiveCharacter::<Fp<5>>::trivial();
    for x in Fp::<5>::enumerate() {
        assert_eq!(psi.eval(x), 0);
    }
}

#[test]
fn additive_char_canonical_fp5_is_identity() {
    let psi = AdditiveCharacter::<Fp<5>>::canonical();
    for x in Fp::<5>::enumerate() {
        assert_eq!(psi.eval(x), x.value());
    }
}

#[test]
fn additive_char_fp5_linearity() {
    // ψ_a(x + y) == (ψ_a(x) + ψ_a(y)) mod 5
    let psi = AdditiveCharacter::<Fp<5>>::with_parameter(Fp::<5>::new(3));
    for x in Fp::<5>::enumerate() {
        for y in Fp::<5>::enumerate() {
            let lhs = psi.eval(x + y);
            let rhs = (psi.eval(x) + psi.eval(y)) % 5;
            assert_eq!(lhs, rhs);
        }
    }
}

// ── AdditiveCharacter on extension fields ─────────────────────────────────────

#[test]
fn trace_character_gf25_agrees_with_trace() {
    use algebraics::traits::SeparableCharPFiniteExtension;
    type F = FpN<5, 2>;
    let psi = AdditiveCharacter::<F>::canonical();
    for x in F::enumerate() {
        let via_char = psi.eval(x.clone());
        let via_trace = x.trace_via_frobenius().value();
        assert_eq!(via_char, via_trace);
    }
}

#[test]
fn additive_char_gf25_linearity() {
    use algebraics::field::extension::FiniteSimpleExtension;
    type F = FpN<5, 2>;
    let a = FiniteSimpleExtension::generator();
    let psi = AdditiveCharacter::<F>::with_parameter(a);
    for x in F::enumerate() {
        for y in F::enumerate() {
            let lhs = psi.eval(x.clone() + y.clone());
            let rhs = (psi.eval(x.clone()) + psi.eval(y)) % 5;
            assert_eq!(lhs, rhs);
        }
    }
}

// ── MultiplicativeCharacter ───────────────────────────────────────────────────

#[test]
fn mult_char_trivial_fp7_always_zero_exponent() {
    type F = Fp<7>;
    let chi = MultiplicativeCharacter::<F>::trivial();
    assert!(chi.is_trivial());
    assert_eq!(chi.order(), 1);
    for x in F::multiplicative_group() {
        assert_eq!(chi.eval(&x), Some(0));
    }
}

#[test]
fn mult_char_order_fp7() {
    type F = Fp<7>;
    // q-1 = 6; gcd(2,6) = 2 → order = 3
    let chi2 = MultiplicativeCharacter::<F>::with_index(2);
    assert_eq!(chi2.order(), 3);
    // gcd(1,6) = 1 → order = 6
    let chi1 = MultiplicativeCharacter::<F>::teichmuller();
    assert_eq!(chi1.order(), 6);
    // gcd(3,6) = 3 → order = 2
    let chi3 = MultiplicativeCharacter::<F>::with_index(3);
    assert_eq!(chi3.order(), 2);
}

#[test]
fn mult_char_eval_zero_is_none() {
    type F = Fp<7>;
    let chi = MultiplicativeCharacter::<F>::teichmuller();
    assert_eq!(chi.eval(&Fp::<7>::zero()), None);
}

#[test]
fn teichmuller_fp7_is_homomorphism() {
    type F = Fp<7>;
    let chi = MultiplicativeCharacter::<F>::teichmuller();
    // χ(xy) == (χ(x) + χ(y)) mod (q-1)
    let group_order = (F::size() - 1) as u64;
    for x in F::multiplicative_group() {
        for y in F::multiplicative_group() {
            let lhs = chi.eval(&(x * y)).unwrap();
            let rhs = (chi.eval(&x).unwrap() + chi.eval(&y).unwrap()) % group_order;
            assert_eq!(lhs, rhs);
        }
    }
}

#[test]
fn teichmuller_gf25_order_is_24() {
    type F = FpN<5, 2>;
    let chi = MultiplicativeCharacter::<F>::teichmuller();
    assert_eq!(chi.order(), 24); // q-1 = 24
}

// ── Discrete log round-trip ───────────────────────────────────────────────────

#[test]
fn discrete_log_round_trip_fp7() {
    type F = Fp<7>;
    let g = F::multiplicative_generator();
    let chi = MultiplicativeCharacter::<F>::teichmuller();
    let mut power = F::one();
    for k in 0..6u64 {
        assert_eq!(chi.eval(&power), Some(k));
        power = power * g;
    }
}

// ── GaussSum phase table ──────────────────────────────────────────────────────

#[test]
fn gauss_sum_phase_table_size_gf25() {
    type F = FpN<5, 2>;
    let gs = GaussSum::<F>::standard(1);
    let table = gs.phase_table();
    // one entry per element of F* → q-1 = 24
    assert_eq!(table.len(), 24);
    let group_order = (F::size() - 1) as u64;
    for (chi_exp, psi_exp) in &table {
        assert!(*chi_exp < group_order, "chi_exp out of range: {chi_exp}");
        assert!(*psi_exp < 5, "psi_exp out of range: {psi_exp}");
    }
}

#[test]
fn trivial_chi_phase_table_all_zero_chi_exp() {
    type F = FpN<5, 2>;
    let gs = GaussSum::<F>::new(
        MultiplicativeCharacter::trivial(),
        AdditiveCharacter::canonical(),
    );
    for (chi_exp, _) in gs.phase_table() {
        assert_eq!(chi_exp, 0);
    }
}

// ── Stickelberger data ────────────────────────────────────────────────────────

#[test]
fn p_adic_valuation_digit_sum() {
    // j = 3 over F_{5^2}: base-5 digits of 3 are [3], digit sum = 3, denom = p-1 = 4
    let data = stickelberger_data_for(5, 25, 3);
    assert_eq!(data.p_adic_valuation, (3, 4));

    // j = 6 = 1*5 + 1: digit sum = 2
    let data2 = stickelberger_data_for(5, 25, 6);
    assert_eq!(data2.p_adic_valuation, (2, 4));
}

#[test]
fn stickelberger_exponents_fp5_j2() {
    // q=5, group_order=4; Galois group = {1,3} (units mod 4)
    // j=2: floor(2*1/4)=0, floor(2*3/4)=1
    let data = stickelberger_data_for(5, 5, 2);
    let exp_map: std::collections::HashMap<u64, u64> =
        data.galois_exponents.into_iter().collect();
    assert_eq!(exp_map[&1], 0);
    assert_eq!(exp_map[&3], 1);
}

#[test]
fn stickelberger_exponents_fp7_j1() {
    // q=7, group_order=6; Galois group = {1,5} (units mod 6)
    // j=1: floor(1/6)=0, floor(5/6)=0
    let data = stickelberger_data_for(7, 7, 1);
    for (_, exp) in &data.galois_exponents {
        assert_eq!(*exp, 0);
    }
}

#[test]
fn stickelberger_element_fp5_coefficients() {
    // q=5, group_order=4; units mod 4 are {1,3}
    // θ = (1/4)σ_1^{-1} + (3/4)σ_3^{-1}
    let elem = stickelberger_element(5);
    let map: std::collections::HashMap<u64, (u64, u64)> = elem.into_iter().collect();
    assert_eq!(map[&1], (1, 4));
    assert_eq!(map[&3], (3, 4));
}

#[test]
fn gauss_sum_stickelberger_data_gf25_j3() {
    type F = FpN<5, 2>;
    let gs = GaussSum::<F>::standard(3);
    let data = gs.stickelberger_data();
    assert_eq!(data.p, 5);
    assert_eq!(data.q, 25);
    assert_eq!(data.char_index, 3);
    assert_eq!(data.p_adic_valuation, (3, 4));
}
