use std::error::Error;

use serde::{Deserialize, Serialize};
use crate::{config::get_csv_delimiter, edge::Edge, errors::GruPHstError, graphs::Graphs, util::get_file_name_from_path, vertex::Vertex};

/// Structure of CSV file
/// grpahs; from_label; from_attributes; relation; to_label; to_attributes
/// shire-friends; gandalf; name: Gandalf | known as: Gandalf the Gray; friend of; Frodo; surname: Bolson
/// shire-friends; gandalf; name: Gandalf | known as: Gandalf the Gray; enemy of; Saruman; known as: The White
/// middle-earth-enemies; saruman; name: Saruman | former: Saruman the White; ally of; sauron; activity: necromancer

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

fn collect_attributes_str(vertex: &Vertex) -> Result<String, Box<dyn Error>> {
    let attr_str_keys = vertex.get_attr_keys();
    let mut res = String::new();
    let mut cntr = 0;
    for attr_k in attr_str_keys.iter() {
        cntr += 1;
        res.push_str(&attr_k);
        res.push_str(": ");
        res.push_str(&vertex.get_attr(attr_k)?);
        if cntr != attr_str_keys.len() {
            res.push_str(" | ");
        }
    }
    Ok(res)
}

fn collect_edge_csv_row_values(edge: &Edge, vault_name: &str) -> Result<CSVRow, Box<dyn Error>> {
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

fn collect_graphs_csv_rows_values<'a>(
    csv_rows: &'a mut Vec<CSVRow>,
    edges:  &'a Vec<Edge>,
    vault_name: &str,
) -> Result<&'a mut Vec<CSVRow>, Box<dyn Error>> {
    for edge in edges.iter() {
        csv_rows.push(collect_edge_csv_row_values(edge, vault_name)?);
    }
    Ok(csv_rows)
}

fn collect_graphs_csv_rows(
    graphs: &Graphs
) -> Result<Vec<CSVRow>, Box<dyn Error>> {
    let mut csv_rows: Vec<CSVRow> = Vec::new();
    let vaults = graphs.get_vaults()?;
    for (vault_name, edges) in vaults {
        let _ = collect_graphs_csv_rows_values(&mut csv_rows, &edges, &vault_name)?;
    }
    Ok(csv_rows)
}

/// Exports Graphs to csv format
/// with semicolon ';' as default delimiter,
/// for custom delimiter character check config file
/// variable GRUPHST_CSV_DELIMITER
// FIXME: add the rest of the graphs in the vault
pub fn export_to_csv_gruphst_format(
    graphs: &Graphs,
    csv_file_path: Option<&str>,
    csv_filename: Option<&str>
) -> Result<(), Box<dyn Error>> {
    let csv_delimiter = get_csv_delimiter();
    let mut export_csv_filename: String = graphs.get_label();
    if let Some(csvflnm) = csv_filename {
        export_csv_filename = csvflnm.to_string();
    }
    if let Some(cvsfpth) = csv_file_path {
        export_csv_filename = format!("{}/{}", cvsfpth, export_csv_filename);
    } 
    let filename = format!("{}.csv", export_csv_filename);
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(csv_delimiter)
        .from_path(filename.as_str())?;
    let csv_rows: Vec<CSVRow> = collect_graphs_csv_rows(graphs)?;
    for csv_row in csv_rows {
        wtr.serialize(csv_row)?;
    }
    wtr.flush()?;
    Ok(())
}

fn fill_vertex_attributes(mut vertex: Vertex, attr_str: &str) {
    let attr: Vec<&str> = attr_str.split(':').collect();
    vertex.set_attr(attr.get(0).unwrap().trim(), attr.get(1).unwrap().trim());
}

fn process_vertex_attributes(vertex: Vertex, attrs_str: &str) {
    if attrs_str.contains("|") {
        let raw_attrs_vec: Vec<&str> = attrs_str.split('|').collect();
        for attr_str in raw_attrs_vec {
            fill_vertex_attributes(vertex.clone(), &attr_str);
        }
    } else {
        fill_vertex_attributes(vertex, attrs_str);
    }
}

fn generate_vertices(csv_row: &CSVRow) -> (Vertex, Vertex) {
    let vertex_from = Vertex::new(&csv_row.from_label);
    if !csv_row.from_attributes.is_empty() {
        process_vertex_attributes(vertex_from.clone(), &csv_row.from_attributes);
    }
    let vertex_to = Vertex::new(&csv_row.to_label);
    if !csv_row.to_attributes.is_empty() {
        process_vertex_attributes(vertex_to.clone(), &csv_row.to_attributes);
    }
    (vertex_from, vertex_to)
}

fn create_vaults_from_csv(graphs: &mut Graphs, csv_rows: &Vec<CSVRow>) {
    for csv_row in csv_rows {
        graphs.insert(&csv_row.graphs_vault);
    }
}

fn generate_graphs_from_csv(graphs_name: &str, csv_rows: &Vec<CSVRow>) -> Result<Graphs, GruPHstError> {
    let mut graphs = Graphs::init(graphs_name);
    create_vaults_from_csv(&mut graphs, csv_rows);
    for csv_row in csv_rows {
        let (vertex_from, vertex_to) = generate_vertices(&csv_row);
        let edge = Edge::create(&vertex_from, &csv_row.relation, &vertex_to);
        graphs.add_edge(&edge, Some(&csv_row.graphs_vault));
    }
    if graphs.get_vaults() == Err(GruPHstError::NoVaultOnGraphs) {
        return Err(GruPHstError::CSVEmpty);
    } 
    Ok(graphs)
}

pub fn import_from_csv_gruphst_format(csv_file_path: &str) -> Result<Graphs, Box<dyn Error>> {
    let csv_delimiter = get_csv_delimiter();
    let graph_name = get_file_name_from_path(csv_file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(csv_delimiter)
        .from_path(csv_file_path)?;
    let mut csv_rows: Vec<CSVRow> = Vec::new();
    for row in rdr.deserialize() {
        let csv_row: CSVRow = row?;
        csv_rows.push(csv_row);
    }
    Ok(generate_graphs_from_csv(&graph_name, &csv_rows)?)
}