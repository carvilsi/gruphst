use std::error::Error;

use crate::{edge::Edge, errors::GruPHstError, graphs::Graphs, vertex::Vertex};

use super::CSVRow;

pub(super) fn collect_graphs_csv_rows(graphs: &Graphs) -> Result<Vec<CSVRow>, Box<dyn Error>> {
    let mut csv_rows: Vec<CSVRow> = Vec::new();
    let vaults = graphs.get_vaults()?;
    for (vault_name, edges) in vaults {
        let _ = collect_graphs_csv_rows_values(&mut csv_rows, &edges, &vault_name)?;
    }
    Ok(csv_rows)
}

pub(super) fn process_vertex_attributes(vertex: &mut Vertex, attrs_str: &str) {
    if attrs_str.contains("|") {
        let raw_attrs_vec: Vec<&str> = attrs_str.split('|').collect();
        for attr_str in raw_attrs_vec.iter() {
            fill_vertex_attributes(vertex, attr_str);
        }
    } else {
        fill_vertex_attributes(vertex, attrs_str);
    }
}

pub(super) fn generate_graphs_from_csv(
    graphs_name: &str,
    csv_rows: &Vec<CSVRow>,
) -> Result<Graphs, GruPHstError> {
    let mut graphs = Graphs::init(graphs_name);
    create_vaults_from_csv(&mut graphs, csv_rows);
    for csv_row in csv_rows.iter() {
        let (vertex_from, vertex_to) = &csv_row.generate_vertices();
        let edge = Edge::create(vertex_from, &csv_row.relation, vertex_to);
        graphs.add_edge(&edge, Some(&csv_row.graphs_vault));
    }
    if graphs.get_vaults() == Err(GruPHstError::NoVaultOnGraphs) {
        return Err(GruPHstError::CSVEmpty);
    }
    Ok(graphs)
}

fn collect_graphs_csv_rows_values<'a>(
    csv_rows: &'a mut Vec<CSVRow>,
    edges: &'a [Edge],
    vault_name: &str,
) -> Result<&'a mut Vec<CSVRow>, Box<dyn Error>> {
    for edge in edges.iter() {
        csv_rows.push(CSVRow::collect_edge_csv_row_values(edge, vault_name)?);
    }
    Ok(csv_rows)
}

fn fill_vertex_attributes(vertex: &mut Vertex, attr_str: &str) {
    let attr: Vec<&str> = attr_str.split(':').collect();
    vertex.set_attr(attr.first().unwrap().trim(), attr.get(1).unwrap().trim());
}

fn create_vaults_from_csv(graphs: &mut Graphs, csv_rows: &[CSVRow]) {
    for csv_row in csv_rows.iter() {
        graphs.insert(&csv_row.graphs_vault);
    }
}
