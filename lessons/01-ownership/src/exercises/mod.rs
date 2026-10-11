//! Exercises, from easiest to hardest.
//!
//! Exercises marked "compile error" are gated behind a Cargo feature:
//! `cargo test -p ownership --features ex02`

pub mod ex01_string_ownership;

#[cfg(feature = "ex02")]
pub mod ex02_fix_the_move;

pub mod ex03_borrow_instead_of_move;

pub mod ex04_mutable_references;

#[cfg(feature = "ex05")]
pub mod ex05_borrowing_rules;

pub mod ex06_string_slices;

#[cfg(feature = "ex07")]
pub mod ex07_dangling_references;

pub mod ex08_text_toolkit;
