use log::warn;

use crate::edge::Edge;
use crate::errors::GruPHstError;
use crate::graphs::Graphs;

impl Graphs {
    /// Returns a collection of Edges that matches the relation
    /// for provided vault or default when None
    pub fn find_edges_by_relation(
        &mut self,
        relation_name: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, GruPHstError> {
        let edges = self.select_vautl(vault_name)?;
        let result = edges.iter()
            .filter(|edge| edge.get_relation() == relation_name)
            .collect::<Vec<&Edge>>();
        if !result.is_empty() {
            Ok(result)
        } else {
            warn!("Any edge found for relation: {}", relation_name);
            Err(GruPHstError::EdgeNotFound)
        }
    }

    /// Returns a collection of Edges elements that matches the relations
    /// in the array
    /// for provided vault or default when None
    pub fn find_edges_by_relations(
        &mut self,
        relations: Vec<&str>,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, GruPHstError> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let edges = edges
                .iter()
                .filter(|edge| relations.contains(&edge.get_relation().as_str()))
                .collect::<Vec<&Edge>>();
            if !edges.is_empty() {
                Ok(edges)
            } else {
                warn!("Any edge found for relations: {}", relations.join(", "));
                Err(GruPHstError::EdgeNotFound)
            }
        } else {
            warn!("Vault {} does not exists", current_vault);
            Err(GruPHstError::VaultNotExists(current_vault))
        }
    }
 
