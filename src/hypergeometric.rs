use crate::input::{DeckSize, DesiredCount, DrawCount, SourceCount};

/// Probability of drawing **at least k** successes in a hypergeometric distribution.
///
/// # Formula
///
/// P(X ≥ k) = 1 − Σ_{i=0}^{k−1} P(X = i)
///
/// Uses the recurrence:
/// P(X = 0) = ∏_{i=0}^{n−1} (N − K − i) / (N − i)
/// P(X = m+1) = P(X = m) × (K − m)(n − m) / ((m+1)(N − K − n + m + 1))
///
/// # Examples
///
/// ```
/// use cardodds::input::{DeckSize, DrawCount, SourceCount, DesiredCount};
/// use cardodds::hypergeometric::probability_at_least_k;
///
/// let deck = DeckSize::new(60).unwrap();
/// let draw = DrawCount::new(7, &deck).unwrap();
/// let sources = SourceCount::new(4, &deck).unwrap();
/// let desired = DesiredCount::new(1, &sources).unwrap();
///
/// let prob = probability_at_least_k(&deck, &draw, &sources, &desired);
/// let rounded = (prob * 10000.0).round() / 10000.0;
/// assert_eq!(rounded, 0.3995);
/// ```
pub fn probability_at_least_k(
    deck: &DeckSize,
    draw: &DrawCount,
    sources: &SourceCount,
    desired: &DesiredCount,
) -> f64 {
    let deck_size = deck.get();
    let draw_count = draw.get();
    let source_count = sources.get();
    let desired_count = desired.get();

    if draw_count == 0 || source_count == 0 {
        return 0.0;
    }

    if desired_count > source_count {
        return 0.0;
    }

    let p0 = prob_x0(deck_size, draw_count, source_count);

    if desired_count == 1 {
        return 1.0 - p0;
    }

    let mut cumulative = p0;
    let mut prob = p0;

    for m in 0..(desired_count - 1) {
        prob = next_prob(prob, deck_size, draw_count, source_count, m);
        cumulative += prob;
    }

    1.0 - cumulative
}

/// P(X = 0) = ∏_{i=0}^{n−1} (N − K − i) / (N − i)
fn prob_x0(deck_size: u64, draw: u64, sources: u64) -> f64 {
    let mut result = 1.0;
    let non_sources = deck_size - sources;

    for i in 0..draw {
        if non_sources <= i {
            return 0.0;
        }
        let numerator = non_sources - i;
        let denominator = deck_size - i;
        result *= numerator as f64 / denominator as f64;
    }

    result
}

/// P(X = m+1) = P(X = m) × (K − m)(n − m) / ((m+1)(N − K − n + m + 1))
fn next_prob(prev: f64, deck_size: u64, draw: u64, sources: u64, m: u64) -> f64 {
    let m_f = m as f64;
    let numerator = (sources as f64 - m_f) * (draw as f64 - m_f);
    let denominator = (m_f + 1.0) * (deck_size as f64 - sources as f64 - draw as f64 + m_f + 1.0);
    prev * numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::{DeckSize, DesiredCount, DrawCount, SourceCount};

    fn make_inputs(
        deck: u64,
        draw: u64,
        sources: u64,
        desired: u64,
    ) -> (DeckSize, DrawCount, SourceCount, DesiredCount) {
        let d = DeckSize::new(deck).unwrap();
        let s = SourceCount::new(sources, &d).unwrap();
        (
            d,
            DrawCount::new(draw, &d).unwrap(),
            s,
            DesiredCount::new(desired, &s).unwrap(),
        )
    }

    #[test]
    fn readme_example_k1() {
        let (d, n, s, des) = make_inputs(60, 7, 4, 1);
        let prob = probability_at_least_k(&d, &n, &s, &des);
        let rounded = (prob * 10000.0).round() / 10000.0;
        assert_eq!(rounded, 0.3995);
    }

    #[test]
    fn k2_gives_smaller_probability() {
        let (d, n, s, des) = make_inputs(60, 7, 4, 2);
        let prob = probability_at_least_k(&d, &n, &s, &des);
        let rounded = (prob * 10000.0).round() / 10000.0;
        assert_eq!(rounded, 0.0632);
    }

    #[test]
    fn zero_sources_returns_zero() {
        let d = DeckSize::new(60).unwrap();
        let n = DrawCount::new(7, &d).unwrap();
        let s = SourceCount::new(0, &d).unwrap();
        let dummy = SourceCount::new(1, &DeckSize::new(60).unwrap()).unwrap();
        let des = DesiredCount::new(1, &dummy).unwrap();
        assert_eq!(probability_at_least_k(&d, &n, &s, &des), 0.0);
    }

    #[test]
    fn all_cards_are_sources_returns_one() {
        let (d, n, s, des) = make_inputs(60, 7, 60, 1);
        assert_eq!(probability_at_least_k(&d, &n, &s, &des), 1.0);
    }

    #[test]
    fn no_draws_returns_zero() {
        let (d, n, s, des) = make_inputs(60, 0, 4, 1);
        assert_eq!(probability_at_least_k(&d, &n, &s, &des), 0.0);
    }

    #[test]
    fn draw_entire_deck_with_one_source_returns_one() {
        let (d, n, s, des) = make_inputs(60, 60, 1, 1);
        assert_eq!(probability_at_least_k(&d, &n, &s, &des), 1.0);
    }

    #[test]
    fn draw_entire_deck_without_sources_returns_zero() {
        let d = DeckSize::new(60).unwrap();
        let n = DrawCount::new(60, &d).unwrap();
        let s = SourceCount::new(0, &d).unwrap();
        let dummy = SourceCount::new(1, &DeckSize::new(60).unwrap()).unwrap();
        let des = DesiredCount::new(1, &dummy).unwrap();
        assert_eq!(probability_at_least_k(&d, &n, &s, &des), 0.0);
    }

    #[test]
    fn small_numbers_k1() {
        let (d, n, s, des) = make_inputs(10, 3, 2, 1);
        let prob = probability_at_least_k(&d, &n, &s, &des);
        assert!((prob - 0.5333).abs() < 0.001);
    }

    #[test]
    fn k_equals_sources_returns_small_probability() {
        let (d, n, s, des) = make_inputs(60, 7, 4, 4);
        let prob = probability_at_least_k(&d, &n, &s, &des);
        assert!((prob - 0.00007).abs() < 0.01);
        assert!(prob > 0.0);
    }
}
