use anyhow::Result;
use cardodds::hypergeometric::probability_at_least_one;
use cardodds::input::{DeckSize, DrawCount, SourceCount};
use clap::Parser;
use dialoguer::Input;

#[derive(Parser)]
#[command(
    version,
    about = "Calculate hypergeometric probabilities for card games"
)]
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

    let has_any_arg = args.deck.is_some() || args.draw.is_some() || args.sources.is_some();

    let (deck, draw, sources) = if has_any_arg {
        let deck = DeckSize::new(args.deck.unwrap_or(60))?;
        let draw = DrawCount::new(args.draw.unwrap_or(7), &deck)?;
        let sources = SourceCount::new(args.sources.unwrap_or(4), &deck)?;
        (deck, draw, sources)
    } else {
        let deck_val: u64 = Input::new()
            .with_prompt("Total cards in deck")
            .default(60)
            .interact_text()?;
        let deck = DeckSize::new(deck_val)?;

        let draw_val: u64 = Input::new()
            .with_prompt("Cards drawn")
            .default(7)
            .interact_text()?;
        let draw = DrawCount::new(draw_val, &deck)?;

        let sources_val: u64 = Input::new()
            .with_prompt("Source cards in deck")
            .default(4)
            .interact_text()?;
        let sources = SourceCount::new(sources_val, &deck)?;

        (deck, draw, sources)
    };

    let prob = probability_at_least_one(&deck, &draw, &sources);
    println!("Probability: {:.4}", prob);

    Ok(())
}
