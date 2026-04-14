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

Requires a recent stable Rust toolchain (edition 2024). The build step generates a lookup table for Conway polynomials from `data/conway_polynomials.txt`; this happens automatically. To build:

```
cargo build
```

To run the test suite:

```
cargo test
```

## Usage

```rust
use algebraics::field::Fp;
use algebraics::field::FpN;
use algebraics::poly::Poly;
use algebraics::traits::{Finite, FiniteExtension, SeparableFiniteExtension};

// Elements of F_7
let a = Fp::<7>::new(3);
let b = Fp::<7>::new(5);
assert_eq!(a + b, Fp::<7>::new(1));  // 3 + 5 = 1 mod 7

// Polynomials over F_7
// p = 1 + 2x + 3x^2,  q = 4 + 5x
let p = Poly::new(vec![Fp::<7>::new(1), Fp::<7>::new(2), Fp::<7>::new(3)]);
let q = Poly::new(vec![Fp::<7>::new(4), Fp::<7>::new(5)]);
let product = &p * &q;
println!("({}) * ({}) = {}", p, q, product);

// GF(9) = F_3[x]/(x^2 + 2x + 2)
type GF9 = FpN<3, 2>;
let alpha = GF9::generator();  // a root of the Conway polynomial
println!("GF(9) has {} elements", GF9::size());
println!("Tr(alpha) = {}", alpha.trace());

// GF(13^6) — a field with over 4 million elements
type GF13_6 = FpN<13, 6>;
println!("degree = {}", <GF13_6 as FiniteExtension>::degree());
```

Instantiation with a composite modulus is a compile-time error:

```rust
let a = Fp::<4>::new(1);  // does not compile
```

## Current Status

The trait hierarchy runs from `Magma` through `Field`, with supporting traits `Zero`, `One`, `Finite`, `FiniteRing`, and `FiniteField`, plus a full suite of extension traits (`FieldExtension`, `FiniteExtension`, `SeparableExtension`, `CharPFiniteExtension`, and more).

**`Fp<P>`** is complete. Primality of `P` is checked at compile time, so instantiation with a composite modulus is a compile error. Multiplicative inversion uses Fermat's little theorem.

**`Poly<T: Ring>`** is complete, with addition, subtraction, multiplication, negation, and polynomial long division (`EuclideanDomain`). `PolyIter<T: FiniteRing>` allows iteration over all polynomials of given or bounded degree.

**`FiniteSimpleExtension<F, M>`** is complete. Given a field `F` and a zero-size marker type `M` implementing `IrreduciblePoly<F>`, this constructs the quotient field `F[x]/(M(x))`. Implements the full extension trait hierarchy including `FiniteExtension` (norm, projection), `SeparableFiniteExtension` (trace), and `CharPFiniteExtension` (Frobenius).

**`FpN<P, N>`** is complete. A type alias for `FiniteSimpleExtension<Fp<P>, ConwayPoly<P, N>>`, giving the finite field GF(p^n) with the Conway polynomial as modulus. Conway polynomials are sourced from Frank Lübeck's precomputed table, embedded at compile time via `build.rs`. The generator `FpN::<P, N>::generator()` is a primitive root of GF(p^n)^×.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.