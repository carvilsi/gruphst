use crate::{errors::GruPHstError, graphs::Graphs};

impl Graphs {
    /// Returns an array with the unique relations in the current graph
    /// or the one provided
    pub fn uniq_graph_relations(
        &self,
        graphs_name: Option<&str>,
    ) -> Result<Vec<String>, GruPHstError> {
        let mut uniq_rel = Vec::new();
        let current_vault = self.select_vault(graphs_name)?;
        for edge in current_vault.iter() {
            uniq_rel.push(edge.get_relation());
        }
        uniq_rel.sort();
        uniq_rel.dedup();
        Ok(uniq_rel)
    }

    /// Returns an array with the unique relations in the whole Graphs
    pub fn uniq_relations(&self) -> Vec<String> {
        let mut uniq_rel = Vec::new();
        for edges in self.vault.values() {
            for edge in edges.iter() {
                uniq_rel.push(edge.get_relation());
            }
            uniq_rel.sort();
            uniq_rel.dedup();
        }
        uniq_rel
    }

    /// Retrieves the length of the Graphs for whole vault
    pub fn len(&self) -> usize {
        let mut length = 0;
        for (_graphs_name, edges) in self.vault.iter() {
            length += edges.len();
        }
        length
    }

    /// Retrieves the length of vault
    pub fn len_graphs(&self) -> usize {
        self.vault.len()
    }

    /// Checks if the Graphs vault is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// function to retrieve memory usage by graphs
    pub fn get_mem(&self) -> Result<usize, &'static str> {
        let bytes = bincode::serialize(self).unwrap();
        Ok(bytes.len())
    }
}
