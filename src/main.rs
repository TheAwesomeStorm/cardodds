use anyhow::Result;
use cardodds::hypergeometric::probability_at_least_one;
use cardodds::input::{DeckSize, DrawCount, SourceCount};
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Calculate hypergeometric probabilities for card games")]
struct Args {
    #[arg(long, help = "Total cards in the deck")]
    deck: Option<u64>,

    #[arg(long, help = "Number of cards drawn")]
    draw: Option<u64>,

    #[arg(long, help = "Number of source cards in the deck")]
    sources: Option<u64>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let deck = DeckSize::new(args.deck.unwrap_or(60))?;
    let draw = DrawCount::new(args.draw.unwrap_or(7), &deck)?;
    let sources = SourceCount::new(args.sources.unwrap_or(4), &deck)?;

    let prob = probability_at_least_one(&deck, &draw, &sources);
    println!("{:.4}", prob);

    Ok(())
}
