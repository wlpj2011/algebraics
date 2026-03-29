use super::core::Poly;
use crate::traits::*;
use std::cmp::max;
use std::ops::{Add, Mul, Neg, Sub};

impl<T> Add for &Poly<T>
where
    T: Clone + Zero + Add<Output = T> + PartialEq,
{
    type Output = Poly<T>;

    fn add(self, rhs: Self) -> Poly<T> {
        let max_degree = max(self.degree(), rhs.degree());
        match max_degree {
            Some(max_degree) => {
                let mut result = Vec::with_capacity(max_degree + 1);
                for i in 0..=max_degree {
                    let a = self.coeff(i);
                    let b = rhs.coeff(i);
                    result.push(a + b);
                }
                Poly::new(result)
            }
            None => Poly::zero(),
        }
    }
}

impl<T> Add for Poly<T>
where
    T: Clone + Zero + Add<Output = T> + PartialEq,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        &self + &rhs
    }
}

impl<T> Neg for &Poly<T>
where
    T: Clone + Zero + Neg<Output = T> + PartialEq,
{
    type Output = Poly<T>;
    fn neg(self) -> Self::Output {
        let degree = self.degree();
        match degree {
            Some(degree) => {
                let mut result = Vec::with_capacity(degree + 1);
                for i in 0..=degree {
                    let a = self.coeff(i);
                    result.push(-a);
                }
                Poly::new(result)
            }
            None => Poly::zero(),
        }
    }
}

impl<T> Neg for Poly<T>
where
    T: Clone + Zero + Neg<Output = T> + PartialEq,
{
    type Output = Self;
    fn neg(self) -> Self {
        -&self
    }
}

impl<T> Sub for &Poly<T>
where
    T: Clone + Zero + Add<Output = T> + Neg<Output = T> + PartialEq,
{
    type Output = Poly<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        self + &(-rhs)
    }
}

impl<T> Sub for Poly<T>
where
    T: Clone + Zero + Add<Output = T> + Neg<Output = T> + PartialEq,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        &self + &(-&rhs)
    }
}

impl<T> Mul for &Poly<T>
where
    T: Clone + Zero + Add<Output = T> + Mul<Output = T> + PartialEq,
{
    type Output = Poly<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self.degree(), rhs.degree()) {
            (None, _) | (_, None) => Poly::zero(),
            (Some(d1), Some(d2)) => {
                let result_degree = d1 + d2;
                let mut result = Vec::with_capacity(result_degree + 1);
                for k in 0..=result_degree {
                    let mut kth_coeff = T::zero();
                    for i in 0..=k {
                        let j = k - i;
                        let a = self.coeff(i);
                        let b = rhs.coeff(j);
                        kth_coeff = kth_coeff + a * b;
                    }
                    result.push(kth_coeff);
                }
                Poly::new(result)
            }
        }
    }
}

impl<T> Mul for Poly<T>
where
    T: Clone + Zero + Add<Output = T> + Mul<Output = T> + PartialEq,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}
