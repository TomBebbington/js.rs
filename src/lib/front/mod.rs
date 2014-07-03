#[macro_escape]
/// A macro which makes Javascript objects with pretty Rust syntax
pub mod macro;
/// Backend-defining traits
pub mod run;
/// The Javascript standard library
pub mod stdlib;