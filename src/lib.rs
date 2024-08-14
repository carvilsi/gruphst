//! Gruphst
//!
//! An in-memory graph database.
//!
//! Possible to persists on file (just because is something that we always expect from an in-memory databases).

pub mod config;
pub mod edge;
pub mod graphs;
pub mod graphs_stats;
pub mod logger;
mod util;
pub mod vertex;
