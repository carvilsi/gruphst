use log::error;

use crate::edge::Edge;
use crate::graphs::Graphs;
use crate::vertex::Vertex;

impl Graphs {
    /// Returns a collection of Edges that matches the relation
    /// for provided vault or default when None
    pub fn find_edges_by_relation(
        &mut self,
        relation_name: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let edges = edges
                .iter()
                .filter(|edge| edge.get_relation() == relation_name)
                .collect::<Vec<&Edge>>();
            if !edges.is_empty() {
                Ok(edges)
            } else {
                error!("Any edge found for relation: {}", relation_name);
                Err("Any edge found for relation")
            }
        } else {
            Err("provided vault does not exists")
        }
    }

    /// Returns a collection of Edges elements that matches the relations
    /// in the array
    /// for provided vault or default when None
    pub fn find_edges_by_relations(
        &mut self,
        relations: Vec<&str>,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let edges = edges
                .iter()
                .filter(|edge| relations.contains(&edge.get_relation().as_str()))
                .collect::<Vec<&Edge>>();
            if !edges.is_empty() {
                Ok(edges)
            } else {
                error!("Any edge found for relations: {:#?}", relations);
                Err("Any edge found for relation")
            }
        } else {
            Err("provided vault does not exists")
        }
    }

    /// Returns a collection of edges that matches any attribute vertex by key
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_key(
        &mut self,
        attr_k: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let edges = edges
                .iter()
                .filter(|edge| edge.has_vertex_with_attr_key(attr_k))
                .collect::<Vec<&Edge>>();
            if !edges.is_empty() {
                Ok(edges)
            } else {
                error!("Any edge found for attribute: {}", attr_k);
                Err("Any edge found for attribute")
            }
        } else {
            Err("provided vault does not exists")
        }
    }

    /// Returns a collection of edges that matches a string attribute vertex by key
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_str_key(
        &mut self,
        attr_k: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let edges = edges
                .iter()
                .filter(|edge| edge.has_vertex_with_attr_str_key(attr_k))
                .collect::<Vec<&Edge>>();
            if !edges.is_empty() {
                Ok(edges)
            } else {
                error!("Any edge found for attribute: {}", attr_k);
                Err("Any edge found for attribute")
            }
        } else {
            Err("provided vault does not exists")
        }
    }

    /// Returns a collection of edges that matches a vector u8 attribute vertex by key
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_vector_u8_key(
        &mut self,
        attr_k: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let edges = edges
                .iter()
                .filter(|edge| edge.has_vertex_with_attr_vec_u8_key(attr_k))
                .collect::<Vec<&Edge>>();
            if !edges.is_empty() {
                Ok(edges)
            } else {
                error!("Any edge found for attribute: {}", attr_k);
                Err("Any edge found for attribute")
            }
        } else {
            Err("provided vault does not exists")
        }
    }

    /// Returns a collection of edges like string attribute vertex key
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_str_key_like(
        &mut self,
        attr_k: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let vrtcs = edges
                .iter()
                .filter(|edge| edge.has_vertex_with_attr_str_key_like(attr_k))
                .collect::<Vec<&Edge>>();
            if !vrtcs.is_empty() {
                Ok(vrtcs)
            } else {
                error!("Any edge found for attribute: {}", attr_k);
                Err("Any edge found for attribute")
            }
        } else {
            Err("provided vault does not exists")
        }
    }
    
    /// Returns a collection of edges like vector u8 attribute vertex key
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_vec_u8_key_like(
        &mut self,
        attr_k: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let vrtcs = edges
                .iter()
                .filter(|edge| edge.has_vertex_with_attr_vec_u8_key_like(attr_k))
                .collect::<Vec<&Edge>>();
            if !vrtcs.is_empty() {
                Ok(vrtcs)
            } else {
                error!("Any edge found for attribute: {}", attr_k);
                Err("Any edge found for attribute")
            }
        } else {
            Err("provided vault does not exists")
        }
    }
 
    /// Returns a collection of edges like any attribute vertex key
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_key_like(
        &mut self,
        attr_k: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let vrtcs = edges
                .iter()
                .filter(|edge| edge.has_vertex_with_attr_key_like(attr_k))
                .collect::<Vec<&Edge>>();
            if !vrtcs.is_empty() {
                Ok(vrtcs)
            } else {
                error!("Any edge found for attribute: {}", attr_k);
                Err("Any edge found for attribute")
            }
        } else {
            Err("provided vault does not exists")
        }
    }
 
    /// Returns a collection of edges that matches a string attribute vertex 
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_str_equals_to<T>(
        &self,
        attr_k: &str,
        attr_v: T,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, &'static str>
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let vrtcs = edges
                .iter()
                .filter(|edge| edge.has_vertex_with_attr_str_value_equals_to(attr_k, attr_v.clone()))
                .collect::<Vec<&Edge>>();
            if !vrtcs.is_empty() {
                Ok(vrtcs)
            } else {
                error!("Any edge found for attribute: {}", attr_k);
                Err("Any edge found for attribute")
            }
        } else {
            Err("provided vault does not exists")
        }
    }

    /// Returns an Edge that provided id matches with Edge Id, or From, To vertices
    /// for some provided vault_name or default when None
    pub fn find_edge_by_id(
        &mut self,
        id: &str,
        vault_name: Option<&str>,
    ) -> Result<&mut Edge, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get_mut(&current_vault) {
            if let Some(edge) = edges.iter_mut().find(|edge| {
                edge.get_id() == id
                    || edge.get_from_vertex().get_id() == id
                    || edge.get_to_vertex().get_id() == id
            }) {
                Ok(edge)
            } else {
                error!("edge with id [{}] not found", id);
                Err("edge not found")
            }
        } else {
            Err("provided vault does not exists")
        }
    }

    /// Find edge by id on any graphs' vault
    pub fn find_edge_by_id_in_graphs(&mut self, id: &str) -> Result<&mut Edge, &'static str> {
        for (_vault_name, edges) in self.vault.iter_mut() {
            if let Some(edge) = edges.iter_mut().find(|vrtx| {
                vrtx.get_id() == id
                    || vrtx.get_from_vertex().get_id() == id
                    || vrtx.get_to_vertex().get_id() == id
            }) {
                return Ok(edge);
            }
        }
        Err("edge not found")
    }

    // TODO: check and try to improve performance; instead match find_edge_by_id
    /// Returns a Vertex that provided id matches with id of From, To vertices
    /// for some provided vault_name or default when None
    pub fn find_vertex_by_id(
        &mut self,
        id: &str,
        vault_name: Option<&str>,
    ) -> Result<Vertex, &'static str> {
        match self.find_edge_by_id(id, vault_name) {
            Ok(edge) => {
                if let Ok(vertex) = edge.find_vertex_by_id(id) {
                    Ok(vertex)
                } else {
                    Err("Vertex not found")
                }
            },
            Err(error) => Err(error),
        }  
    }

    /// Returns a Vertex that provided id matches with id of From, To vertices
    /// on any graphs' vault
    pub fn find_vertex_by_id_in_graphs(&mut self, id: &str) -> Result<Vertex, &'static str> {
        for (_vault_name, edges) in self.vault.iter_mut() {
            for edge in edges {
                if let Ok(vertex) = edge.find_vertex_by_id(id) {
                    return Ok(vertex);
                }
            }
        }
        Err("Vertex found")
    }

    /// Retrieves all the vertices with incoming relation
    /// for some provided vault_name or default when None
    pub fn find_vertices_with_relation_in(
        &self,
        relation_in: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<Vertex>, &'static str> {
        let mut relations_in: Vec<Vertex> = Vec::new();
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            for edge in edges {
                if edge.get_relation() == relation_in
                    && !relations_in.contains(&edge.get_to_vertex())
                {
                    relations_in.push(edge.get_to_vertex().clone());
                }
            }
        } else {
            return Err("provided vault does not exists");
        }
        if !relations_in.is_empty() {
            Ok(relations_in)
        } else {
            Err("any vertex found with relation in")
        }
    }

    /// Retrieves all the vertices with outcoming relation
    /// for some provided vault_name or default when None
    pub fn find_vertices_with_relation_out(
        &self,
        relation_out: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<Vertex>, &'static str> {
        let mut relations_out: Vec<Vertex> = Vec::new();
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            for edge in edges {
                if edge.get_relation() == relation_out
                    && !relations_out.contains(&edge.get_from_vertex())
                {
                    relations_out.push(edge.get_from_vertex().clone());
                }
            }
        } else {
            return Err("provided vault does not exists");
        }
        if !relations_out.is_empty() {
            Ok(relations_out)
        } else {
            Err("any vertex found with relation out")
        }
    }
}

// TODO: review this whole query
// needs methods:
// - retrieve vertex by attrs
// 
