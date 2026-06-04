use std::ops::Mul;
use crate::field::Fp;
use crate::traits::{One, SeparableCharPFiniteExtension, Zero};
use super::trace_to_u64::TraceToU64;

/// An additive character ψ_a: (F, +) → Z/pZ.
///
/// Evaluation is `ψ_a(x) = Tr_{F/Fp}(a · x)` interpreted as an integer in `0..p`.
///
/// The parameter `a ∈ F` indexes the character:
/// - `a = 0` gives the trivial character (constant 0)
/// - `a = 1` gives the canonical trace character ψ_trace
///
/// Two characters are equal iff their parameters are equal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdditiveCharacter<F> {
    parameter: F,
}

impl<F: Zero> AdditiveCharacter<F> {
    /// The trivial additive character (constant 0).
    pub fn trivial() -> Self {
        AdditiveCharacter { parameter: F::zero() }
    }
}

impl<F: One> AdditiveCharacter<F> {
    /// The canonical trace character ψ_trace: x ↦ Tr_{F/Fp}(x) mod p.
    pub fn canonical() -> Self {
        AdditiveCharacter { parameter: F::one() }
    }
}

impl<F> AdditiveCharacter<F> {
    pub fn with_parameter(a: F) -> Self {
        AdditiveCharacter { parameter: a }
    }

    pub fn parameter(&self) -> &F {
        &self.parameter
    }
}

impl<F: Zero + PartialEq> AdditiveCharacter<F> {
    pub fn is_trivial(&self) -> bool {
        self.parameter == F::zero()
    }
}

#[allow(private_bounds)]
impl<F> AdditiveCharacter<F>
where
    F: SeparableCharPFiniteExtension + Clone,
    F: Mul<Output = F>,
    F::BaseField: TraceToU64,
{
    /// Evaluates ψ_a(x) = Tr_{F/Fp}(a · x) as an integer in `0..p`.
    pub fn eval(&self, x: F) -> u64 {
        (self.parameter.clone() * x).trace_to_u64()
    }
}

impl<const P: u64> AdditiveCharacter<Fp<P>> {
    /// Evaluates ψ_a(x) = (a · x) as an integer in `0..P`.
    pub fn eval(&self, x: Fp<P>) -> u64 {
        (self.parameter * x).value()
    }
}
