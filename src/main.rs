use anyhow::Result;
use cardodds::hypergeometric::probability_at_least_one;
use cardodds::input::{DeckSize, DrawCount, SourceCount};

fn main() -> Result<()> {
    let deck = DeckSize::new(60)?;
    let draw = DrawCount::new(7, &deck)?;
    let sources = SourceCount::new(4, &deck)?;

    let prob = probability_at_least_one(&deck, &draw, &sources);
    println!("{:.4}", prob);

    Ok(())
}
