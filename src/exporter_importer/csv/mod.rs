//! CSV export/import module
//!
//! Structure of CSV file:
//! ```csv
//! grpahs; from_label; from_attributes; relation; to_label; to_attributes
//! shire-friends; gandalf; name: Gandalf | known as: Gandalf the Gray; friend of; Frodo; surname: Bolson
//! shire-friends; gandalf; name: Gandalf | known as: Gandalf the Gray; enemy of; Saruman; known as: The White
//! middle-earth-enemies; saruman; name: Saruman | former: Saruman the White; ally of; sauron; activity: necromancer
//! ```

use std::error::Error;

use crate::{
    config::get_csv_delimiter, edge::Edge, errors::GruPHstError, graphs::Graphs,
    util::get_file_name_from_path, vertex::Vertex,
};
use csv_handlers::{collect_graphs_csv_rows, generate_graphs_from_csv, process_vertex_attributes};
use serde::{Deserialize, Serialize};

use super::util::{collect_attributes_str, get_filename, ExportFileFormat};

mod csv_handlers;

// XXX: Comments; things to think and to implement
// - We need to support at least two different types of attributes, the Stringy ones and the Vec<u8>

#[derive(Debug, Serialize, Deserialize)]
pub struct CSVRow {
    graphs_vault: String,
    from_label: String,
    from_attributes: String,
    relation: String,
    to_label: String,
    to_attributes: String,
}

impl CSVRow {
    fn generate_vertices(&self) -> (Vertex, Vertex) {
        let mut vertex_from = Vertex::new(&self.from_label);
        if !self.from_attributes.is_empty() {
            process_vertex_attributes(&mut vertex_from, &self.from_attributes);
        }
        let mut vertex_to = Vertex::new(&self.to_label);
        if !self.to_attributes.is_empty() {
            process_vertex_attributes(&mut vertex_to, &self.to_attributes);
        }
        (vertex_from, vertex_to)
    }

    fn collect_edge_csv_row_values(edge: &Edge, vault_name: &str) -> Result<Self, Box<dyn Error>> {
        let mut vertex = edge.get_from_vertex();
        let from_label = vertex.get_label();
        let from_attributes = collect_attributes_str(&vertex)?;
        vertex = edge.get_to_vertex();
        let to_label = vertex.get_label();
        let to_attributes = collect_attributes_str(&vertex)?;
        let csv_row = CSVRow {
            graphs_vault: vault_name.to_string(),
            from_label,
            from_attributes,
            relation: edge.get_relation(),
            to_label,
            to_attributes,
        };
        Ok(csv_row)
    }
}

/// Exports Graphs to csv format
/// with semicolon ';' as default delimiter,
/// for custom delimiter character check config file
/// variable GRUPHST_CSV_DELIMITER
///
/// #Examples
/// ```rust
/// use gruphst::edge::Edge;
/// use gruphst::vertex::Vertex;
/// use gruphst::graphs::Graphs;
/// use gruphst::exporter_importer::csv::export_to_csv_gruphst_format;
///
/// let mut gru = Graphs::init("shire-friendships");
///
/// let mut gandalf_v = Vertex::new("gandalf");
/// gandalf_v.set_attr("name", "Gandalf");
/// gandalf_v.set_attr("known as", "Gandalf the Gray");
///
/// let mut frodo_v = Vertex::new("frodo");
/// frodo_v.set_attr("name", "Frodo Bolson");
///
/// let edge = Edge::create(&gandalf_v, "friend of", &frodo_v);
///
/// gru.add_edge(&edge, None);
///
/// export_to_csv_gruphst_format(
///     &gru,
///     Some("./"),
///     Some("export_csv_filename")
/// ).unwrap();
/// ```
pub fn export_to_csv_gruphst_format(
    graphs: &Graphs,
    csv_file_path: Option<&str>,
    csv_filename: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let csv_delimiter = get_csv_delimiter();
    let filename = get_filename(graphs, csv_filename, csv_file_path, ExportFileFormat::CSV);
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(csv_delimiter)
        .from_path(filename.as_str())?;
    let csv_rows: Vec<CSVRow> = collect_graphs_csv_rows(graphs)?;
    for csv_row in csv_rows.iter() {
        wtr.serialize(csv_row)?;
    }
    wtr.flush()?;
    Ok(())
}

/// Imports Graphs from csv format file
/// with semicolon ';' as default delimiter,
/// for custom delimiter character check config file
/// variable GRUPHST_CSV_DELIMITER
///
/// #Examples
/// ```rust
/// use gruphst::graphs::Graphs;
/// use gruphst::exporter_importer::csv::import_from_csv_gruphst_format;
///
/// let csv_file_path = "./tests/data/exported.csv";
/// let graphs: Graphs = import_from_csv_gruphst_format(csv_file_path).unwrap();
/// ```
pub fn import_from_csv_gruphst_format(csv_file_path: &str) -> Result<Graphs, Box<dyn Error>> {
    let csv_delimiter = get_csv_delimiter();
    let graph_name = get_file_name_from_path(csv_file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(csv_delimiter)
        .from_path(csv_file_path)?;
    let mut csv_rows: Vec<CSVRow> = Vec::new();
    for row in rdr.deserialize() {
        let csv_row: CSVRow = row?;
        if csv_row.relation.trim().is_empty() {
            return Err(Box::new(GruPHstError::CSVEdgeMissingRelation));
        }
        csv_rows.push(csv_row);
    }
    Ok(generate_graphs_from_csv(&graph_name, &csv_rows)?)
}
