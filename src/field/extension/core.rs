use std::marker::PhantomData;

use crate::traits::*;
use crate::poly::Poly;

#[derive(Debug, Eq)]
pub struct SimpleExtension<F: Field, M: IrreduciblePoly<F>> {
    repr: Poly<F>,
    _m: PhantomData<M>,   // M is only used as a const modulus, never stored
}

impl<F: Field, M: IrreduciblePoly<F>> Clone for SimpleExtension<F, M> {
    fn clone(&self) -> Self {
        Self {
            repr: self.repr.clone(),
            _m: PhantomData,
        }
    }
}

impl<F: Field, M: IrreduciblePoly<F>> PartialEq for SimpleExtension<F, M> {
    fn eq(&self, other: &Self) -> bool {
        self.repr == other.repr
    }
}

impl<F: Field, M: IrreduciblePoly<F>> Zero for SimpleExtension<F, M> {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl<F: Field, M: IrreduciblePoly<F>> One for SimpleExtension<F, M> {
    fn one() -> Self {
        todo!()
    }
}

impl<F: Field, M: IrreduciblePoly<F>> Magma for SimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> Semigroup for SimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> Monoid for SimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> Group for SimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> AbelianGroup for SimpleExtension<F, M> {}

impl<F: Field, M: IrreduciblePoly<F>> Ring for SimpleExtension<F, M> {
    fn characteristic() -> u64 {
        todo!()
    }
}

impl<F: Field, M: IrreduciblePoly<F>> CommutativeRing for SimpleExtension<F, M> {}
impl<F: Field, M: IrreduciblePoly<F>> IntegralDomain for SimpleExtension<F, M> {}

impl<F: Field, M: IrreduciblePoly<F>> Field for SimpleExtension<F, M> {
    fn inv(&self) -> Option<Self> {
        todo!()
    }
}

impl<F: Field, M: IrreduciblePoly<F>> FieldExtension for SimpleExtension<F,M> {
    type BaseField = F;

    fn embed(x: Self::BaseField) -> Self {
        Self{repr: Poly::new_constant(x), _m: PhantomData}
    }
}


/// Blanket: any extension of a perfect field is separable.
impl<F: PerfectField, M: IrreduciblePoly<F>> SeparableExtension
    for SimpleExtension<F, M>
{
    fn trace(&self) -> F { todo!() }
}