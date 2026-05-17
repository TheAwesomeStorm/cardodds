/// Probability of drawing **at least one** success in a hypergeometric distribution.
///
/// # Formula
///
/// P(X ≥ 1) = 1 − ∏_{i=0}^{n−1} (N − K − i) / (N − i)
///
/// # Arguments
///
/// * `deck`  — total cards in the deck (N)
/// * `draw`  — number of cards drawn (n)
/// * `sources` — number of source cards in the deck (K)
///
/// # Panics
///
/// Panics if `deck` is 0, `draw > deck`, or `sources > deck`.
///
/// # Examples
///
/// ```
/// use cardodds::hypergeometric::probability_at_least_one;
///
/// let prob = probability_at_least_one(60, 7, 4);
/// let rounded = (prob * 10000.0).round() / 10000.0;
/// assert_eq!(rounded, 0.3995);
/// ```
pub fn probability_at_least_one(deck: u64, draw: u64, sources: u64) -> f64 {
    assert!(deck > 0, "deck must be non-empty");
    assert!(draw <= deck, "draw ({draw}) cannot exceed deck size ({deck})");
    assert!(
        sources <= deck,
        "sources ({sources}) cannot exceed deck size ({deck})"
    );

    if draw == 0 || sources == 0 {
        return 0.0;
    }

    let mut prob_no_sources = 1.0;
    let non_sources = deck - sources;

    for i in 0..draw {
        if non_sources <= i {
            prob_no_sources = 0.0;
            break;
        }
        let numerator = non_sources - i;
        let denominator = deck - i;
        prob_no_sources *= numerator as f64 / denominator as f64;
    }

    1.0 - prob_no_sources
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_example() {
        let prob = probability_at_least_one(60, 7, 4);
        let rounded = (prob * 10000.0).round() / 10000.0;
        assert_eq!(rounded, 0.3995);
    }

    #[test]
    fn zero_sources_returns_zero() {
        assert_eq!(probability_at_least_one(60, 7, 0), 0.0);
    }

    #[test]
    fn all_cards_are_sources_returns_one() {
        assert_eq!(probability_at_least_one(60, 7, 60), 1.0);
    }

    #[test]
    fn no_draws_returns_zero() {
        assert_eq!(probability_at_least_one(60, 0, 4), 0.0);
    }

    #[test]
    fn draw_entire_deck_with_sources_returns_one() {
        assert_eq!(probability_at_least_one(60, 60, 1), 1.0);
    }

    #[test]
    fn draw_entire_deck_without_sources_returns_zero() {
        assert_eq!(probability_at_least_one(60, 60, 0), 0.0);
    }

    #[test]
    fn small_numbers() {
        let prob = probability_at_least_one(10, 3, 2);
        assert!((prob - 0.5333).abs() < 0.001);
    }
}
