use crate::traits::{FiniteField, HasMultiplicativeGenerator, SeparableCharPFiniteExtension};
use super::additive::AdditiveCharacter;
use super::multiplicative::MultiplicativeCharacter;
use super::trace_to_u64::TraceToU64;

/// A Gauss sum g(χ, ψ) = Σ_{x ∈ F*} χ(x) · ψ(x).
///
/// Characters are stored symbolically; evaluation targets are deferred until
/// number/cyclotomic field types are available (see [`EvalInField`]).
#[derive(Clone, Debug)]
pub struct GaussSum<F: FiniteField + HasMultiplicativeGenerator> {
    pub chi: MultiplicativeCharacter<F>,
    pub psi: AdditiveCharacter<F>,
}

#[allow(private_bounds)]
impl<F> GaussSum<F>
where
    F: FiniteField + HasMultiplicativeGenerator + PartialEq + Clone,
    F: SeparableCharPFiniteExtension,
    F::BaseField: TraceToU64,
{
    pub fn new(chi: MultiplicativeCharacter<F>, psi: AdditiveCharacter<F>) -> Self {
        GaussSum { chi, psi }
    }

    /// The standard Gauss sum g(ω^j, ψ_trace) used in the Gross–Koblitz formula.
    pub fn standard(j: u64) -> Self {
        GaussSum {
            chi: MultiplicativeCharacter::with_index(j),
            psi: AdditiveCharacter::canonical(),
        }
    }

    /// Returns the symbolic phase table: one `(chi_exp, psi_exp)` pair per x ∈ F*.
    ///
    /// - `chi_exp` ∈ `0..q−1` is the exponent of ζ_{q-1} contributed by χ(x)
    /// - `psi_exp` ∈ `0..p`   is the exponent of ζ_p   contributed by ψ(x)
    ///
    /// The actual Gauss sum is Σ ζ_{q-1}^{chi_exp} · ζ_p^{psi_exp} in any ring
    /// containing the appropriate roots of unity.
    pub fn phase_table(&self) -> Vec<(u64, u64)>
    where
        F: std::ops::Mul<Output = F>,
    {
        F::multiplicative_group()
            .map(|x| {
                let chi_exp = self.chi.eval(&x).unwrap_or(0);
                let psi_exp = self.psi.eval(x);
                (chi_exp, psi_exp)
            })
            .collect()
    }

    /// Computes the Stickelberger data for g(ω^j, ψ_trace) where j = chi.index().
    ///
    /// Returns integer-only data (valuations, Galois exponents) that does not require
    /// number fields. See [`StickelbergerData`] for details.
    pub fn stickelberger_data(&self) -> StickelbergerData {
        let p = F::characteristic();
        let q = F::size() as u64;
        let j = self.chi.index();
        stickelberger_data_for(p, q, j)
    }
}

// ── Deferred evaluation trait ──────────────────────────────────────────────

/// Evaluates a Gauss sum in a target ring `T` containing appropriate roots of unity.
///
/// No implementations exist yet; this will be implemented once cyclotomic / number
/// field types are available.
pub trait EvalInField<T> {
    fn eval_in(&self) -> T;
}

// ── Stickelberger data ─────────────────────────────────────────────────────

/// Integer-valued data extracted from a Gauss sum g(ω^j, ψ_trace) that encodes
/// its factorization in the cyclotomic field Q(ζ_{q-1}) via Stickelberger's theorem.
///
/// None of this requires number fields: everything is computable from `p`, `q`, and `j`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StickelbergerData {
    /// The field characteristic p.
    pub p: u64,
    /// The field size q = p^f.
    pub q: u64,
    /// The character index j (so the Gauss sum is g(ω^j, ψ)).
    pub char_index: u64,

    /// Galois exponents: for each a ∈ {1,..,q-2} with gcd(a, q-1) = 1,
    /// the pair (a, ⌊j·a / (q−1)⌋).
    ///
    /// These are the exponents in the Stickelberger ideal factorization
    /// (g(ω^j, ψ)) = ∏_{a} p_a^{⌊j·a/(q-1)⌋} inside Q(ζ_{q-1}).
    pub galois_exponents: Vec<(u64, u64)>,

    /// The p-adic valuation v_p(g(ω^j, ψ)) = s_p(j) / (p−1) as (numerator, denominator).
    ///
    /// `s_p(j)` is the digit sum of j in base p.
    pub p_adic_valuation: (u64, u64),
}

/// Computes [`StickelbergerData`] for g(ω^j, ψ_trace) over F_q (char p).
pub fn stickelberger_data_for(p: u64, q: u64, j: u64) -> StickelbergerData {
    let group_order = q - 1;

    let galois_exponents = (1..group_order)
        .filter(|&a| gcd(a, group_order) == 1)
        .map(|a| (a, j * a / group_order))
        .collect();

    let digit_sum = digit_sum_base(j, p);
    let p_adic_valuation = (digit_sum, p - 1);

    StickelbergerData { p, q, char_index: j, galois_exponents, p_adic_valuation }
}

/// Returns the Stickelberger element θ = Σ_{a=1}^{q-2} ⟨a/(q−1)⟩ σ_a^{-1} ∈ Q[G].
///
/// Each entry is `(a, (numerator, denominator))` where `a/(q−1)` is already in
/// lowest terms.  The Galois group G = (Z/(q−1)Z)* acts on ζ_{q-1} by σ_a(ζ) = ζ^a.
pub fn stickelberger_element(q: u64) -> Vec<(u64, (u64, u64))> {
    let group_order = q - 1;
    (1..group_order)
        .filter(|&a| gcd(a, group_order) == 1)
        .map(|a| {
            let d = gcd(a, group_order);
            (a, (a / d, group_order / d))
        })
        .collect()
}

// ── Helpers ────────────────────────────────────────────────────────────────

fn digit_sum_base(mut n: u64, base: u64) -> u64 {
    if base <= 1 {
        return n;
    }
    let mut sum = 0;
    while n > 0 {
        sum += n % base;
        n /= base;
    }
    sum
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
