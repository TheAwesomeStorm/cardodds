use crate::input::{DeckSize, DrawCount, SourceCount};

/// Probability of drawing **at least one** success in a hypergeometric distribution.
///
/// # Formula
///
/// P(X ≥ 1) = 1 − ∏_{i=0}^{n−1} (N − K − i) / (N − i)
///
/// # Examples
///
/// ```
/// use cardodds::input::{DeckSize, DrawCount, SourceCount};
/// use cardodds::hypergeometric::probability_at_least_one;
///
/// let deck = DeckSize::new(60).unwrap();
/// let draw = DrawCount::new(7, &deck).unwrap();
/// let sources = SourceCount::new(4, &deck).unwrap();
///
/// let prob = probability_at_least_one(&deck, &draw, &sources);
/// let rounded = (prob * 10000.0).round() / 10000.0;
/// assert_eq!(rounded, 0.3995);
/// ```
pub fn probability_at_least_one(deck: &DeckSize, draw: &DrawCount, sources: &SourceCount) -> f64 {
    let d = deck.get();
    let n = draw.get();
    let k = sources.get();

    if n == 0 || k == 0 {
        return 0.0;
    }

    let mut prob_no_sources = 1.0;
    let non_sources = d - k;

    for i in 0..n {
        if non_sources <= i {
            prob_no_sources = 0.0;
            break;
        }
        let numerator = non_sources - i;
        let denominator = d - i;
        prob_no_sources *= numerator as f64 / denominator as f64;
    }

    1.0 - prob_no_sources
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::{DeckSize, DrawCount, SourceCount};

    fn make_inputs(deck: u64, draw: u64, sources: u64) -> (DeckSize, DrawCount, SourceCount) {
        let d = DeckSize::new(deck).unwrap();
        (
            d,
            DrawCount::new(draw, &d).unwrap(),
            SourceCount::new(sources, &d).unwrap(),
        )
    }

    #[test]
    fn readme_example() {
        let (d, n, k) = make_inputs(60, 7, 4);
        let prob = probability_at_least_one(&d, &n, &k);
        let rounded = (prob * 10000.0).round() / 10000.0;
        assert_eq!(rounded, 0.3995);
    }

    #[test]
    fn zero_sources_returns_zero() {
        let (d, n, k) = make_inputs(60, 7, 0);
        assert_eq!(probability_at_least_one(&d, &n, &k), 0.0);
    }

    #[test]
    fn all_cards_are_sources_returns_one() {
        let (d, n, k) = make_inputs(60, 7, 60);
        assert_eq!(probability_at_least_one(&d, &n, &k), 1.0);
    }

    #[test]
    fn no_draws_returns_zero() {
        let (d, n, k) = make_inputs(60, 0, 4);
        assert_eq!(probability_at_least_one(&d, &n, &k), 0.0);
    }

    #[test]
    fn draw_entire_deck_with_sources_returns_one() {
        let (d, n, k) = make_inputs(60, 60, 1);
        assert_eq!(probability_at_least_one(&d, &n, &k), 1.0);
    }

    #[test]
    fn draw_entire_deck_without_sources_returns_zero() {
        let (d, n, k) = make_inputs(60, 60, 0);
        assert_eq!(probability_at_least_one(&d, &n, &k), 0.0);
    }

    #[test]
    fn small_numbers() {
        let (d, n, k) = make_inputs(10, 3, 2);
        let prob = probability_at_least_one(&d, &n, &k);
        assert!((prob - 0.5333).abs() < 0.001);
    }
}
