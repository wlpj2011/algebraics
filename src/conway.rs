//! Conway polynomial lookup for finite field extensions.
//!
//! Conway polynomials are a canonical choice of defining polynomial for GF(p^n):
//! they are primitive (their root generates the multiplicative group) and satisfy
//! compatibility conditions that make them the standard choice for constructing
//! field towers.
//!
//! The table is Frank Lübeck's precomputed list, compiled into the binary at
//! build time via `build.rs`. Not all `(p, n)` pairs are tabulated; lookups
//! return `None` when the entry is absent.

use crate::field::Fp;
use crate::poly::Poly;

// src/conway.rs
include!(concat!(env!("OUT_DIR"), "/conway_table.rs"));
// generated file contains only COEFFS_* and CONWAY_TABLE

/// Looks up the Conway polynomial for GF(p^n), returning its coefficients in
/// ascending degree order, or `None` if the pair is not in the table.
///
/// # Examples
/// ```
/// use algebraics::conway::conway_poly;
/// assert_eq!(conway_poly(2, 1), Some(&[1u64, 1][..])); // x + 1 over F_2
/// assert!(conway_poly(2, 10000).is_none()); // degree too large, not tabulated
/// ```
pub fn conway_poly(p: u64, n: u64) -> Option<&'static [u64]> {
    match CONWAY_TABLE.binary_search_by_key(&(p, n), |&(pp, nn, _)| (pp, nn)) {
        Ok(idx) => Some(CONWAY_TABLE[idx].2),
        Err(_) => None,
    }
}

/// Looks up the Conway polynomial for GF(p^n) as a `Poly<Fp<P>>`, or `None`
/// if the pair is not in the table.
///
/// # Examples
/// ```
/// use algebraics::conway::conway_poly_fp;
/// let p = conway_poly_fp::<2>(1).unwrap(); // x + 1 over F_2
/// assert_eq!(p.degree(), Some(1));
/// ```
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
