//! Gruphst
//!
//! An in-memory graph database.
//!
//! Possible to persists on file.

pub mod graph;
pub mod graphs;
pub mod node;
pub mod persistence;

// TODO: add env config for level
/// Enables logging providing a level
///
/// # Examples
/// ```rust
/// use gruphst::enable_logging;
///
/// enable_logging(log::Level::Info);
/// ```
pub fn enable_logging(level: log::Level) {
    simple_logger::init_with_level(level).unwrap();
}
