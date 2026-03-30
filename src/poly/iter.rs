//! Iterators over polynomials with finite coefficients.
//!
//! `PolyIter<T>` allows you to enumerate all polynomials with coefficients in
//! a finite type `T` (i.e., `T: Finite + Zero + Clone + PartialEq`). It supports
//! iterating over polynomials of either bounded degree (≤ n) or exact degree (== n).
//!
//! # Examples
//! ```
//! # use algebraics::poly::{Poly, PolyIter};
//! # use algebraics::finite_field::Fp;
//! # use algebraics::traits::{Zero, One};
//!
//! type F7 = Fp<7>;
//!
//! // Iterate over all polynomials of degree ≤ 2
//! let polys: Vec<Poly<F7>> = PolyIter::<F7>::all_of_bounded_degree(2).collect();
//! assert_eq!(polys.len(), 7usize.pow(3)); // 0..2^7 coefficients
//!
//! // Iterate over all polynomials of degree exactly 2
//! let polys_deg2: Vec<Poly<F7>> = PolyIter::<F7>::all_of_exact_degree(2).collect();
//! for p in polys_deg2 {
//!     assert_eq!(p.degree().unwrap(), 2);
//! }
//! ```
use crate::poly::Poly;
use crate::traits::{Finite, Zero};

/// Iterator over polynomials with coefficients in a finite type `T`.
///
/// This iterator can enumerate:
/// - all polynomials of degree **less than or equal** to `n`
/// - all polynomials of **exact degree** `n`
///
/// # Type requirements
/// - `T` must implement `Finite` to enumerate all elements.
/// - `T` must implement `Zero` and `Clone` to initialize polynomials.
/// - `T` must implement `PartialEq` to support equality checks.
///
/// # Examples
/// ```
/// # use algebraics::poly::{Poly, PolyIter};
/// # use algebraics::finite_field::Fp;
/// # use algebraics::traits::{Zero, One};
///
/// type F7 = Fp<7>;
/// // iterate over all polynomials of degree <= 2
/// let polys: Vec<Poly<F7>> = PolyIter::<F7>::all_of_bounded_degree(2).collect();
/// // iterate over all polynomials of degree exactly 2
/// let exact_deg: Vec<Poly<F7>> = PolyIter::<F7>::all_of_exact_degree(2).collect();
/// ```
pub struct PolyIter<T: Finite> {
    current: Vec<T>,
    elements: Vec<T>, // all elements of T, computed once
    indices: Vec<usize>,
    done: bool,
    remaining: usize,
}

impl<T: Finite + Zero + Clone + PartialEq> PolyIter<T> {
    /// Iterates over all polynomials of degree less than or equal to n,
    /// i.e. with at most n+1 coefficients
    pub fn all_of_bounded_degree(n: usize) -> Self {
        let elements: Vec<T> = T::enumerate().collect();
        let total = T::size().pow((n + 1) as u32);
        PolyIter {
            current: vec![T::zero(); n + 1],
            indices: vec![0; n + 1],
            elements,
            done: false,
            remaining: total,
        }
    }

    /// Iterates over all polynomials of degree equal to n,
    /// i.e. with exactly n+1 coefficients
    pub fn all_of_exact_degree(n: usize) -> impl Iterator<Item = Poly<T>> {
        PolyIter::all_of_bounded_degree(n).filter(move |p| p.degree() == Some(n))
    }
}

impl<T: Finite + Clone + Zero + PartialEq> Iterator for PolyIter<T> {
    type Item = Poly<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = Poly::new(self.current.clone());

        // Decrement Remaining
        self.remaining -= 1;
        if self.remaining == 0 {
            self.done = true;
        }

        // Increment indices
        let mut carry = true;
        for (idx, coeff) in self.indices.iter_mut().zip(self.current.iter_mut()) {
            if carry {
                *idx += 1;
                if *idx >= self.elements.len() {
                    *idx = 0;
                    *coeff = self.elements[0].clone();
                } else {
                    *coeff = self.elements[*idx].clone();
                    carry = false;
                    break;
                }
            }
        }
        if carry {
            self.done = true;
        }

        Some(result)
    }
}

impl<T: Finite + Clone + Zero + PartialEq> ExactSizeIterator for PolyIter<T> {
    fn len(&self) -> usize {
        self.remaining
    }
}
