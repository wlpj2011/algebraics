//! A library for algebraic computations over finite fields and their extensions,
//! developed in connection with research on the local converse theorem.
//!
//! # Structure
//! - [`traits`]: the algebraic hierarchy from [`traits::Magma`] to [`traits::Field`]
//! - [`field`]: finite field implementations ([`field::finite_field::Fp`])
//! - [`poly`]: polynomial arithmetic over generic rings ([`poly::Poly`])
//! - [`arithmetic`]: low-level modular arithmetic primitives
pub mod traits;

pub mod arithmetic;
pub mod field;
pub mod matrix_arithemetic;
pub mod poly;
