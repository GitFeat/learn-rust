# Rust (Personal) Bootstrap

This repo is for tracking my progress with Rust concepts, primarily following the [Rust Programming Language Book](https://doc.rust-lang.org/book/).

**Disclaimer**: If you are an advanced developer, this repo will certainly have no interest for you, if not, feel free to browse or fork the repo if you are also learning the **Rust** language. :)

Make sure you have [Rust and Cargo installed](https://www.rust-lang.org/tools/install) on your machine (via `rustup`). 
I also use [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension to help me during coding in Rust. 

## Getting Started

### 1. Clone the repository
```bash
git clone git@github.com:GitFeat/rust-bootstrap.git
cd rust
```

### 2. Go to a specific chapter
```bash
cd guessing_game
cargo run
```

## Tech Stack & Environment

- Language: Rust (version 1.90.0)
- Build System: Cargo
- Environment: Windows 11 (WSL / Ubuntu)

## Good to know

1. Functions could be deactivated with prefix: _my_func

## Lessons & exercises

Guided lessons with exercises live in [`lessons/`](lessons/), a dedicated Cargo workspace
(every `lessons/NN-topic/` crate is picked up automatically). Solutions are not provided.

| Lesson | Crate | Book chapter |
| --- | --- | --- |
| [01 — Ownership](lessons/01-ownership/LESSON.md) | `ownership` | [4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) |

```bash
cd lessons
cargo test -p ownership                     # todo!() exercises (all fail at first: expected)
cargo test -p ownership ex04                # a single exercise
cargo test -p ownership --features ex02     # opt-in exercise with an intentional compile error
cargo test -p ownership --features broken   # everything — goal: all green
```

Workflow: read `LESSON.md`, replace the `todo!()`s (or fix the code), rerun the tests.
Do not modify the tests.
