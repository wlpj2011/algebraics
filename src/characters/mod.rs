//! Additive and multiplicative characters of finite fields, and Gauss sums.
//!
//! # Characters
//!
//! - [`AdditiveCharacter<F>`]: ψ_a(x) = Tr_{F/Fp}(a·x) mod p
//! - [`MultiplicativeCharacter<F>`]: χ_k(x) = discrete-log exponent k·log_g(x) mod (q−1)
//!
//! # Gauss sums
//!
//! - [`GaussSum<F>`]: g(χ, ψ) = Σ_{x ∈ F*} χ(x)·ψ(x), stored symbolically
//! - [`EvalInField<T>`]: deferred trait for evaluating into a ring with roots of unity
//!
//! # Stickelberger data
//!
//! - [`StickelbergerData`]: p-adic valuation and Galois factorization exponents
//! - [`stickelberger_data_for`]: compute data for g(ω^j, ψ_trace) from integers alone
//! - [`stickelberger_element`]: the full element θ ∈ Q[G] as rational coefficients

pub mod additive;
pub mod gauss_sum;
pub mod multiplicative;
pub(crate) mod trace_to_u64;

pub use additive::AdditiveCharacter;
pub use gauss_sum::{stickelberger_data_for, stickelberger_element, EvalInField, GaussSum, StickelbergerData};
pub use multiplicative::MultiplicativeCharacter;
