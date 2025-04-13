use log::warn;

use crate::errors::GruPHstError;
use crate::graphs::Graphs;
use crate::vertex::Vertex;

impl Graphs {
    /// Returns a Vertex that provided id matches with id of From, To vertices
    /// for some provided vault_name or default when None
    pub fn find_vertex_by_id(
        &mut self,
        id: &str,
        vault_name: Option<&str>,
    ) -> Result<Vertex, GruPHstError> {
        let current_vault = self.select_vault_mut(vault_name)?;
        for edge in current_vault {
            if let Ok(vertex) = edge.find_vertex_by_id(id) {
                return Ok(vertex);
            }
        }
        warn!("Vertex with id: {} not found", id);
        Err(GruPHstError::VertexNotFound)
    }

    /// Returns a Vertex that provided id matches with id of From, To vertices
    /// on any graphs' vault
    pub fn find_vertex_by_id_in_graphs(&mut self, id: &str) -> Result<Vertex, GruPHstError> {
        for (vault_name, _edges) in self.vault.clone() {
            if let Ok(vertex) = self.find_vertex_by_id(id, Some(vault_name.as_str())) {
                return Ok(vertex);
            }
        }
        warn!("Vertex with id: {} not found in graphs", id);
        Err(GruPHstError::VertexNotFound)
    }

    /// Retrieves all the vertices with incoming relation
    /// for some provided vault_name or default when None
    pub fn find_vertices_with_relation_in(
        &self,
        relation_in: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<Vertex>, GruPHstError> {
        let mut relations_in: Vec<Vertex> = Vec::new();
        let current_vault = self.select_vault(vault_name)?;
        for edge in current_vault {
            if edge.get_relation() == relation_in && !relations_in.contains(&edge.get_to_vertex()) {
                relations_in.push(edge.get_to_vertex().clone());
            }
        }
        if !relations_in.is_empty() {
            Ok(relations_in)
        } else {
            warn!("Vertex with relation in: {} not found", relation_in);
            Err(GruPHstError::VertexNotFound)
        }
    }

    /// Retrieves all the vertices with outcoming relation
    /// for some provided vault_name or default when None
    pub fn find_vertices_with_relation_out(
        &self,
        relation_out: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<Vertex>, GruPHstError> {
        let mut relations_out: Vec<Vertex> = Vec::new();
        let current_vault = self.select_vault(vault_name)?;
        for edge in current_vault {
            if edge.get_relation() == relation_out
                && !relations_out.contains(&edge.get_from_vertex())
            {
                relations_out.push(edge.get_from_vertex().clone());
            }
        }
        if !relations_out.is_empty() {
            Ok(relations_out)
        } else {
            warn!("Vertex with relation out: {} not found", relation_out);
            Err(GruPHstError::VertexNotFound)
        }
    }
}
