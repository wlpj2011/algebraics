use crate::poly::Poly;
use crate::traits::{Finite, Zero};

pub struct PolyIter<T: Finite> {
    current: Vec<T>,
    elements: Vec<T>, // all elements of T, computed once
    indices: Vec<usize>,
    done: bool,
}

impl<T: Finite + Zero + Clone + PartialEq> PolyIter<T> {
    /// Iterates over all polynomials of degree less than or equal to n,
    /// i.e. with at most n+1 coefficients
    pub fn all_of_bounded_degree(n: usize) -> Self {
        let elements: Vec<T> = T::enumerate().collect();
        PolyIter {
            current: vec![T::zero(); n + 1],
            indices: vec![0; n + 1],
            elements,
            done: false,
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
