use crate::field::Fp;
use crate::poly::Poly;

// src/conway.rs
include!(concat!(env!("OUT_DIR"), "/conway_table.rs"));
// generated file contains only COEFFS_* and CONWAY_TABLE

pub fn conway_poly(p: u64, n: u64) -> Option<&'static [u64]> {
    match CONWAY_TABLE.binary_search_by_key(&(p, n), |&(pp, nn, _)| (pp, nn)) {
        Ok(idx) => Some(CONWAY_TABLE[idx].2),
        Err(_) => None,
    }
}

pub fn conway_poly_fp<const P: u64>(n: u64) -> Option<Poly<Fp<P>>> {
    conway_poly(P, n).map(|coeffs| Poly::new(coeffs.iter().map(|&c| Fp::<P>::new(c)).collect()))
}

#[test]
fn test_conway() {
    use crate::traits::Zero;
    assert!(conway_poly(2, 1).is_some());
    assert_eq!(conway_poly(2, 1).unwrap(), &[1u64, 1]);
    assert_eq!(
        conway_poly_fp::<3>(3),
        Some(Poly::new(vec![
            Fp::<3>::new(1),
            Fp::<3>::new(2),
            Fp::<3>::zero(),
            Fp::<3>::new(1)
        ]))
    );
}
