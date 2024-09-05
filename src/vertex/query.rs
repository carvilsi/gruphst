use super::Vertex;

impl Vertex {
    /// Checks if an attribute key exists
    /// on String attribute
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Frodo");
    /// vertex.set_attr("surname", "Baggins");
    ///
    /// assert!(vertex.has_attr_str_key_equals_to("surname"));
    /// assert!(!vertex.has_attr_str_key_equals_to("age"));
    /// ```
    pub fn has_attr_str_key_equals_to(&self, attr_k: &str) -> bool {
        self.vrtx.borrow().attr.contains_key(attr_k)
    }

    /// Checks if an Vec<u8> attribute key exists
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Frodo");
    /// vertex.set_attr("surname", "Baggins");
    /// 
    /// let vu8: Vec<u8> = vec![3, 1, 3, 3, 7];
    /// vertex.set_attr_vec_u8("code", &vu8);
    ///
    /// assert!(vertex.has_attr_vec_u8_key_equals_to("code"));
    /// assert!(!vertex.has_attr_vec_u8_key_equals_to("edoc"));
    /// ```
    pub fn has_attr_vec_u8_key_equals_to(&self, attr_k: &str) -> bool {
        self.vrtx.borrow().attr_vec_u8.contains_key(attr_k)
    }
    
    /// Checks if an attribute key exists
    /// either on String or Vec<u8> attribute
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Frodo");
    /// vertex.set_attr("surname", "Baggins");
    /// 
    /// let vu8: Vec<u8> = vec![3, 1, 3, 3, 7];
    /// vertex.set_attr_vec_u8("code", &vu8);
    ///
    /// assert!(vertex.has_attr_key("surname"));
    /// assert!(vertex.has_attr_key("code"));
    /// assert!(!vertex.has_attr_key("age"));
    /// ```
    pub fn has_attr_key(&self, attr_k: &str) -> bool {
        self.vrtx.borrow().attr.contains_key(attr_k) || self.vrtx.borrow().attr_vec_u8.contains_key(attr_k)
    }

    /// Checks if an attribute values is like on a vertex
    /// 
    /// # Examples
    /// ```rust 
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Frodo");
    /// vertex.set_attr("surname", "Baggins");
    ///
    /// assert!(vertex.has_attr_like("gGin"));
    /// assert!(!vertex.has_attr_like("Sur"));
    /// ```
    pub fn has_attr_like<T>(&self, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        for (_key, val) in self.vrtx.borrow().attr.clone().into_iter() {
            let v = attr_v.to_string().to_lowercase();
            if val.to_lowercase().contains(&v) {
                return true;
            }
        }
        false
    }

    /// Checks if an String attribute key is like on a vertex
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Frodo");
    /// vertex.set_attr("surname", "Baggins");
    ///
    /// assert!(vertex.has_attr_str_key_like("SuRn"));
    /// assert!(!vertex.has_attr_str_key_like("ag"));
    /// ```
    pub fn has_attr_str_key_like(&self, attr_k: &str) -> bool {
        for key in self.vrtx.borrow().attr.keys() {
            if key.to_lowercase().contains(&attr_k.to_lowercase()) {
                return true;
            }
        }
        false
    }

    /// Checks if an Vec<u8> attribute key is like on a vertex 
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Frodo");
    /// vertex.set_attr("surname", "Baggins");
    /// 
    /// let vu8: Vec<u8> = vec![3, 1, 3, 3, 7];
    /// vertex.set_attr_vec_u8("code", &vu8);
    ///
    /// assert!(vertex.has_attr_vec_u8_key_like("oDe"));
    /// assert!(!vertex.has_attr_vec_u8_key_like("dOC"));
    /// ```
    pub fn has_attr_vec_u8_key_like(&self, attr_k: &str) -> bool {
        for key in self.vrtx.borrow().attr_vec_u8.keys() {
            if key.to_lowercase().contains(&attr_k.to_lowercase()) {
                return true;
            }
        }
        false
    }

    /// Checks if an String attribute value matches on a vertex
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Frodo");
    /// vertex.set_attr("surname", "Baggins");
    ///
    /// assert!(vertex.has_attr_str_equals_to("surname", "Baggins"));
    /// assert!(!vertex.has_attr_str_equals_to("surname", "Brandigamo"));
    /// assert!(!vertex.has_attr_str_equals_to("age", 42));
    /// ```
    pub fn has_attr_str_equals_to<T>(&self, attr_k: &str, attr_v: T) -> bool
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

    /// Checks if a Vec<u8> attribute value matches on a vertex
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Frodo");
    /// vertex.set_attr("surname", "Baggins");
    /// 
    /// let vu8: Vec<u8> = vec![3, 1, 3, 3, 7];
    /// let v_nok: Vec<u8> = vec![1, 0, 1];
    /// vertex.set_attr_vec_u8("code", &vu8);
    ///
    /// assert!(vertex.has_attr_vec_u8_equals_to("code", &vu8));
    /// assert!(!vertex.has_attr_vec_u8_equals_to("code", &v_nok));
    /// assert!(!vertex.has_attr_vec_u8_equals_to("edoc", &vu8));
    /// ```
    pub fn has_attr_vec_u8_equals_to(&self, attr_k: &str, attr_v: &Vec<u8>) -> bool {
        match self.vrtx.borrow().attr_vec_u8.get(attr_k) {
            Some(val) => {
                // let v = attr_v.clone();
                *val == *attr_v
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
    /// let v: Vec<u8> = vec![3, 1, 3, 3, 7];
    /// vertex.set_attr_vec_u8("code", &v);
    /// assert_eq!(vertex.attrs_len(), 3);
    /// ```
    pub fn attrs_len(&self) -> usize {
        let mut c = self.vrtx.borrow().attr.len();
        c += self.vrtx.borrow().attr_vec_u8.len();
        c
    }

    /// Checks if attributes for a vertex is empty
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Frodo");
    ///
    /// assert!(vertex.attrs_empty());
    ///
    /// vertex.set_attr("surname", "Baggins");
    /// vertex.set_attr("weapon", "Sting");
    ///
    /// assert!(!vertex.attrs_empty());
    /// ```
    pub fn attrs_empty(&self) -> bool {
        self.attrs_len() == 0
    }
}
