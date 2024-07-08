//! Gruphst
//!
//! An in-memory graph database.
//!
//! Possible to persists on file.

pub mod graph;
pub mod graphs;
pub mod node;
pub mod persistence;
pub mod config;

/// Enables logging providing a level
///
/// # Examples
/// ```rust
/// use gruphst::enable_logging;
/// use gruphst::config::get_log_level;
///
/// let log_level = get_log_level();
/// enable_logging(log_level);
/// ```
pub fn enable_logging(level: log::Level) {
    simple_logger::init_with_level(level).unwrap();
}
