use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::attributes::Attributes;
use crate::CURNodeGraph;
use crate::RUDAttribute;

mod query;

/// Representation of a Node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    /// A Node consists on a uuid as identifier
    id: String,
    /// And a name
    label: String,
    /// The attributes for a node
    attr: Attributes,
}

impl CURNodeGraph for Node {
    /// Creates a Node with the given label, the id is generated
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use crate::gruphst::*;
    ///
    /// let node = Node::new("alice node");
    /// ```
    fn new(label: &str) -> Self {
        let node = Node {
            label: String::from(label),
            id: Uuid::new_v4().to_string(),
            attr: Attributes::new(),
        };
        debug!("The created node: {:#?}", &node);
        node
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_label(&self) -> String {
        self.label.clone()
    }

    fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }

    fn get_attributes(&self) -> Attributes {
        self.attr.clone()
    }

    fn set_attributes(&mut self, attributes: Attributes) {
        self.attr = attributes;
    }
}

impl RUDAttribute for Node {
    fn set_attr<T>(&mut self, key: &str, val: T)
    where
        T: std::fmt::Display,
    {
        self.attr.set_attr(key, val);
    }

    fn get_attr(&self, key: &str) -> Result<&String, &'static str> {
        self.attr.get_attr(key)
    }

    fn update_attr<T>(&mut self, attr_k: &str, attr_v: T) -> Result<(), &'static str>
    where
        T: std::fmt::Display,
    {
        self.attr.update_attr(attr_k, attr_v)
    }

    fn upsert_attr<T>(&mut self, attr_k: &str, attr_v: T)
    where
        T: std::fmt::Display,
    {
        self.attr.upsert_attr(attr_k, attr_v)
    }

    fn del_attr(&mut self, v: &str) -> Result<(), &'static str> {
        self.attr.del_attr(v)
    }

    fn get_attr_keys(&self) -> Vec<&str> {
        self.attr.get_attr_keys()
    }
}
