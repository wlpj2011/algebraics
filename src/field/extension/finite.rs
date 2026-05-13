use std::marker::PhantomData;

use crate::field::FiniteSimpleExtension;
use crate::poly::PolyIter;
use crate::traits::*;

/// Enumerates all `F.size()^n` elements by iterating polynomials of degree `< n` over `F`.
impl<F: FiniteField, M: IrreduciblePoly<F>> Finite for FiniteSimpleExtension<F, M> {
    fn enumerate() -> impl Iterator<Item = Self> {
        PolyIter::<F>::all_of_bounded_degree(M::degree() - 1).map(|p| Self {
            repr: p,
            _m: PhantomData,
        })
    }
    fn size() -> usize {
        F::size().pow(M::degree() as u32)
    }
}

/// Every non-zero element of a field is a unit.
impl<F: FiniteField, M: IrreduciblePoly<F>> FiniteRing for FiniteSimpleExtension<F, M> {
    fn is_unit(&self) -> bool {
        !self.is_zero()
    }
}

/// The image of `x` in `F\[x\]/(M)` generates the multiplicative group.
impl<F: FiniteField, M: IrreduciblePoly<F>> HasMultiplicativeGenerator
    for FiniteSimpleExtension<F, M>
{
    fn multiplicative_generator() -> Self {
        Self::generator()
    }
}
