use algebraics::field::{Fp, FpN};
use algebraics::poly::Poly;
use algebraics::traits::*;

#[test]
fn test_fp_characteristic() {
    assert_eq!(Fp::<2>::characteristic(), 2);
    assert_eq!(Fp::<7>::characteristic(), 7);
    assert_eq!(Fp::<13>::characteristic(), 13);
}

#[test]
fn test_poly_characteristic_propagates() {
    // Poly<F> inherits characteristic from its coefficient ring.
    assert_eq!(<Poly<Fp<2>> as Ring>::characteristic(), 2);
    assert_eq!(<Poly<Fp<7>> as Ring>::characteristic(), 7);
    assert_eq!(<Poly<Fp<13>> as Ring>::characteristic(), 13);
}

#[test]
fn test_fpn_characteristic() {
    // FpN extensions inherit characteristic from the base prime field.
    assert_eq!(FpN::<2, 2>::characteristic(), 2); // GF(4)
    assert_eq!(FpN::<2, 3>::characteristic(), 2); // GF(8)
    assert_eq!(FpN::<3, 2>::characteristic(), 3); // GF(9)
    assert_eq!(FpN::<13, 6>::characteristic(), 13); // GF(13^6)
}
