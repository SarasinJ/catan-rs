# catan-rs

A multiplayer Settlers of Catan server, built as a learning project. Unofficial and
unaffiliated — just a fan implementation for playing with friends and experimenting
with bots.

## What this is

- **`catan-core`** — the game engine. Pure Rust, no I/O, no networking. Every rule of
  the game lives here as small, tested functions. If you're new to the project, this
  is where you'll work.
- **`catan-server`** — WebSocket server that hosts games and talks to clients.
  *(coming in M4)*
- **`client/`** — a thin web client for playing and debugging. *(coming in M5)*

Down the road: baseline bots, self-play evaluation, and AWS deployment.

The engine is deliberately pure and deterministic (randomness is always injected):
this makes it easy to test, easy to replay, and — later — fast to run headless for
bot training.

## Getting started

You need [Rust](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Then clone the repo and run the tests:

```bash
git clone https://github.com/SarasinJ/catan-rs.git && cd catan-rs
cargo test --workspace
```

## Working on the project

**Start here → [CONTRIBUTING.md](CONTRIBUTING.md).** It's short and it answers
"what do I do next?" — the loop is: pick the lowest-numbered open issue in the
current milestone (skip `design` and `blocked` labels), branch, PR, merge.

Every issue is fully specced: what to build, what must hold true, and which tests prove it. You should never need to guess what "done" means.

## Repository layout

The workspace is organized around a small Rust monorepo:

```text
.
├── Cargo.toml
├── CONTRIBUTING.md
├── README.md
└── crates/
    ├── catan-core/
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    └── catan-server/
        ├── Cargo.toml
        └── src/
            └── main.rs
```

Most engine work happens in the core crate under crates/catan-core/src/.

## Project status

Early days: core game types and the board (M1–M2). Nothing playable yet — we're building the engine from the ground up, one small piece at a time.