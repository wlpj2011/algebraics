//! Traits defining the identity elements

/// The additive identity element.
///
/// # Contract
/// - **Left identity**: `Zero::zero() + a == a` for all `a`
/// - **Right identity**: `a + Zero::zero() == a` for all `a`
/// - **Consistency**: `is_zero()` returns `true` if and only if `self == Zero`
pub trait Zero {
    /// Returns the additive identity element.
    fn zero() -> Self;

    /// Returns `true` if `self` is the additive identity.
    fn is_zero(&self) -> bool;
}

/// The multiplicative identity element.
///
/// # Contract
/// - **Left identity**: `One::one() * a == a` for all `a`
/// - **Right identity**: `a * One::one() == a` for all `a`
pub trait One {
    /// Returns the multiplicative identity element.
    fn one() -> Self;
}
