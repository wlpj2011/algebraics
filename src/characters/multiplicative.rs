use std::marker::PhantomData;
use crate::traits::{Finite, FiniteField, HasMultiplicativeGenerator};

/// A multiplicative character χ_k: F* → Z/(q−1)Z.
///
/// The generator `g = F::multiplicative_generator()` fixes the parameterization:
/// `χ_k(g^j)` encodes the root-of-unity exponent `k·j mod (q−1)`.
///
/// - `k = 0` → trivial character (every nonzero element maps to exponent 0, i.e. value 1)
/// - `k = 1` → Teichmüller character ω (canonical character of order q−1)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultiplicativeCharacter<F: Finite> {
    index: u64,
    _f: PhantomData<F>,
}

impl<F: Finite> MultiplicativeCharacter<F> {
    /// The trivial multiplicative character (index 0).
    pub fn trivial() -> Self {
        MultiplicativeCharacter { index: 0, _f: PhantomData }
    }

    /// The Teichmüller character ω (index 1, order q−1).
    pub fn teichmuller() -> Self {
        MultiplicativeCharacter { index: 1, _f: PhantomData }
    }

    /// Constructs the character with the given index, reduced mod (q−1).
    pub fn with_index(k: u64) -> Self {
        let order = (F::size() - 1) as u64;
        MultiplicativeCharacter { index: k % order, _f: PhantomData }
    }

    pub fn index(&self) -> u64 {
        self.index
    }

    /// The order of this character: (q−1) / gcd(index, q−1).
    pub fn order(&self) -> u64 {
        let group_order = (F::size() - 1) as u64;
        if self.index == 0 {
            return 1;
        }
        group_order / gcd(self.index, group_order)
    }

    pub fn is_trivial(&self) -> bool {
        self.index == 0
    }
}

impl<F> MultiplicativeCharacter<F>
where
    F: FiniteField + HasMultiplicativeGenerator + PartialEq + Clone,
{
    /// Evaluates χ_k(x) as a root-of-unity exponent in `0..q−1`.
    ///
    /// Returns `None` for x = 0 (zero is not in the multiplicative group).
    /// Returns `Some(k · dlog_g(x) mod (q−1))`.
    pub fn eval(&self, x: &F) -> Option<u64> {
        if x.is_zero() {
            return None;
        }
        let dlog = discrete_log(x)?;
        let group_order = (F::size() - 1) as u64;
        Some((self.index as u128 * dlog as u128 % group_order as u128) as u64)
    }
}

/// Computes `dlog_g(x)` — the unique `k ∈ 0..q−1` with `g^k = x` — by exhaustive search.
pub(super) fn discrete_log<F>(x: &F) -> Option<u64>
where
    F: FiniteField + HasMultiplicativeGenerator + PartialEq + Clone,
{
    if x.is_zero() {
        return None;
    }
    let g = F::multiplicative_generator();
    let order = (F::size() - 1) as u64;
    let mut cur = F::one();
    for k in 0..order {
        if cur == *x {
            return Some(k);
        }
        cur = cur * g.clone();
    }
    None
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
