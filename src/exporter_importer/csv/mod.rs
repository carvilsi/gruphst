use std::error::Error;

use serde::{Deserialize, Serialize};
use crate::{config::get_csv_delimiter, graphs::{self, Graphs}};

/// Structure of CSV file
/// from_label; from_attributes; relation; to_label; to_attributes
/// gandalf; name: Gandalf | known as: Gandalf the Gray; friend of; Frodo; surname: Bolson
/// gandalf; name: Gandalf | known as: Gandalf the Gray; enemy of; Saruman; known as: The White

// XXX: Comments; things to think and to implement
// - If we want to import with specific Id we need to add an option to the init/new for Vertex/Edge
// - We need to support at least two different types of attributes, the Stringy ones and the Vec<u8>

#[derive(Debug, Serialize, Deserialize)]
pub struct CSVRow {
    from_label: String,
    from_attributes: String,
    relation: String,
    to_label: String,
    to_attributes: String,
}

pub fn export_to_csv_gruphst_format(
    graphs: Graphs,
    csv_filname: Option<&str>
) -> Result<(), Box<dyn Error>> {
    let csv_delimiter = get_csv_delimiter();
    let mut export_csv_filename: String = graphs.get_label();
    if let Some(csvflnm) = csv_filname {
        export_csv_filename = csvflnm.to_string();
    }
    let filename = format!("{}.csv", export_csv_filename);
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(csv_delimiter)
        .from_path(filename.as_str())?;

    Ok(())
}