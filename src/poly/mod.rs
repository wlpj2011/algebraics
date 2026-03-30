//! Polynomial arithmetic over generic rings and finite fields.
//!
//! This module provides:
//!
//! - **`Poly<T>`** – A polynomial type with coefficients in `T`.
//!   - Supports addition, subtraction, multiplication, negation.
//!   - Automatically removes trailing zeros.
//!   - Implements `Display` for pretty-printing.
//! - **`PolyIter<T>`** – An iterator over all polynomials of bounded or exact degree
//!   for finite coefficient types.
//! - Traits from `crate::traits` can be used to constrain coefficients (`Ring`, `Field`, `Finite`, etc.).
//!
//! # Examples
//! ```
//! use algebraics::poly::Poly;
//! use algebraics::poly::PolyIter;
//! use algebraics::prime_field::Fp;
//! use algebraics::traits::{Zero, One};
//!
//! type F7 = Fp<7>;
//!
//! // Construct a polynomial x^2 + 1 in F7
//! let p = Poly::<F7>::new(vec![F7::one(), F7::zero(), F7::one()]);
//! assert_eq!(format!("{}", p), "x^2 + 1");
//!
//! // Iterate over all degree ≤ 2 polynomials
//! let polys: Vec<_> = PolyIter::<F7>::all_of_bounded_degree(2).collect();
//! assert_eq!(polys.len(), 7usize.pow(3));
//! ```
pub mod core;
pub mod iter;

mod display;
mod ops;

pub use core::Poly;
pub use iter::PolyIter;
