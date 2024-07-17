use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::RUDAttribute;

mod query;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    attr: HashMap<String, String>,
    id: String,
}

impl Default for Attributes {
    fn default() -> Self {
        Self::new()
    }
}

impl RUDAttribute for Attributes {
    /// Set attributes for a Node
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use crate::gruphst::*;
    ///
    /// let mut node = Node::new("Alice");
    /// node.set_attr("Address", "Elm street");
    /// ```
    fn set_attr<T>(&mut self, attr_k: &str, attr_v: T)
    where
        T: std::fmt::Display,
    {
        self.attr.insert(attr_k.to_string(), attr_v.to_string());
        debug!(
            "added attribute key: {} with value {} for node {}",
            attr_k, attr_v, self.id
        );
    }

    /// Get attribute for a Node
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use crate::gruphst::*;
    ///
    /// let mut node = Node::new("Alice");
    /// node.set_attr("Address", "Elm street");
    /// let attr = node.get_attr("Address").unwrap();
    /// assert_eq!(attr, "Elm street");
    /// ```
    fn get_attr(&self, attr_k: &str) -> Result<&String, &'static str> {
        let res = self.attr.get(attr_k);
        match res {
            Some(res) => {
                debug!(
                    "retrieved attribute value '{}' for '{}' for node [{}]",
                    res, attr_k, self.id
                );
                Ok(res)
            }
            None => {
                warn!("attribute '{}' not found", attr_k);
                Err("attribute not found")
            }
        }
    }

    /// Updates the value of an attribute
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use crate::gruphst::*;
    ///
    /// let mut node = Node::new("Alice");
    /// node.set_attr("Address", "Elm street");
    /// node.set_attr("age", 44);
    ///
    /// assert_eq!(node.get_attr("age").unwrap(), "44");
    ///
    /// node.update_attr("age", 55);
    /// assert_eq!(node.get_attr("age").unwrap(), "55");
    /// ```
    fn update_attr<T>(&mut self, attr_k: &str, attr_v: T) -> Result<(), &'static str>
    where
        T: std::fmt::Display,
    {
        debug!(
            "updated attribute key: {} with value {} for node {}",
            attr_k, attr_v, self.id
        );
        if let Some(attr) = self.attr.get_mut(attr_k) {
            *attr = attr_v.to_string();
            return Ok(());
        }
        Err("not attribute found to update")
    }

    /// Updates the value of an attribute or creates a new one if attribute key does not exists
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use crate::gruphst::*;
    ///
    /// let mut node = Node::new("Alice");
    /// node.set_attr("Address", "Elm street");
    /// assert_eq!(node.len_attr(), 1);
    /// node.upsert_attr("age", 44);
    /// assert_eq!(node.len_attr(), 2);
    /// assert_eq!(node.get_attr("age").unwrap(), "44");
    ///
    /// node.upsert_attr("age", 55);
    /// assert_eq!(node.get_attr("age").unwrap(), "55");
    /// ```
    fn upsert_attr<T>(&mut self, attr_k: &str, attr_v: T)
    where
        T: std::fmt::Display,
    {
        match self.attr.get_mut(attr_k) {
            Some(attr) => {
                *attr = attr_v.to_string();
                debug!(
                    "updated (upsert) attribute key: {} with value {} for node {}",
                    attr_k, attr_v, self.id
                );
            }
            None => {
                self.attr.insert(attr_k.to_string(), attr_v.to_string());
                debug!(
                    "added (upsert) attribute key: {} with value {} for node {}",
                    attr_k, attr_v, self.id
                );
            }
        }
    }

    /// Deletes an attribute
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use crate::gruphst::*;
    ///
    /// let mut node = Node::new("Alice");
    /// assert!(node.is_empty_attr());
    /// node.set_attr("Address", "Elm street");
    /// assert!(!node.is_empty_attr());
    /// node.del_attr("Address");
    /// assert!(node.is_empty_attr());
    /// ```
    fn del_attr(&mut self, v: &str) -> Result<(), &'static str> {
        let res = self.attr.remove(v);
        match res {
            Some(_) => {
                debug!("Removed '{}' attribute for {}", v, self.id);
                Ok(())
            }
            None => {
                warn!("attribute {} not found for remove", v);
                Err("attribute not found for remove")
            }
        }
    }

    /// Returns an Array containing all attribute keys
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use crate::gruphst::*;
    ///
    /// let mut node = Node::new("Alice");
    /// node.set_attr("Address", "Elm street");
    /// node.set_attr("age", 44);
    /// let keys = node.get_attr_keys();
    /// assert!(keys.contains(&&"Address"));
    /// assert!(keys.contains(&&"age"));
    /// ```
    fn get_attr_keys(&self) -> Vec<&str> {
        let mut key_vec = Vec::new();
        for key in self.attr.keys() {
            key_vec.push(key.as_str());
        }
        debug!(
            "requested array of attributes for {} node {:#?}",
            self.id, key_vec
        );
        key_vec
    }
}

impl Attributes {
    pub fn new() -> Self {
        Attributes {
            attr: HashMap::new(),
            id: Uuid::new_v4().to_string(),
        }
    }
}
