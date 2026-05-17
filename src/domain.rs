use crate::hypergeometric;
use crate::input::{DeckSize, DesiredCount, DrawCount, PoolDrawn, PoolSize, SourceCount};

/// Probability of drawing at least `desired` source cards, given a deck of
/// `deck` cards, drawing `draw` cards, with `sources` total copies in the deck.
///
/// ```
/// use cardodds::input::{DeckSize, DrawCount, SourceCount, DesiredCount};
/// use cardodds::domain::probability_at_least_k;
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
    hypergeometric::prob_at_least_k(deck.get(), draw.get(), sources.get(), desired.get())
}

/// Probability of drawing at least `desired` source cards, given a deck of
/// `deck` cards, drawing `draw` cards, with `sources` copies inside a pool of
/// `pool` items, conditioning on at least `pool_drawn` pool items being drawn.
pub fn conditional_probability(
    deck: &DeckSize,
    draw: &DrawCount,
    pool: &PoolSize,
    sources: &SourceCount,
    pool_drawn: &PoolDrawn,
    desired: &DesiredCount,
) -> f64 {
    hypergeometric::conditional_at_least_k(
        deck.get(),
        draw.get(),
        pool.get(),
        pool_drawn.get(),
        sources.get(),
        desired.get(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn make_conditional_inputs(
        deck: u64,
        draw: u64,
        sources: u64,
        desired: u64,
        pool: u64,
        pool_drawn: u64,
    ) -> (
        DeckSize,
        DrawCount,
        SourceCount,
        DesiredCount,
        PoolSize,
        PoolDrawn,
    ) {
        let d = DeckSize::new(deck).unwrap();
        let s = SourceCount::new(sources, &d).unwrap();
        let p = PoolSize::new(pool, &d, &s).unwrap();
        (
            d,
            DrawCount::new(draw, &d).unwrap(),
            s,
            DesiredCount::new(desired, &s).unwrap(),
            p,
            PoolDrawn::new(pool_drawn, &p).unwrap(),
        )
    }

    #[test]
    fn conditional_basic() {
        let (d, n, s, des, p, pd) = make_conditional_inputs(60, 9, 18, 2, 24, 3);
        let prob = conditional_probability(&d, &n, &p, &s, &pd, &des);
        assert!(prob > 0.0 && prob <= 1.0);
    }

    #[test]
    fn conditional_zero_draws_returns_zero() {
        let (d, n, s, des, p, pd) = make_conditional_inputs(60, 0, 18, 2, 24, 3);
        assert_eq!(conditional_probability(&d, &n, &p, &s, &pd, &des), 0.0);
    }
}
