use anyhow::{bail, Result};
use std::fmt;

/// Total number of cards in the deck. Must be > 0.
#[derive(Debug, Clone, Copy)]
pub struct DeckSize(u64);

impl DeckSize {
    pub fn new(value: u64) -> Result<Self> {
        if value == 0 {
            bail!("deck must have at least 1 card");
        }
        Ok(Self(value))
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for DeckSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Number of cards drawn. Must not exceed the deck size.
#[derive(Debug, Clone, Copy)]
pub struct DrawCount(u64);

impl DrawCount {
    pub fn new(value: u64, deck: &DeckSize) -> Result<Self> {
        if value > deck.get() {
            bail!(
                "draw count ({value}) cannot exceed deck size ({})",
                deck.get()
            );
        }
        Ok(Self(value))
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for DrawCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Number of source cards in the deck. Must not exceed the deck size.
#[derive(Debug, Clone, Copy)]
pub struct SourceCount(u64);

impl SourceCount {
    pub fn new(value: u64, deck: &DeckSize) -> Result<Self> {
        if value > deck.get() {
            bail!(
                "source count ({value}) cannot exceed deck size ({})",
                deck.get()
            );
        }
        Ok(Self(value))
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for SourceCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Minimum number of desired cards in hand. Must be ≥ 1 and ≤ source count.
#[derive(Debug, Clone, Copy)]
pub struct DesiredCount(u64);

impl DesiredCount {
    pub fn new(value: u64, sources: &SourceCount) -> Result<Self> {
        if value == 0 {
            bail!("desired count must be at least 1");
        }
        if value > sources.get() {
            bail!(
                "desired count ({value}) cannot exceed source count ({})",
                sources.get()
            );
        }
        Ok(Self(value))
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for DesiredCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Total items in the pool. Must be ≥ source count and ≤ deck size.
#[derive(Debug, Clone, Copy)]
pub struct PoolSize(u64);

impl PoolSize {
    pub fn new(value: u64, deck: &DeckSize, sources: &SourceCount) -> Result<Self> {
        if value > deck.get() {
            bail!(
                "pool size ({value}) cannot exceed deck size ({})",
                deck.get()
            );
        }
        if value < sources.get() {
            bail!(
                "pool size ({value}) cannot be less than source count ({})",
                sources.get()
            );
        }
        Ok(Self(value))
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for PoolSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Minimum number of pool items drawn that we condition on. Must be ≤ pool size.
#[derive(Debug, Clone, Copy)]
pub struct PoolDrawn(u64);

impl PoolDrawn {
    pub fn new(value: u64, pool: &PoolSize) -> Result<Self> {
        if value > pool.get() {
            bail!(
                "pool drawn ({value}) cannot exceed pool size ({})",
                pool.get()
            );
        }
        Ok(Self(value))
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for PoolDrawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_deck_size() {
        let deck = DeckSize::new(60).unwrap();
        assert_eq!(deck.get(), 60);
    }

    #[test]
    fn zero_deck_size_rejected() {
        assert!(DeckSize::new(0).is_err());
    }

    #[test]
    fn draw_within_bounds_accepted() {
        let deck = DeckSize::new(60).unwrap();
        let draw = DrawCount::new(7, &deck).unwrap();
        assert_eq!(draw.get(), 7);
    }

    #[test]
    fn draw_exceeding_deck_rejected() {
        let deck = DeckSize::new(60).unwrap();
        assert!(DrawCount::new(61, &deck).is_err());
    }

    #[test]
    fn sources_within_bounds_accepted() {
        let deck = DeckSize::new(60).unwrap();
        let sources = SourceCount::new(4, &deck).unwrap();
        assert_eq!(sources.get(), 4);
    }

    #[test]
    fn sources_exceeding_deck_rejected() {
        let deck = DeckSize::new(60).unwrap();
        assert!(SourceCount::new(61, &deck).is_err());
    }

    #[test]
    fn valid_desired_count() {
        let deck = DeckSize::new(60).unwrap();
        let sources = SourceCount::new(4, &deck).unwrap();
        let target = DesiredCount::new(2, &sources).unwrap();
        assert_eq!(target.get(), 2);
    }

    #[test]
    fn desired_count_one_is_valid() {
        let deck = DeckSize::new(60).unwrap();
        let sources = SourceCount::new(4, &deck).unwrap();
        assert!(DesiredCount::new(1, &sources).is_ok());
    }

    #[test]
    fn desired_count_zero_rejected() {
        let deck = DeckSize::new(60).unwrap();
        let sources = SourceCount::new(4, &deck).unwrap();
        assert!(DesiredCount::new(0, &sources).is_err());
    }

    #[test]
    fn desired_count_exceeding_sources_rejected() {
        let deck = DeckSize::new(60).unwrap();
        let sources = SourceCount::new(4, &deck).unwrap();
        assert!(DesiredCount::new(5, &sources).is_err());
    }

    #[test]
    fn valid_pool_size() {
        let deck = DeckSize::new(60).unwrap();
        let sources = SourceCount::new(18, &deck).unwrap();
        let pool = PoolSize::new(24, &deck, &sources).unwrap();
        assert_eq!(pool.get(), 24);
    }

    #[test]
    fn pool_size_exceeding_deck_rejected() {
        let deck = DeckSize::new(60).unwrap();
        let sources = SourceCount::new(18, &deck).unwrap();
        assert!(PoolSize::new(61, &deck, &sources).is_err());
    }

    #[test]
    fn pool_size_less_than_sources_rejected() {
        let deck = DeckSize::new(60).unwrap();
        let sources = SourceCount::new(18, &deck).unwrap();
        assert!(PoolSize::new(10, &deck, &sources).is_err());
    }

    #[test]
    fn valid_pool_drawn() {
        let deck = DeckSize::new(60).unwrap();
        let sources = SourceCount::new(18, &deck).unwrap();
        let pool = PoolSize::new(24, &deck, &sources).unwrap();
        let drawn = PoolDrawn::new(3, &pool).unwrap();
        assert_eq!(drawn.get(), 3);
    }

    #[test]
    fn pool_drawn_exceeding_pool_rejected() {
        let deck = DeckSize::new(60).unwrap();
        let sources = SourceCount::new(18, &deck).unwrap();
        let pool = PoolSize::new(24, &deck, &sources).unwrap();
        assert!(PoolDrawn::new(25, &pool).is_err());
    }
}
