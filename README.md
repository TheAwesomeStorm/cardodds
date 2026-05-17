# cardodds

**cardodds** is a CLI tool that calculates the probability of drawing **at least one** specific card (a "source") from a deck after drawing a given number of cards — also known as a **hypergeometric probability**.

This is commonly used in trading card games (TCGs) like *Magic: The Gathering*, *Yu-Gi-Oh!*, *Pokémon TCG*, and *Hearthstone* to determine the likelihood of having a key card in your opening hand.

---

## The Math

The probability of drawing at least 1 source card is calculated using the **hypergeometric distribution**:

P(at least 1) = 1 − ∏_{i=0}^{n−1} (N − K − i) / (N − i)

Where:

- **N** — total number of cards in the deck
- **n** — number of cards drawn
- **K** — number of source cards in the deck

This is equivalent to: **1 − P(X = 0)**, where X is the number of source cards drawn.

### Worked Example

**Input:** N = 60, n = 7, K = 4

P(at least 1) = 1 − (56/60 × 55/59 × 54/58 × 53/57 × 52/56 × 51/55 × 50/54)

Result: **≈ 0.3995** (≈ 39.95%)

---

## Inputs

| Variable | Description | Example |
|----------|-------------|---------|
| N        | Total cards in deck | 60 |
| n        | Number of cards drawn | 7 |
| K        | Number of source cards in deck | 4 |

## Output

A single decimal value between 0 and 1 representing the probability.

```
0.3995
```

---

## Building on Windows

This project is designed to be built and run on **Windows** (the executable is intended for Windows use).

### Prerequisites

Install Rust from [rustup.rs](https://rustup.rs/) if you haven't already. This gives you `cargo` in your terminal (PowerShell, CMD, etc.).

### Build

```powershell
cd C:\Users\mendes\Projects\rust\cardodds
cargo build --release
```

The compiled executable will be at:

```
target\release\cardodds.exe
```

You can run it directly:

```powershell
.\target\release\cardodds.exe
```

Or add the `target\release` directory to your PATH for quick access.

---

## Pre-commit Hooks

This repository includes a pre-commit hook that runs `cargo fmt --check` and `cargo clippy` automatically before each commit.

### Install

From PowerShell, CMD, or WSL, run:

```powershell
git config core.hooksPath .githooks
```

This tells Git to use the hooks inside the `.githooks/` directory (which is tracked in the repo). Only needs to be done once per clone.

### What it checks

| Check | Command | Fails if... |
|-------|---------|-------------|
| Formatting | `cargo fmt --check` | Code doesn't match rustfmt style |
| Lints | `cargo clippy -- -D warnings` | Any clippy or compiler warning exists |

---

## Usage

The program will prompt you for the three inputs interactively:

```
Enter total cards in deck: 60
Enter cards drawn: 7
Enter source cards in deck: 4
Probability: 0.3995
```

---

## Future Plans

- Support for arbitrary k values (e.g., probability of drawing **exactly** 2 sources, or **at least** 3)
- Cumulative probabilities: P(X = k), P(X ≥ k), P(X ≤ k)
- Non-interactive mode via command-line arguments (e.g., `cardodds --deck 60 --draw 7 --source 4`)
- Hypergeometric distribution table output
