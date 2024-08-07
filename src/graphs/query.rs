use log::error;

use crate::edge::Edge;
use crate::graphs::Graphs;
use crate::vertex::Vertex;

impl Graphs {
    /// Returns a collection of Vertices that matches the relation
    /// for provided vault or default when None
    pub fn find_vertices_by_relation(
        &mut self,
        relation_name: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(vertices) = self.vault.get(&current_vault) {
            let vrtcs = vertices
                .iter()
                .filter(|grph| grph.get_relation() == relation_name)
                .collect::<Vec<&Vertex>>();
            if !vrtcs.is_empty() {
                Ok(vrtcs)
            } else {
                error!("Any vertex found for relation: {}", relation_name);
                Err("Any vertex found for relation")
            }
        } else {
            Err("provided vault does not exists")
        }
    }

    /// Returns a collection of Vertices elements that matches the relations
    /// in the array
    /// for provided vault or default when None
    pub fn find_vertices_by_relations(
        &mut self,
        relations: Vec<&str>,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(vertices) = self.vault.get(&current_vault) {
            let vertex = vertices
                .iter()
                .filter(|grph| relations.contains(&grph.get_relation().as_str()))
                .collect::<Vec<&Vertex>>();
            if !vertex.is_empty() {
                Ok(vertex)
            } else {
                error!("Any vertex found for relations: {:#?}", relations);
                Err("Any vertex found for relation")
            }
        } else {
            Err("provided vault does not exists")
        }
    }

    /// Returns a collection of vertices that matches an attribute edge by key
    pub fn has_edge_attr_on_vertices(
        &mut self,
        attr_k: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(vertices) = self.vault.get(&current_vault) {
            let vrtcs = vertices
                .iter()
                .filter(|grph| grph.has_edge_with_attr_key(attr_k))
                .collect::<Vec<&Vertex>>();
            if !vrtcs.is_empty() {
                Ok(vrtcs)
            } else {
                error!("Any vertex found for attribute: {}", attr_k);
                Err("Any vertex found for attribute")
            }
        } else {
            Err("provided vault does not exists")
        }
    }

    /// Returns a collection of graphs like an attribute edge by key
    pub fn like_graph_edge_attr(
        &mut self,
        attr_k: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(vertices) = self.vault.get(&current_vault) {
            let vrtcs = vertices
                .iter()
                .filter(|grph| grph.has_edge_with_attr_key_like(attr_k))
                .collect::<Vec<&Vertex>>();
            if !vrtcs.is_empty() {
                Ok(vrtcs)
            } else {
                error!("Any vertex found for attribute: {}", attr_k);
                Err("Any vertex found for attribute")
            }
        } else {
            Err("provided vault does not exists")
        }
    }

    /// Returns a collection of vertices that matches an attribute
    /// and value
    // XXX: add a method to find attr on all graphs????
    pub fn attr_equals_to<T>(
        &self,
        attr_k: &str,
        attr_v: T,
        vault_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str>
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(vertices) = self.vault.get(&current_vault) {
            let vrtcs = vertices
                .iter()
                .filter(|grph| grph.has_edge_with_attr_value_equal(attr_k, attr_v.clone()))
                .collect::<Vec<&Vertex>>();
            if !vrtcs.is_empty() {
                Ok(vrtcs)
            } else {
                error!("Any vertex found for attribute: {}", attr_k);
                Err("Any vertex found for attribute")
            }
        } else {
            Err("provided vault does not exists")
        }
    }

    /// Returns a Vertex that provided id matches with Vertex, or From, To edges
    pub fn find_by_id(
        &mut self,
        id: &str,
        vault_name: Option<&str>,
    ) -> Result<&mut Vertex, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(vertices) = self.vault.get_mut(&current_vault) {
            if let Some(vertex) = vertices.iter_mut().find(|graph| {
                graph.get_id() == id
                    || graph.get_from_edge().get_id() == id
                    || graph.get_to_edge().get_id() == id
            }) {
                Ok(vertex)
            } else {
                error!("Vertex with id [{}] not found", id);
                Err("Vertex not found")
            }
        } else {
            Err("provided vault does not exists")
        }
    }

    /// Find in any graph on vault by id
    pub fn find_by_id_in_graphs(&mut self, id: &str) -> Result<&mut Vertex, &'static str> {
        for (_vault_name, vertices) in self.vault.iter_mut() {
            if let Some(vertex) = vertices.iter_mut().find(|vrtx| {
                vrtx.get_id() == id
                    || vrtx.get_from_edge().get_id() == id
                    || vrtx.get_to_edge().get_id() == id
            }) {
                return Ok(vertex);
            }
        }
        Err("Vertex not found")
    }

    /// Retrieves all the edges with incoming relation
    pub fn has_relation_in(
        &self,
        relation_in: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<Edge>, &'static str> {
        let mut relations_in: Vec<Edge> = Vec::new();
        let current_vault = self.select_vault_label(vault_name);
        if let Some(vertices) = self.vault.get(&current_vault) {
            for vertex in vertices {
                if vertex.get_relation() == relation_in
                    && !relations_in.contains(&vertex.get_to_edge())
                {
                    relations_in.push(vertex.get_to_edge().clone());
                }
            }
        } else {
            return Err("provided vault does not exists");
        }
        if !relations_in.is_empty() {
            Ok(relations_in)
        } else {
            Err("any edge found with relation in")
        }
    }

    /// Retrieves all the edges with outcoming relation
    pub fn has_relation_out(
        &self,
        relation_out: &str,
        vault_name: Option<&str>,
    ) -> Result<Vec<Edge>, &'static str> {
        let mut relations_out: Vec<Edge> = Vec::new();
        let current_vault = self.select_vault_label(vault_name);
        if let Some(vertices) = self.vault.get(&current_vault) {
            for vertex in vertices {
                if vertex.get_relation() == relation_out
                    && !relations_out.contains(&vertex.get_from_edge())
                {
                    relations_out.push(vertex.get_from_edge().clone());
                }
            }
        } else {
            return Err("provided vault does not exists");
        }
        if !relations_out.is_empty() {
            Ok(relations_out)
        } else {
            Err("any edge found with relation out")
        }
    }
}
