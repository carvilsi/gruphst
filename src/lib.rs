//! Gruphst
//!
//! An in-memory graph database.
//!
//! Possible to persists on file (just because is something that we always expect from an in-memory databases).

pub mod attributes;
pub mod config;
pub mod edge;
pub mod graphs;
pub mod vertex;

pub trait RUDAttribute {
    fn set<T>(&mut self, key: &str, val: T)
    where
        T: std::fmt::Display;
    fn get(&self, key: &str) -> Result<&String, &'static str>;
    fn update<T>(&mut self, attr_k: &str, attr_v: T) -> Result<(), &'static str>
    where
        T: std::fmt::Display;
    fn upsert<T>(&mut self, attr_k: &str, attr_v: T)
    where
        T: std::fmt::Display;
    fn delete(&mut self, v: &str) -> Result<(), &'static str>;
    fn get_keys(&self) -> Vec<&str>;
}

pub trait QueryAttribute {
    fn has(&self, attr_k: &str) -> bool;
    fn like(&self, attr_k: &str) -> bool;
    fn equals_to<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

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
