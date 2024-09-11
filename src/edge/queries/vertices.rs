use crate::{edge::Edge, errors::GruPHstError, vertex::Vertex};

impl Edge {
    /// Checks if "from" or "to" vertices has any attribute key
    pub fn has_vertex_with_attr_key(&self, attr_k: &str) -> bool {
        self.get_from_vertex().has_attr_key(attr_k) || self.get_to_vertex().has_attr_key(attr_k)
    }

    /// Checks if "from" or "to" vertices has an String attribute key
    pub fn has_vertex_with_attr_str_key(&self, attr_k: &str) -> bool {
        self.get_from_vertex().has_attr_str_key_equals_to(attr_k) || self.get_to_vertex().has_attr_str_key_equals_to(attr_k)
    }
    
    /// Checks if "from" or "to" vertex has any attribute like
    pub fn has_vertex_with_attr_key_like(&self, attr_k: &str) -> bool {
        self.get_from_vertex().has_attr_key_like(attr_k) || self.get_to_vertex().has_attr_key_like(attr_k)
    }

    /// Checks if "from" or "to" vertex has a string attribute key like
    pub fn has_vertex_with_attr_str_key_like(&self, attr_k: &str) -> bool {
        self.get_from_vertex().has_attr_str_key_like(attr_k) || self.get_to_vertex().has_attr_str_key_like(attr_k)
    }

    /// Checks if "from" or "to" vertex has a string attribute and equal for value
    pub fn has_vertex_with_attr_str_value_equals_to<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        self.get_from_vertex()
            .has_attr_str_equals_to(attr_k, attr_v.clone())
            || self.get_to_vertex().has_attr_str_equals_to(attr_k, attr_v.clone())
    }

    /// Checks if "from" or "to" vertices has a Vec<u8> attribute key
    pub fn has_vertex_with_attr_vec_u8_key(&self, attr_k: &str) -> bool {
        self.get_from_vertex().has_attr_vec_u8_key_equals_to(attr_k) || self.get_to_vertex().has_attr_vec_u8_key_equals_to(attr_k)
    }

    /// Checks if "from" or "to" vertex has a vec_u8 attribute key like
    pub fn has_vertex_with_attr_vec_u8_key_like(&self, attr_k: &str) -> bool {
        self.get_from_vertex().has_attr_vec_u8_key_like(attr_k) || self.get_to_vertex().has_attr_vec_u8_key_like(attr_k)
    }

    /// Checks if "from" or "to" vertex has a Vec<u8> attribute and equal for value
    pub fn has_vertex_with_attr_vec_u8_value_equals_to(&self, attr_k: &str, attr_v: &Vec<u8>) -> bool {
        self.get_from_vertex()
            .has_attr_vec_u8_equals_to(attr_k, attr_v)
            || self.get_to_vertex().has_attr_vec_u8_equals_to(attr_k, attr_v)
    }

    /// finds a Vertex by id on an Edge on "from" and "to" vertices
    pub fn find_vertex_by_id(&self, id: &str) -> Result<Vertex, GruPHstError> {
        if self.get_from_vertex().get_id() == *id {
            Ok(self.get_from_vertex())
        } else if self.get_to_vertex().get_id() == *id {
            Ok(self.get_to_vertex())
        } else {
            Err(GruPHstError::VertexError)
        }
    }
}
