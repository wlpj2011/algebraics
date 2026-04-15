use algebraics::arithmetic::pow;
use algebraics::traits::*;

pub fn exhaustive_elements<F: Finite>() -> Vec<F> {
    F::enumerate().collect()
}

pub fn take_elements<T: Finite>(limit: usize) -> Vec<T> {
    T::enumerate().take(limit).collect()
}

pub fn generator_powers<F: Field + HasMultiplicativeGenerator>(steps: usize) -> Vec<F> {
    let mut elems = Vec::with_capacity(steps);
    let mut x = F::one();
    let g = F::multiplicative_generator();

    for _ in 0..steps {
        elems.push(x.clone());
        x = x * g.clone();
    }

    elems
}

pub fn structured_samples<F: FiniteField + HasMultiplicativeGenerator>(steps: usize) -> Vec<F> {
    let g = F::multiplicative_generator();

    let mut elems = Vec::with_capacity(3 * steps);

    // Multiplicative walk
    let mut x = F::one();
    for _ in 0..steps {
        elems.push(x.clone());
        x = x * g.clone();
    }

    // Shifted multiplicative walk
    let mut y = F::one();
    let g1 = g.clone() + F::one();
    for _ in 0..steps {
        elems.push(y.clone());
        y = y * g1.clone();
    }

    // Additive mixing
    for i in 0..steps {
        elems.push(pow(g.clone(), i as u64) + F::one());
    }

    elems
}
