use std::error::Error;
use std::io::Write;

use crate::graphs::Graphs;

use super::generic::collect_attributes_str;

fn generate_graphviz_header() -> String {
    String::from("digraph {")
}

fn generate_graphviz_footer() -> String {
    String::from("}")
}

pub fn export_to_graphviz_format(
  graphs: &Graphs,
  gv_file_path: Option<&str>,
  gv_filename: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let mut export_gv_filename: String = graphs.get_label();
    if let Some(gvflnm) = gv_filename {
        export_gv_filename = gvflnm.to_string();
    }
    if let Some(gvfpth) = gv_file_path {
        export_gv_filename = format!("{}/{}", gvfpth, export_gv_filename);
    } 
    let filename = format!("{}.gv.txt", export_gv_filename);

    let mut file = std::fs::File::create(&filename)?;

    let header = generate_graphviz_header();
    writeln!(file, "{}", header)?;
    
    let vertives = graphs.get_uniq_vertices_on_graphs()?;
    for vertex in vertives {
        let label = vertex.get_label();
        let tooltip = collect_attributes_str(&vertex)?;
        writeln!(file, "\t{} [label=\"{}\" tooltip=\"{}\"];", label, label, tooltip)?;
    }

    let vaults = graphs.get_vaults()?;
    for (_vault_name, edges) in vaults {
      for edge in edges {
        let from_vertex = edge.get_from_vertex();
        let to_vertex = edge.get_to_vertex();
        let relation = edge.get_relation();
        writeln!(
            file,
            "\t{} -> {} [label=\"{}\"];",
            from_vertex.get_label(),
            to_vertex.get_label(),
            relation
        )?;
      }
    }

    writeln!(file, "{}", generate_graphviz_footer())?;
    Ok(())

}