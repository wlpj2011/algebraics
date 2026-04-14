use crate::conway::conway_poly_fp;
use crate::field::FiniteSimpleExtension;
use crate::field::Fp;
use crate::poly::Poly;
use crate::traits::*;

pub struct ConwayPoly<const P: u64, const N: u64>;

impl<const P: u64, const N: u64> IrreduciblePoly<Fp<P>> for ConwayPoly<P, N> {
    fn modulus() -> Poly<Fp<P>> {
        conway_poly_fp::<P>(N).expect("Conway polynomial not in table")
    }
}

pub type FpN<const P: u64, const N: u64> = FiniteSimpleExtension<Fp<P>, ConwayPoly<P, N>>;
