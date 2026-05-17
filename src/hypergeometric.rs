/// P(X = 0) = ∏_{i=0}^{draw−1} (pop − successes − i) / (pop − i)
fn prob_x0(population: u64, draw: u64, successes: u64) -> f64 {
    let mut result = 1.0;
    let non_successes = population - successes;

    for i in 0..draw {
        if non_successes <= i {
            return 0.0;
        }
        let numerator = non_successes - i;
        let denominator = population - i;
        result *= numerator as f64 / denominator as f64;
    }

    result
}

/// P(X = m+1) = P(X = m) × (successes − m)(draw − m) /
///              ((m+1)(pop − successes − draw + m + 1))
fn next_prob(prev: f64, population: u64, draw: u64, successes: u64, m: u64) -> f64 {
    let m_f = m as f64;
    let numerator = (successes as f64 - m_f) * (draw as f64 - m_f);
    let denominator =
        (m_f + 1.0) * (population as f64 - successes as f64 - draw as f64 + m_f + 1.0);
    prev * numerator / denominator
}

/// P(X = k) where X ~ Hypergeometric(population, successes, draws).
///
/// Preconditions: `successes ≤ population`, `draws ≤ population`.
/// Results are undefined if violated.
///
/// ```
/// use cardodds::hypergeometric::prob_exact;
/// let p = prob_exact(60, 7, 4, 1);
/// let rounded = (p * 10000.0).round() / 10000.0;
/// assert_eq!(rounded, 0.3363);
/// ```
pub fn prob_exact(population: u64, draws: u64, successes: u64, k: u64) -> f64 {
    if k > draws || k > successes {
        return 0.0;
    }

    let mut prob = prob_x0(population, draws, successes);
    for m in 0..k {
        prob = next_prob(prob, population, draws, successes, m);
    }
    prob
}

/// P(X ≥ k) where X ~ Hypergeometric(population, successes, draws).
///
/// Preconditions: `successes ≤ population`, `draws ≤ population`.
/// Results are undefined if violated.
///
/// ```
/// use cardodds::hypergeometric::prob_at_least_k;
/// let p = prob_at_least_k(60, 7, 4, 1);
/// let rounded = (p * 10000.0).round() / 10000.0;
/// assert_eq!(rounded, 0.3995);
/// ```
pub fn prob_at_least_k(population: u64, draws: u64, successes: u64, desired: u64) -> f64 {
    if draws == 0 || successes == 0 || desired > successes {
        return 0.0;
    }

    let p0 = prob_x0(population, draws, successes);

    if desired == 1 {
        return 1.0 - p0;
    }

    let mut cumulative = p0;
    let mut prob = p0;

    for m in 0..(desired - 1) {
        prob = next_prob(prob, population, draws, successes, m);
        cumulative += prob;
    }

    1.0 - cumulative
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn k1_matches_known_value() {
        let p = prob_at_least_k(60, 7, 4, 1);
        let rounded = (p * 10000.0).round() / 10000.0;
        assert_eq!(rounded, 0.3995);
    }

    #[test]
    fn k2_matches_known_value() {
        let p = prob_at_least_k(60, 7, 4, 2);
        let rounded = (p * 10000.0).round() / 10000.0;
        assert_eq!(rounded, 0.0632);
    }

    #[test]
    fn zero_successes_returns_zero() {
        assert_eq!(prob_at_least_k(60, 7, 0, 1), 0.0);
    }

    #[test]
    fn zero_draws_returns_zero() {
        assert_eq!(prob_at_least_k(60, 0, 4, 1), 0.0);
    }

    #[test]
    fn all_successes_returns_one() {
        assert_eq!(prob_at_least_k(60, 7, 60, 1), 1.0);
    }

    #[test]
    fn draw_entire_population_with_one_success_returns_one() {
        assert_eq!(prob_at_least_k(60, 60, 1, 1), 1.0);
    }

    #[test]
    fn draw_entire_population_without_successes_returns_zero() {
        assert_eq!(prob_at_least_k(60, 60, 0, 1), 0.0);
    }

    #[test]
    fn small_numbers_k1() {
        let p = prob_at_least_k(10, 3, 2, 1);
        assert!((p - 0.5333).abs() < 0.001);
    }

    #[test]
    fn k_equals_successes_returns_small_probability() {
        let p = prob_at_least_k(60, 7, 4, 4);
        assert!((p - 0.00007).abs() < 0.01);
        assert!(p > 0.0);
    }

    #[test]
    fn desired_exceeds_successes_returns_zero() {
        assert_eq!(prob_at_least_k(60, 7, 4, 5), 0.0);
    }

    #[test]
    fn prob_exact_k0() {
        let p = prob_exact(60, 7, 4, 0);
        let rounded = (p * 10000.0).round() / 10000.0;
        assert_eq!(rounded, 0.6005);
    }

    #[test]
    fn prob_exact_k1() {
        let p = prob_exact(60, 7, 4, 1);
        let rounded = (p * 10000.0).round() / 10000.0;
        assert_eq!(rounded, 0.3363);
    }

    #[test]
    fn prob_exact_exceeds_draws_returns_zero() {
        assert_eq!(prob_exact(60, 3, 4, 5), 0.0);
    }

    #[test]
    fn prob_exact_exceeds_successes_returns_zero() {
        assert_eq!(prob_exact(60, 7, 3, 5), 0.0);
    }
}
