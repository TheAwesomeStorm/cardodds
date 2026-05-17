use anyhow::Result;
use cardodds::domain::probability_at_least_k;
use cardodds::input::{DeckSize, DesiredCount, DrawCount, SourceCount};
use clap::Parser;
use dialoguer::Input;

#[derive(Parser)]
#[command(
    version,
    about = "Calculate hypergeometric probabilities for card games"
)]
struct Args {
    #[arg(short = 'N', long, help = "Total cards in the deck")]
    deck: Option<u64>,

    #[arg(short = 'n', long, help = "Number of cards drawn")]
    draw: Option<u64>,

    #[arg(short = 'K', long, help = "Number of source cards in the deck")]
    sources: Option<u64>,

    #[arg(short = 'k', long, help = "Minimum number of desired cards in hand")]
    desired: Option<u64>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let has_any_arg = args.deck.is_some()
        || args.draw.is_some()
        || args.sources.is_some()
        || args.desired.is_some();

    let (deck, draw, sources, desired) = if has_any_arg {
        let deck = DeckSize::new(args.deck.unwrap_or(60))?;
        let draw = DrawCount::new(args.draw.unwrap_or(7), &deck)?;
        let sources = SourceCount::new(args.sources.unwrap_or(4), &deck)?;
        let desired = DesiredCount::new(args.desired.unwrap_or(1), &sources)?;
        (deck, draw, sources, desired)
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

        let desired_val: u64 = Input::new()
            .with_prompt("Minimum desired cards in hand")
            .default(1)
            .interact_text()?;
        let desired = DesiredCount::new(desired_val, &sources)?;

        (deck, draw, sources, desired)
    };

    let prob = probability_at_least_k(&deck, &draw, &sources, &desired);
    let k = desired.get();
    let card_label = if k == 1 { "card" } else { "cards" };
    println!("Probability of having at least {k} {card_label} in hand: {prob:.4}");

    Ok(())
}
