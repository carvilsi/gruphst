use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Representation of a Node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    /// A Node consists on a uuid as identifier
    pub id: String,
    /// And a name
    pub name: String,
    attr: HashMap<String, String>,
}

impl Node {
    /// Creates a Node with the given name, the id is generated
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    ///
    /// let node = Node::new("alice node");
    /// ```
    pub fn new(name: &str) -> Self {
        let node = Node {
            name: String::from(name),
            id: Uuid::new_v4().to_string(),
            attr: HashMap::new(),
        };
        debug!("The created node: {:#?}", &node);
        node
    }

    /// Updates the name of the Node
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    ///
    ///
    /// let mut node = Node::new("alice node");
    /// assert_eq!(node.name, "alice node");
    /// node.update_name("just alice");
    /// assert_eq!(node.name, "just alice");
    /// ```
    pub fn update_name(&mut self, name: &str) {
        debug!("Updated Node [{}] with name: {}", self.id, name);
        self.name = name.to_string();
    }

    /// Set attributes for a Node
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    ///
    /// let mut node = Node::new("Alice");
    /// node.set_attr("Address", "Elm street");
    /// ```
    pub fn set_attr<T>(&mut self, attr_k: &str, attr_v: T)
    where
        T: std::fmt::Display,
    {
        self.attr.insert(attr_k.to_string(), attr_v.to_string());
        debug!(
            "added attribute key: {} with value {} for node {}",
            attr_k, attr_v, self.id
        );
    }

    /// Get attributes for a Node
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    ///
    /// let mut node = Node::new("Alice");
    /// node.set_attr("Address", "Elm street");
    /// let attr = node.get_attr("Address").unwrap();
    /// assert_eq!(attr, "Elm street");
    /// ```
    pub fn get_attr(&self, attr_k: &str) -> Result<&String, &'static str> {
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

    /// Returns an Array containing all attribute keys
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    ///
    /// let mut node = Node::new("Alice");
    /// node.set_attr("Address", "Elm street");
    /// node.set_attr("age", 44);
    /// let keys = node.get_attr_keys();
    /// assert!(keys.contains(&&"Address"));
    /// assert!(keys.contains(&&"age"));
    /// ```
    pub fn get_attr_keys(&self) -> Vec<&str> {
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

    /// Updates the value of an attribute
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
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
    pub fn update_attr<T>(&mut self, attr_k: &str, attr_v: T) -> Result<(), &'static str>
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
    pub fn upsert_attr<T>(&mut self, attr_k: &str, attr_v: T)
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

    /// Retrieves the lenght of attributes for a Node
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    ///
    /// let mut node = Node::new("Alice");
    /// node.set_attr("Address", "Elm street");
    /// node.set_attr("age", 25);
    /// assert_eq!(node.len_attr(), 2);
    /// ```
    pub fn len_attr(&self) -> usize {
        self.attr.len()
    }

    /// Checks if attributes for a Node is empty
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    ///
    /// let mut node = Node::new("Alice");
    /// assert!(node.is_empty_attr());
    /// node.set_attr("Address", "Elm street");
    /// node.set_attr("age", 25);
    /// assert!(!node.is_empty_attr());
    /// ```
    pub fn is_empty_attr(&self) -> bool {
        self.len_attr() == 0
    }

    /// Deletes an attribute for a Node
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    ///
    /// let mut node = Node::new("Alice");
    /// assert!(node.is_empty_attr());
    /// node.set_attr("Address", "Elm street");
    /// assert!(!node.is_empty_attr());
    /// node.del_attr("Address");
    /// assert!(node.is_empty_attr());
    /// ```
    pub fn del_attr(&mut self, v: &str) -> Result<(), &'static str> {
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
}
