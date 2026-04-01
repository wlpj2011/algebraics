use std::marker::PhantomData;

use crate::poly::Poly;
use crate::traits::*;

#[derive(Debug, Eq)]
pub struct FiniteSimpleExtension<F: Field, M: IrreduciblePoly<F>> {
    pub(crate) repr: Poly<F>,
    _m: PhantomData<M>, // M is only used as a const modulus, never stored
}

impl<F: Field, M: IrreduciblePoly<F>> FiniteSimpleExtension<F, M> {
    pub fn new(repr: Poly<F>) -> Self {
        let reduced_repr = repr.div_rem(M::modulus()).1;
        FiniteSimpleExtension {
            repr: reduced_repr,
            _m: PhantomData,
        }
    }
}

impl<F: Field, M: IrreduciblePoly<F>> Clone for FiniteSimpleExtension<F, M> {
    fn clone(&self) -> Self {
        FiniteSimpleExtension {
            repr: self.repr.clone(),
            _m: PhantomData,
        }
    }
}

impl<F: Field, M: IrreduciblePoly<F>> PartialEq for FiniteSimpleExtension<F, M> {
    fn eq(&self, other: &Self) -> bool {
        self.repr == other.repr
    }
}

impl<F: Field, M: IrreduciblePoly<F>> Zero for FiniteSimpleExtension<F, M> {
    fn zero() -> Self {
        FiniteSimpleExtension {
            repr: Poly::zero(),
            _m: PhantomData,
        }
    }

    fn is_zero(&self) -> bool {
        self.repr.is_zero()
    }
}

impl<F: Field, M: IrreduciblePoly<F>> One for FiniteSimpleExtension<F, M> {
    fn one() -> Self {
        FiniteSimpleExtension {
            repr: Poly::one(),
            _m: PhantomData,
        }
    }
}

impl<F: Field, M: IrreduciblePoly<F>> Magma for FiniteSimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> Semigroup for FiniteSimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> Monoid for FiniteSimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> Group for FiniteSimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> AbelianGroup for FiniteSimpleExtension<F, M> {}

impl<F: Field, M: IrreduciblePoly<F>> Ring for FiniteSimpleExtension<F, M> {
    fn characteristic() -> u64 {
        F::characteristic()
    }
}

impl<F: Field, M: IrreduciblePoly<F>> CommutativeRing for FiniteSimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> IntegralDomain for FiniteSimpleExtension<F, M> {}

impl<F: Field, M: IrreduciblePoly<F>> Field for FiniteSimpleExtension<F, M> {
    fn inv(&self) -> Option<Self> {
        todo!()
    }
}

impl<F: Field, M: IrreduciblePoly<F>> FieldExtension for FiniteSimpleExtension<F, M> {
    type BaseField = F;

    fn embed(x: Self::BaseField) -> Self {
        Self {
            repr: Poly::new_constant(x),
            _m: PhantomData,
        }
    }
}

impl<F: Field, M: IrreduciblePoly<F>> FiniteExtension for FiniteSimpleExtension<F, M> {
    fn degree() -> usize {
        M::degree()
    }

    fn project_to_base(&self) -> Option<Self::BaseField> {
        todo!()
    }

    fn norm(&self) -> Self::BaseField {
        todo!()
    }
}

/// Blanket: any extension of a perfect field is separable.
impl<F: PerfectField, M: IrreduciblePoly<F>> SeparableExtension for FiniteSimpleExtension<F, M> {}

impl<F: PerfectField, M: IrreduciblePoly<F>> SeparableFiniteExtension
    for FiniteSimpleExtension<F, M>
{
    fn trace(&self) -> Self::BaseField {
        todo!()
    }
}
