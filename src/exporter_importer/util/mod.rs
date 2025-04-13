use crate::{graphs::Graphs, vertex::Vertex};
use std::error::Error;

const CSV_EXTENSION: &str = "csv";
const GRAPHVIZ_EXTENSION: &str = "gv.txt";

pub(super) enum ExportFileFormat {
    CSV,
    GraphViz,
}

pub(super) fn collect_attributes_str(vertex: &Vertex) -> Result<String, Box<dyn Error>> {
    let attr_str_keys = vertex.get_attr_keys();
    let mut res = String::new();
    let mut cntr = 0;
    for attr_k in attr_str_keys.iter() {
        cntr += 1;
        res.push_str(attr_k);
        res.push_str(": ");
        res.push_str(&vertex.get_attr(attr_k)?);
        if cntr != attr_str_keys.len() {
            res.push_str(" | ");
        }
    }
    Ok(res)
}

pub(super) fn get_filename(
    graphs: &Graphs,
    flnm: Option<&str>,
    flpth: Option<&str>,
    format: ExportFileFormat,
) -> String {
    let mut export_filename: String = graphs.get_label();
    if let Some(csvflnm) = flnm {
        export_filename = csvflnm.to_string();
    }
    if let Some(cvsfpth) = flpth {
        export_filename = format!("{}/{}", cvsfpth, export_filename);
    }
    let extension = match format {
        ExportFileFormat::CSV => CSV_EXTENSION,
        ExportFileFormat::GraphViz => GRAPHVIZ_EXTENSION,
    };
    let filename = format!("{}.{}", export_filename, extension);
    filename
}
