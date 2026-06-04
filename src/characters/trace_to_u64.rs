use crate::field::Fp;
use crate::traits::SeparableCharPFiniteExtension;

/// Extracts the absolute trace of a finite field element as a `u64` in `0..p`.
///
/// For `Fp<P>`: returns the element value directly.
/// For extension fields: applies `trace_via_frobenius` recursively down to the prime field.
pub(crate) trait TraceToU64 {
    fn trace_to_u64(self) -> u64;
}

impl<const P: u64> TraceToU64 for Fp<P> {
    fn trace_to_u64(self) -> u64 {
        self.value()
    }
}

// Fp<P> does not implement SeparableCharPFiniteExtension, so there is no overlap.
impl<F> TraceToU64 for F
where
    F: SeparableCharPFiniteExtension,
    F::BaseField: TraceToU64,
{
    fn trace_to_u64(self) -> u64 {
        self.trace_via_frobenius().trace_to_u64()
    }
}
