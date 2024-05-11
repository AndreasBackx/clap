//! Complete commands within shells
//!
//! For quick-start, see [`shells::CompleteCommand`]

mod complete;
mod completion;
mod registrar;

pub mod shells;

pub use registrar::*;
