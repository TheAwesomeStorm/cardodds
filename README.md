# cardodds

**cardodds** is a CLI tool that calculates hypergeometric probabilities for card games — the chance of drawing **at least k** copies of a specific card (a "source") from a deck after drawing a given number of cards.

This is commonly used in trading card games (TCGs) like *Magic: The Gathering*, *Yu-Gi-Oh!*, *Pokémon TCG*, and *Hearthstone* to determine the likelihood of having a key card in your opening hand.

---

## The Math

Two probability modes are supported:

### P(X ≥ k) — Basic probability

The probability of drawing **at least k** source cards follows the **hypergeometric distribution**, computed via a recurrence relation:

P(X = 0) = ∏_{i=0}^{n−1} (N − K − i) / (N − i)

P(X = m + 1) = P(X = m) × (K − m)(n − m) / ((m + 1)(N − K − n + m + 1))

P(X ≥ k) = 1 − Σ_{i=0}^{k−1} P(X = i)

Where:

- **N** — total number of cards in the deck
- **n** — number of cards drawn
- **K** — number of source cards in the deck
- **k** — minimum number of source cards desired in hand

#### Worked Example (k = 1)

**Input:** N = 60, n = 7, K = 4, k = 1

P(at least 1) = 1 − (56/60 × 55/59 × 54/58 × 53/57 × 52/56 × 51/55 × 50/54)

Result: **≈ 0.3995** (≈ 39.95%)

---

### P(X ≥ k | Y ≥ g) — Conditional probability

The probability of drawing **at least k** sources **given that at least g** pool cards were drawn. This is useful when the deck has two overlapping card groups — for example, "what's the chance I have at least 2 red sources in hand, given I've drawn at least 3 lands?"

The calculation sums over all possible pool draws Y = y:

P(X ≥ k | Y ≥ g) = Σ_{y = g}^{min(n, G)} P(Y = y) × P(X ≥ k | Y = y)

Where P(Y = y) is the hypergeometric probability of drawing exactly y pool cards, and P(X ≥ k | Y = y) is the probability of finding at least k sources among the drawn pool cards and non-pool draws combined.

Additional variables:

- **G** — total cards in the pool (e.g., lands in deck)
- **g** — minimum number of pool cards drawn

#### Worked Example

**Input:** N = 60, n = 9, K = 18, k = 2, G = 24, g = 3

Result: **≈ 0.9380** (≈ 93.80%)

This answers: "In a 60-card deck with 24 lands and 18 red sources, after drawing 9 cards, what's the probability of having at least 2 red sources given I've drawn at least 3 lands?"

---

## Inputs

| Variable | Description | Example |
|----------|-------------|---------|
| N        | Total cards in deck | 60 |
| n        | Number of cards drawn | 7 |
| K        | Number of source cards in deck | 4 |
| k        | Minimum desired source cards in hand | 1 |
| G        | Total cards in the pool (conditional mode) | 24 |
| g        | Minimum pool cards drawn (conditional mode) | 3 |

## Output

A formatted line showing the result:

```
Probability of having at least 1 card in hand: 0.3995
Probability of having at least 2 cards in hand: 0.0632
```

---

## Building and Installing on Windows

This project is designed to be built and run on **Windows** (the executable is intended for Windows use).

### Prerequisites

Install Rust from [rustup.rs](https://rustup.rs/) if you haven't already. This gives you `cargo` in your terminal (PowerShell, CMD, etc.).

### Install (adds to PATH automatically)

```powershell
cd C:\Users\mendes\Projects\rust\cardodds
cargo install --path .
```

The executable is installed to `%USERPROFILE%\.cargo\bin\cardodds.exe`, which is already in your PATH. You can now run `cardodds` from anywhere.

### Build only (no PATH install)

```powershell
cargo build --release
```

The compiled executable will be at `target\release\cardodds.exe`.

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

Two subcommands — **default** (P(X ≥ k)) and **`conditional`** (P(X ≥ k | Y ≥ g)). Each supports **interactive** (no args) or **CLI arguments**.

---

### Default mode — P(X ≥ k)

#### Interactive

```
> cardodds
Total cards in deck [60]:
Cards drawn [7]:
Source cards in deck [4]:
Minimum desired cards in hand [1]:
Probability of having at least 1 card in hand: 0.3995
```

#### CLI arguments

All arguments are optional — omitted ones default to 60, 7, 4, 1.

```powershell
cardodds --deck 40 --draw 5 --sources 3 --desired 2
```

```powershell
cardodds -N 60 -n 7 -K 4 -k 1
```

| Short | Long | Default |
|-------|------|---------|
| `-N` | `--deck` | 60 |
| `-n` | `--draw` | 7 |
| `-K` | `--sources` | 4 |
| `-k` | `--desired` | 1 |

---

### Conditional mode — P(X ≥ k | Y ≥ g)

#### Interactive

```
> cardodds conditional
Total cards in deck [60]:
Cards drawn [7]:
Source cards in deck [4]:
Minimum desired cards in hand [1]:
Total cards in the pool [24]:
Minimum pool cards drawn [3]:
Probability of having at least 1 source in hand
  given at least 3 pool cards drawn: 0.9943
```

#### CLI arguments

`-K`, `-G`, and `-g` are required. `-N` (default 60), `-n` (default 7), and `-k` (default 1) are optional.

```powershell
cardodds conditional -K 18 -G 24 -g 3 -k 2
```

```powershell
cardodds conditional --sources 18 --pool 24 --pool-drawn 3 --desired 2
```

| Short | Long | Required | Description |
|-------|------|----------|-------------|
| `-N` | `--deck` | No (default 60) | Total cards in deck |
| `-n` | `--draw` | No (default 7) | Cards drawn |
| `-K` | `--sources` | **Yes** | Source cards in deck |
| `-k` | `--desired` | No (default 1) | Minimum desired cards in hand |
| `-G` | `--pool` | **Yes** | Total cards in the pool |
| `-g` | `--pool-drawn` | **Yes** | Minimum pool cards drawn |

