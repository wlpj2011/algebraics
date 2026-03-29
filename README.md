# algebraics

A personal algebraic computation library written in Rust. The goal is not a general-purpose system on the scale of Sage, but something an individual can build, understand in full, and use as a substrate for research-level computation in algebraic number theory.

## Goals

The intended scope is as follows:

- Algebraic structures — groups, rings, fields, modules, vector spaces — defined as a trait hierarchy, with implementations for the canonical examples of interest
- Polynomial rings over arbitrary coefficient rings, and quotient rings thereof
- Finite fields, with an emphasis on the case $\mathbb{F}_{p^n}$ realized as $\mathbb{F}_p[x]/\langle f \rangle$ for an irreducible $f$ of degree $n$
- Multiplicative and additive characters on finite fields, with a view toward Gauss sum computation
- Eventually: rings of integers of number fields, and $p$-adic fields

The immediate research motivation is computation related to Gauss sums, Frobenius orbits, and the Gross–Koblitz formula.

## Building and Testing

Requires a recent stable Rust toolchain (edition 2024). To build:

```
cargo build
```

To run the test suite:

```
cargo test
```

## Usage

```rust
use algebraics::prime_field::Fp;
use algebraics::poly::Poly;

// Elements of F_7
let a = Fp::<7>::new(3);
let b = Fp::<7>::new(5);
assert_eq!(a + b, Fp::<7>::new(1));  // 3 + 5 = 1 mod 7

// Polynomials over F_7
// p = 1 + 2x + 3x^2,  q = 4 + 5x
// The reference &p, &q prevents p,q from being 'consumed' by taking their product
let p = Poly::new(vec![Fp::<7>::new(1), Fp::<7>::new(2), Fp::<7>::new(3)]);
let q = Poly::new(vec![Fp::<7>::new(4), Fp::<7>::new(5)]);
let product = &p * &q;
println!("({}) * ({}) = {}", p, q, product)
```

Instantiation with a composite modulus is a compile-time error:

```rust
let a = Fp::<4>::new(1);  // does not compile
```

## Current Status

The trait hierarchy is in place, running from `Magma` through `Field`, with supporting traits `Zero`, `One`, `Finite`, `FiniteRing`, and `FiniteField`.

The prime field $\mathbb{F}_p$, implemented as `Fp<const P: u64>`, is complete. Primality of `P` is checked at compile time via a `const` assertion, so instantiation with a composite modulus is a compile error. Multiplicative inversion uses Fermat's little theorem. The full test suite covers additive and multiplicative axioms exhaustively over small fields, and Fermat's little theorem over a larger prime.

The polynomial ring `Poly<T: Ring>` is implemented, with addition, subtraction, multiplication, and negation. Normalization (removal of leading zero coefficients) is handled on construction. Tests cover the ring axioms and degree behavior over $\mathbb{F}_7$.

See the issue tracker for known bugs and planned features.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.