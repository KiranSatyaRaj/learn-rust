// here on the comments written will be documentation comment
//! # Cargo and Crates
//!
//! `cargo-and-crates` is a sample crate for playing with cargo cli and knowing functionality.
//! I'm learning this a part of rust book chapter and it contains utilities to perform calculations

/// Adds one to the number given
///
/// # Example
///
/// ```
/// let arg = 5;
/// let answer = cargo_and_crates::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
// this documentation comments will be generated as
// html content when executed with cargo doc
pub fn add_one(x: i32) -> i32 {
    x + 1
}