    /// Returns a collection of edges like any attribute vertex key
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_key_like(
        &mut self,
        attr_k: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, GruPHstError> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let vrtcs = edges
                .iter()
                .filter(|edge| edge.has_vertex_with_attr_key_like(attr_k))
                .collect::<Vec<&Edge>>();
            if !vrtcs.is_empty() {
                Ok(vrtcs)
            } else {
                warn!("Any edge found for attribute: {}", attr_k);
                Err(GruPHstError::EdgeNotFound)
            }
        } else {
            warn!("Vault {} does not exists", current_vault);
            Err(GruPHstError::VaultNotExists(current_vault))
        }
    }

    /// Returns a collection of edges that matches any attribute vertex by key
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_key(
        &mut self,
        attr_k: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, GruPHstError> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let edges = edges
                .iter()
                .filter(|edge| edge.has_vertex_with_attr_key(attr_k))
                .collect::<Vec<&Edge>>();
            if !edges.is_empty() {
                Ok(edges)
            } else {
                warn!("Any edge found for attribute: {}", attr_k);
                Err(GruPHstError::EdgeNotFound)
            }
        } else {
            warn!("Vault {} does not exists", current_vault);
            Err(GruPHstError::VaultNotExists(current_vault))
        }
    }

    /// Returns a collection of edges that matches a string attribute vertex by key
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_str_key(
        &mut self,
        attr_k: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, GruPHstError> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let edges = edges
                .iter()
                .filter(|edge| edge.has_vertex_with_attr_str_key(attr_k))
                .collect::<Vec<&Edge>>();
            if !edges.is_empty() {
                Ok(edges)
            } else {
                warn!("Any edge found for attribute: {}", attr_k);
                Err(GruPHstError::EdgeNotFound)
            }
        } else {
            warn!("Vault {} does not exists", current_vault);
            Err(GruPHstError::VaultNotExists(current_vault))
        }
    }

    /// Returns a collection of edges like string attribute vertex key
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_str_key_like(
        &mut self,
        attr_k: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, GruPHstError> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let vrtcs = edges
                .iter()
                .filter(|edge| edge.has_vertex_with_attr_str_key_like(attr_k))
                .collect::<Vec<&Edge>>();
            if !vrtcs.is_empty() {
                Ok(vrtcs)
            } else {
                warn!("Any edge found for attribute: {}", attr_k);
                Err(GruPHstError::EdgeNotFound)
            }
        } else {
            warn!("Vault {} does not exists", current_vault);
            Err(GruPHstError::VaultNotExists(current_vault))
        }
    }
 
 
    /// Returns a collection of edges that matches a string attribute vertex 
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_str_equals_to<T>(
        &self,
        attr_k: &str,
        attr_v: T,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, GruPHstError>
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
                warn!("Any edge found for attribute: {}", attr_k);
                Err(GruPHstError::EdgeNotFound)
            }
        } else {
            warn!("Vault {} does not exists", current_vault);
            Err(GruPHstError::VaultNotExists(current_vault))
        }
    }   
 
    /// Returns a collection of edges that matches a vector u8 attribute vertex by key
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_vec_u8_key(
        &mut self,
        attr_k: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, GruPHstError> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let edges = edges
                .iter()
                .filter(|edge| edge.has_vertex_with_attr_vec_u8_key(attr_k))
                .collect::<Vec<&Edge>>();
            if !edges.is_empty() {
                Ok(edges)
            } else {
                warn!("Any edge found for attribute: {}", attr_k);
                Err(GruPHstError::EdgeNotFound)
            }
        } else {
            warn!("Vault {} does not exists", current_vault);
            Err(GruPHstError::VaultNotExists(current_vault))
        }
    }

    /// Returns a collection of edges like vector u8 attribute vertex key
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_vec_u8_key_like(
        &mut self,
        attr_k: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, GruPHstError> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let vrtcs = edges
                .iter()
                .filter(|edge| edge.has_vertex_with_attr_vec_u8_key_like(attr_k))
                .collect::<Vec<&Edge>>();
            if !vrtcs.is_empty() {
                Ok(vrtcs)
            } else {
                warn!("Any edge found for attribute: {}", attr_k);
                Err(GruPHstError::EdgeNotFound)
            }
        } else {
            warn!("Vault {} does not exists", current_vault);
            Err(GruPHstError::VaultNotExists(current_vault))
        }
    }

    /// Returns a collection of edges where vector u8 attribute value is equals to
    /// for some provided vault_name or default when None
    pub fn find_edges_with_vertex_attr_vec_u8_equals_to(
        &mut self,
        attr_k: &str,
        attr_v: &Vec<u8>,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Edge>, GruPHstError> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            let vrtcs = edges
                .iter()
                .filter(|edge| edge.has_vertex_with_attr_vec_u8_value_equals_to(attr_k, attr_v))
                .collect::<Vec<&Edge>>();
            if !vrtcs.is_empty() {
                Ok(vrtcs)
            } else {
                warn!("Any edge found for attribute: {}", attr_k);
                Err(GruPHstError::EdgeNotFound)
            }
        } else {
            warn!("Vault {} does not exists", current_vault);
            Err(GruPHstError::VaultNotExists(current_vault))
        }
    }


    /// Returns an Edge that provided id matches with Edge Id, or From, To vertices
    /// for some provided vault_name or default when None
    pub fn find_edge_by_id(
        &mut self,
        id: &str,
        vault_name: Option<&str>,
    ) -> Result<&mut Edge, GruPHstError> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get_mut(&current_vault) {
            if let Some(edge) = edges.iter_mut().find(|edge| {
                edge.get_id() == id
                    || edge.get_from_vertex().get_id() == id
                    || edge.get_to_vertex().get_id() == id
            }) {
                Ok(edge)
            } else {
                warn!("edge with id [{}] not found", id);
                Err(GruPHstError::EdgeNotFound)
            }
        } else {
            warn!("Vault {} does not exists", current_vault);
            Err(GruPHstError::VaultNotExists(current_vault))
        }
    }

    /// Find edge by id on any graphs' vault
    pub fn find_edge_by_id_in_graphs(&mut self, id: &str) -> Result<&mut Edge, GruPHstError> {
        for (_vault_name, edges) in self.vault.iter_mut() {
            if let Some(edge) = edges.iter_mut().find(|vrtx| {
                vrtx.get_id() == id
                    || vrtx.get_from_vertex().get_id() == id
                    || vrtx.get_to_vertex().get_id() == id
            }) {
                return Ok(edge);
            }
        }
        warn!("edge with id {} not found in graphs", id);
        Err(GruPHstError::EdgeNotFound)
    }
}

