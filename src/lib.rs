//! Gruphst
//!
//! An in-memory graph database.
//!
//! Possible to persists on file (just because is something that we always expect from an in-memory databases).

use attributes::Attributes;

pub mod attributes;
pub mod config;
pub mod graph;
pub mod graphs;
pub mod node;
mod util;

// XXX: maybe add set_attributes???
pub trait CURNodeGraph {
    fn new(label: &str) -> Self;
    fn get_id(&self) -> String;
    fn get_label(&self) -> String;
    fn set_label(&mut self, label: &str);
    fn get_attributes(&self) -> Attributes;
}

pub trait RUDAttribute {
    fn set_attr<T>(&mut self, key: &str, val: T)
    where
        T: std::fmt::Display;
    fn get_attr(&self, key: &str) -> Result<&String, &'static str>;
    fn update_attr<T>(&mut self, attr_k: &str, attr_v: T) -> Result<(), &'static str>
    where
        T: std::fmt::Display;
    fn upsert_attr<T>(&mut self, attr_k: &str, attr_v: T)
    where
        T: std::fmt::Display;
    fn del_attr(&mut self, v: &str) -> Result<(), &'static str>;
    fn get_attr_keys(&self) -> Vec<&str>;
}

pub trait QueryAttribute {
    fn has_attr(&self, attr_k: &str) -> bool;
    fn like_attr(&self, attr_k: &str) -> bool;
    fn equals_attr<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone;
    fn len_attr(&self) -> usize;
    fn is_empty_attr(&self) -> bool;
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
