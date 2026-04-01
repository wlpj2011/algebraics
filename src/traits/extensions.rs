//! Traits relating to defining extensions of fields.
//! Includes traits for IrreduciblePoly used for doing arithmetic with extensions

use crate::poly::Poly;
use crate::traits::*;

/// Marker trait for irrreducibly poly
///
/// # Contract
/// modulus() must be an irreducible polynomial over F
pub trait IrreduciblePoly<F: Field> {
    fn modulus() -> Poly<F>;
    fn degree() -> usize {
        Self::modulus().degree().unwrap()
    }
}

/// A fully generic field extension E/K
/// BaseField = K
/// Can include infinite extensions
pub trait FieldExtension: Field {
    type BaseField: Field;
    /// embed a BaseField element in K into E
    fn embed(x: Self::BaseField) -> Self;
}

/// Any finite-degree field extension E/K.
/// No assumptions on characteristic or separability.
/// Does NOT include infinite extensions — degree() returning usize
/// encodes finiteness by construction.
pub trait FiniteExtension: FieldExtension {
    /// Degree \[E:K\]
    fn degree() -> usize;
    /// Project element to base if possible (identity for trivial extensions)
    fn project_to_base(&self) -> Option<Self::BaseField>;
    /// Field norm E -> K. Determinant of multiplication map
    fn norm(&self) -> Self::BaseField;
}

/// Separable finite extension — trace possibly well-defined
/// and the trace form is non-degenerate if defined.
/// Trace might not be computatble, so not requiring it
pub trait SeparableExtension: FieldExtension {}

/// Separable finite extension — trace well-defined
/// and the trace form is non-degenerate.
/// Trace is computable as sum of embeddings ffor finite extensions
pub trait SeparableFiniteExtension: SeparableExtension + FiniteExtension {
    /// Field trace E -> K. trace of multiplication map
    fn trace(&self) -> Self::BaseField;
}

/// Char-p extension. Frobenius only exists here.
pub trait CharPFiniteExtension: FiniteExtension + CharPField {
    fn frobenius(&self) -> Self;

    fn frobenius_iter(&self, k: usize) -> Self {
        let mut result = self.clone();
        for _ in 0..k {
            result = result.frobenius();
        }
        result
    }
}

/// Separable char-p extension of finite degree. Trace has an efficient Frobenius-based formula.
pub trait SeparableCharPFiniteExtension: CharPFiniteExtension + SeparableFiniteExtension {
    fn trace_via_frobenius(&self) -> Self::BaseField {
        todo!()
    }
}

// Finite field extension.
// pub trait FiniteFieldExtension: SeparableCharPExtension + FiniteField {}
