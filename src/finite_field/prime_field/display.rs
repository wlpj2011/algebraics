//! Display formatting for `Fp<P>`.
use super::core::Fp;
use std::fmt::Display;

impl<const P: u64> Display for Fp<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
