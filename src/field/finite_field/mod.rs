//! Finite fields of prime size
//!
//! This module provides:
//!
//! - **`Fp<P: u64>`** – A finite field type with for a prime `P`.
//! - Eventually other finite fields
pub mod fpn;
pub mod prime_field;

pub use fpn::FpN;
pub use prime_field::core::Fp;
