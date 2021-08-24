#![deny(missing_docs, warnings)]

//! Open Container Initiative (OCI) Specifications for Rust.

#[macro_use]
mod macros;

#[cfg(feature = "distribution")]
pub mod distribution;
mod error;
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "runtime")]
pub mod runtime;

pub use error::*;
