use super::Vertex;

impl Vertex {
    /// Checks if an attribute key exists
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Frodo");
    /// vertex.set_attr("surname", "Baggins");
    /// 
    /// assert!(vertex.has_attr("surname"));
    /// assert!(!vertex.has_attr("age"));
    /// ```
    pub fn has_attr(&self, attr_k: &str) -> bool {
        self.vrtx.borrow().attr.contains_key(attr_k)
    }

    /// Checks if an attribute key is like on a vertex
    /// 
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Frodo");
    /// vertex.set_attr("surname", "Baggins");
    /// 
    /// assert!(vertex.has_attr_like("SuRn"));
    /// assert!(!vertex.has_attr_like("ag"));
    /// ```
    pub fn has_attr_like(&self, attr_k: &str) -> bool {
        for key in self.vrtx.borrow().attr.keys() {
            if key.to_lowercase().contains(&attr_k.to_lowercase()) {
                return true;
            }
        }
        false
    }

    /// Checks if an attribute matches on a vertex
    /// 
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Frodo");
    /// vertex.set_attr("surname", "Baggins");
    /// 
    /// assert!(vertex.attr_equals_to("surname", "Baggins"));
    /// assert!(!vertex.attr_equals_to("surname", "Brandigamo"));
    /// assert!(!vertex.attr_equals_to("age", 42));
    /// ```
    pub fn attr_equals_to<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        match self.vrtx.borrow().attr.get(attr_k) {
            Some(val) => {
                let v = attr_v.clone();
                *val == v.to_string()
            }
            None => false,
        }
    }

    /// Retrieves the lenght of attributes for a vertex
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Frodo");
    /// vertex.set_attr("surname", "Baggins");
    /// vertex.set_attr("weapon", "Sting");
    /// 
    /// assert_eq!(vertex.attr_len(), 2);
    /// ```
    pub fn attr_len(&self) -> usize {
        self.vrtx.borrow().attr.len()
    }

    /// Checks if attributes for a vertex is empty
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Frodo");
    /// 
    /// assert!(vertex.attr_is_empty());
    /// 
    /// vertex.set_attr("surname", "Baggins");
    /// vertex.set_attr("weapon", "Sting");
    /// 
    /// assert!(!vertex.attr_is_empty());
    /// ```
    pub fn attr_is_empty(&self) -> bool {
        self.attr_len() == 0
    }
}
