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

    if k == 0 {
        return prob_x0(population, draws, successes);
    }

    // Not enough non-successes in the population to support (draws - k) of them.
    if draws - k > population - successes {
        return 0.0;
    }

    // Compute P(X = k) = C(K, k) × C(N-K, n-k) / C(N, n)
    // using a forward product to avoid dividing by zero when P(X=0)=0.
    let mut prob = 1.0;
    for i in 0..k {
        prob *= (successes - i) as f64 / (i + 1) as f64;
    }
    for i in 0..(draws - k) {
        prob *= (population - successes - i) as f64 / (i + 1) as f64;
    }
    for i in 0..draws {
        prob /= (population - i) as f64;
        prob *= (i + 1) as f64;
    }
    prob
}

/// P(subgroup ≥ k | pool ≥ g) — conditional hypergeometric.
///
/// Given N items, G pool items, K subgroup items within the pool,
/// draw n and condition on at least g pool items being drawn.
///
/// # Formula
///
/// P(X ≥ k | Y ≥ g) = Σ P(Y=y) × P(X ≥ k | Y=y) / Σ P(Y=y)
///                     y≥g                      y≥g
///
/// Where Y ~ Hypergeometric(N, n, G).
/// (X | Y=y) ~ Hypergeometric(G, y, K).
///
/// Preconditions: `K ≤ G ≤ N`, `g ≤ G`, `draws ≤ N`, `desired ≤ K`.
/// Results are undefined if violated.
///
/// ```
/// use cardodds::hypergeometric::conditional_at_least_k;
/// let p = conditional_at_least_k(60, 9, 24, 3, 18, 2);
/// assert!(p > 0.0 && p <= 1.0);
/// ```
pub fn conditional_at_least_k(
    population: u64,
    draws: u64,
    pool_size: u64,
    pool_drawn: u64,
    successes: u64,
    desired: u64,
) -> f64 {
    if draws == 0 || pool_size == 0 || successes == 0 || desired > successes {
        return 0.0;
    }
    if desired == 0 {
        return 1.0;
    }
    if pool_drawn > draws {
        return 0.0;
    }

    let max_y = std::cmp::min(draws, pool_size);

    let mut numerator = 0.0;
    let mut denominator = 0.0;

    for y in pool_drawn..=max_y {
        let p_y = prob_exact(population, draws, pool_size, y);
        denominator += p_y;

        if y >= desired {
            let p_x_given_y = prob_at_least_k(pool_size, y, successes, desired);
            numerator += p_y * p_x_given_y;
        }
    }

    if denominator == 0.0 {
        return 0.0;
    }

    numerator / denominator
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
    if desired == 0 {
        return 1.0;
    }

    let p0 = prob_x0(population, draws, successes);

    if p0 == 0.0 {
        // Guaranteed at least 1 success (fewer non-successes than draws).
        // The recurrence cannot climb from P(X=0)=0, so start from P(X=1).
        if desired == 1 {
            return 1.0;
        }
        let mut prob = prob_exact(population, draws, successes, 1);
        let mut cumulative = prob;
        for m in 1..(desired - 1) {
            prob = next_prob(prob, population, draws, successes, m);
            cumulative += prob;
        }
        return 1.0 - cumulative;
    }

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

    #[test]
    fn conditional_zero_draws_returns_zero() {
        assert_eq!(conditional_at_least_k(60, 0, 24, 3, 18, 2), 0.0);
    }

    #[test]
    fn conditional_pool_drawn_exceeds_draws_returns_zero() {
        assert_eq!(conditional_at_least_k(60, 3, 24, 5, 18, 2), 0.0);
    }

    #[test]
    fn conditional_desired_exceeds_successes_returns_zero() {
        assert_eq!(conditional_at_least_k(60, 9, 24, 3, 18, 20), 0.0);
    }

    #[test]
    fn conditional_small_manual() {
        // N=6, n=3, G=3, g=2, K=2, k=1
        // Deck: 3 pool (2 subgroup, 1 other pool), 3 non-pool
        // Condition: at least 2 pool drawn
        // P(Y=2) = C(3,2)*C(3,1)/C(6,3) = 3*3/20 = 9/20
        // P(Y=3) = C(3,3)*C(3,0)/C(6,3) = 1*1/20 = 1/20
        // P(Y≥2) = 10/20 = 0.5
        // P(X≥1|Y=2) = 1 - C(2,0)*C(1,2)/C(3,2) = 1 - 0 = 1.0 (must draw 2 subgroup if 2 pool)
        // Wait, with G=3, K=2, y=2: P(X≥1) = 1 - P(X=0) = 1 - C(2,0)*C(1,2)/C(3,2) = 1 - 0 = 1.0
        // P(X≥1|Y=3) = 1 - C(2,0)*C(1,3)/C(3,3) = 1 - 0 = 1.0
        // numerator = 9/20 * 1.0 + 1/20 * 1.0 = 10/20 = 0.5
        // denominator = 10/20 = 0.5
        // result = 0.5/0.5 = 1.0
        let p = conditional_at_least_k(6, 3, 3, 2, 2, 1);
        assert!((p - 1.0).abs() < 0.001);
    }

    #[test]
    fn conditional_small_manual_2() {
        // N=6, n=2, G=4, g=1, K=2, k=1
        // Pool: 4 items (2 subgroup), non-pool: 2 items
        // P(Y=1) = C(4,1)*C(2,1)/C(6,2) = 4*2/15 = 8/15
        // P(Y=2) = C(4,2)*C(2,0)/C(6,2) = 6*1/15 = 6/15
        // P(Y≥1) = 14/15
        // P(X≥1|Y=1) = 1 - C(2,0)*C(2,1)/C(4,1) = 1 - 2/4 = 0.5
        // P(X≥1|Y=2) = 1 - C(2,0)*C(2,2)/C(4,2) = 1 - 1/6 = 5/6
        // numerator = 8/15 * 0.5 + 6/15 * 5/6 = 4/15 + 5/15 = 9/15 = 0.6
        // denominator = 14/15
        // result = (9/15) / (14/15) = 9/14 ≈ 0.6429
        let p = conditional_at_least_k(6, 2, 4, 1, 2, 1);
        let expected = 9.0 / 14.0;
        assert!((p - expected).abs() < 0.001);
    }
}
