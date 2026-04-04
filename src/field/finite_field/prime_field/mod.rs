//! Finite fields of prime size
//!
//! This module provides:
//!
//! - **`Fp<P: u64>`** – A finite field type with for a prime `P`.
//!   - Supports addition, subtraction, multiplication, and division.
//!   - Construction utomatically normalizes to 0 <= x < P
//!   - Implements `Display` for pretty-printing.
//!
//! # Examples
//! ```
//! # use algebraics::field::Fp;
//! # use algebraics::traits::{Field, Zero, One};
//!
//! type F7 = Fp<7>;
//!
//! // Check 3 == 10 mod 7
//! let a = F7::new(3);
//! let b = F7::new(10);
//! assert_eq!(a,b);
//!
//! // Check 3 * 5 == 1 mod 7
//! let a_inv = F7::new(5);
//! assert_eq!(a_inv, a.inv().unwrap()); //Note `unwrap()` is necessary for `inv()` to protect against zero-divison.
//! assert_eq!(a * a_inv, F7::one());
//! ```
pub mod core;

mod display;
mod ops;
