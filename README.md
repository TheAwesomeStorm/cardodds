# cardodds

**cardodds** is a CLI tool that calculates hypergeometric probabilities for card games — the chance of drawing **at least k** copies of a specific card (a "source") from a deck after drawing a given number of cards.

This is commonly used in trading card games (TCGs) like *Magic: The Gathering*, *Yu-Gi-Oh!*, *Pokémon TCG*, and *Hearthstone* to determine the likelihood of having a key card in your opening hand.

---

## The Math

The probability of drawing **at least k** source cards follows the **hypergeometric distribution**, computed via a recurrence relation:

P(X = 0) = ∏_{i=0}^{n−1} (N − K − i) / (N − i)

P(X = m + 1) = P(X = m) × (K − m)(n − m) / ((m + 1)(N − K − n + m + 1))

P(X ≥ k) = 1 − Σ_{i=0}^{k−1} P(X = i)

Where:

- **N** — total number of cards in the deck
- **n** — number of cards drawn
- **K** — number of source cards in the deck
- **k** — minimum number of source cards desired in hand

### Worked Example (k = 1)

**Input:** N = 60, n = 7, K = 4, k = 1

P(at least 1) = 1 − (56/60 × 55/59 × 54/58 × 53/57 × 52/56 × 51/55 × 50/54)

Result: **≈ 0.3995** (≈ 39.95%)

---

## Inputs

| Variable | Description | Example |
|----------|-------------|---------|
| N        | Total cards in deck | 60 |
| n        | Number of cards drawn | 7 |
| K        | Number of source cards in deck | 4 |
| k        | Minimum desired source cards in hand | 1 |

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

Two modes — **interactive** (default) or **CLI arguments**.

### Interactive mode

Run without arguments and enter values at the prompts. Press Enter to accept defaults (60, 7, 4, 1).

```
> cardodds
Total cards in deck [60]:
Cards drawn [7]:
Source cards in deck [4]:
Minimum desired cards in hand [1]:
Probability of having at least 1 card in hand: 0.3995
```

### CLI arguments

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

