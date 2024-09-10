#![doc = include_str!("../README.md")]

pub mod config;
pub mod edge;
pub mod graphs;
pub mod graphs_stats;
pub mod logger;
mod util;
pub mod vertex;

// TODO: add an importer/exporter thingy; to_csv, from_csv
// TODO: Improve Error handling