//! A library for algebraic computations over finite fields and their extensions,
//! developed in connection with research on the local converse theorem.
//!
//! # Structure
//! - [`traits`]: the algebraic hierarchy from [`traits::Magma`] to [`traits::Field`]
//! - [`finite_field`]: concrete finite field implementations ([`finite_field::Fp`])
//! - [`poly`]: polynomial arithmetic over generic rings ([`poly::Poly`])
//! - [`arithmetic`]: low-level modular arithmetic primitives
pub mod traits;

pub mod arithmetic;
pub mod finite_field;
pub mod poly;
