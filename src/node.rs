use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

mod query;
mod attributes;

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
}
