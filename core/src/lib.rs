//! The Harvest Intermediate Representation ([HarvestIR]), types it depends on (e.g.
//! [Representation]), and utilities for working with them.

pub mod edit;
pub mod fs;
mod id;
pub mod ir;

pub use edit::Edit;
pub use id::Id;
pub use ir::{HarvestIR, Representation};
