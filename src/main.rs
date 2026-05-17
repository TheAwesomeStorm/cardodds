use anyhow::anyhow;
use anyhow::Result;
use cardodds::domain::{conditional_probability, probability_at_least_k};
use cardodds::input::{DeckSize, DesiredCount, DrawCount, PoolDrawn, PoolSize, SourceCount};
use clap::{Parser, Subcommand};
use dialoguer::Input;

#[derive(Parser)]
#[command(
    version,
    about = "Calculate hypergeometric probabilities for card games",
    args_conflicts_with_subcommands = true
)]
struct Cli {
    #[arg(short = 'N', long, help = "Total cards in the deck")]
    deck: Option<u64>,

    #[arg(short = 'n', long, help = "Number of cards drawn")]
    draw: Option<u64>,

    #[arg(short = 'K', long, help = "Number of source cards in the deck")]
    sources: Option<u64>,

    #[arg(short = 'k', long, help = "Minimum number of desired cards in hand")]
    desired: Option<u64>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// P(subgroup ≥ k | pool ≥ g) — conditional probability
    Conditional {
        #[arg(short = 'N', long, help = "Total cards in the deck")]
        deck: Option<u64>,

        #[arg(short = 'n', long, help = "Number of cards drawn")]
        draw: Option<u64>,

        #[arg(short = 'K', long, help = "Number of source cards in the deck")]
        sources: Option<u64>,

        #[arg(short = 'k', long, help = "Minimum number of desired cards in hand")]
        desired: Option<u64>,

        #[arg(short = 'G', long, help = "Total cards in the pool")]
        pool: Option<u64>,

        #[arg(short = 'g', long, help = "Minimum number of pool cards drawn")]
        pool_drawn: Option<u64>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Conditional {
            deck,
            draw,
            sources,
            desired,
            pool,
            pool_drawn,
        }) => {
            let has_any_arg = deck.is_some()
                || draw.is_some()
                || sources.is_some()
                || desired.is_some()
                || pool.is_some()
                || pool_drawn.is_some();

            if has_any_arg {
                let deck = DeckSize::new(deck.unwrap_or(60))?;
                let draw = DrawCount::new(draw.unwrap_or(7), &deck)?;
                let sources = SourceCount::new(
                    sources.ok_or_else(|| anyhow!("--sources/-K is required in CLI mode"))?,
                    &deck,
                )?;
                let pool = PoolSize::new(
                    pool.ok_or_else(|| anyhow!("--pool/-G is required in CLI mode"))?,
                    &deck,
                    &sources,
                )?;
                let pool_drawn = PoolDrawn::new(
                    pool_drawn.ok_or_else(|| anyhow!("--pool-drawn/-g is required in CLI mode"))?,
                    &pool,
                )?;
                let desired = DesiredCount::new(desired.unwrap_or(1), &sources)?;

                let prob =
                    conditional_probability(&deck, &draw, &pool, &sources, &pool_drawn, &desired);
                print_conditional_result(&prob, &desired, &pool_drawn);
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

                let pool_val: u64 = Input::new()
                    .with_prompt("Total cards in the pool")
                    .default(24)
                    .interact_text()?;
                let pool = PoolSize::new(pool_val, &deck, &sources)?;

                let pool_drawn_val: u64 = Input::new()
                    .with_prompt("Minimum pool cards drawn")
                    .default(3)
                    .interact_text()?;
                let pool_drawn = PoolDrawn::new(pool_drawn_val, &pool)?;

                let prob =
                    conditional_probability(&deck, &draw, &pool, &sources, &pool_drawn, &desired);
                print_conditional_result(&prob, &desired, &pool_drawn);
            }
        }
        None => {
            let has_any_arg = cli.deck.is_some()
                || cli.draw.is_some()
                || cli.sources.is_some()
                || cli.desired.is_some();

            let (deck, draw, sources, desired) = if has_any_arg {
                let deck = DeckSize::new(cli.deck.unwrap_or(60))?;
                let draw = DrawCount::new(cli.draw.unwrap_or(7), &deck)?;
                let sources = SourceCount::new(cli.sources.unwrap_or(4), &deck)?;
                let desired = DesiredCount::new(cli.desired.unwrap_or(1), &sources)?;
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
        }
    }

    Ok(())
}

fn print_conditional_result(prob: &f64, desired: &DesiredCount, pool_drawn: &PoolDrawn) {
    let k = desired.get();
    let g = pool_drawn.get();
    let source_label = if k == 1 { "source" } else { "sources" };
    let pool_label = if g == 1 { "pool card" } else { "pool cards" };
    println!(
        "Probability of having at least {k} {source_label} in hand\n  given at least {g} {pool_label} drawn: {prob:.4}"
    );
}
