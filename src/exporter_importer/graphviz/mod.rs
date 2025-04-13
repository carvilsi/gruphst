use std::error::Error;
use std::io::Write;

use crate::graphs::Graphs;

use super::util::{collect_attributes_str, get_filename, ExportFileFormat};

fn generate_graphviz_header() -> String {
    String::from("digraph {")
}

fn generate_graphviz_footer() -> String {
    String::from("}")
}

/// Export the Graphs to a Graphviz format
///
/// #Examples
/// ```rust
/// use gruphst::edge::Edge;
/// use gruphst::vertex::Vertex;
/// use gruphst::graphs::Graphs;
/// use gruphst::exporter_importer::graphviz::export_to_graphviz_format;
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
/// export_to_graphviz_format(
///     &gru,
///     Some("./"),
///     Some("export_graphviz_filename")
/// ).unwrap();
/// ```
pub fn export_to_graphviz_format(
    graphs: &Graphs,
    gv_file_path: Option<&str>,
    gv_filename: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let filename = get_filename(
        graphs,
        gv_filename,
        gv_file_path,
        ExportFileFormat::GraphViz,
    );

    let mut file = std::fs::File::create(&filename)?;

    let header = generate_graphviz_header();
    writeln!(file, "{}", header)?;

    let vertives = graphs.get_uniq_vertices_on_graphs()?;
    for vertex in vertives {
        let label = vertex.get_label();
        let tooltip = collect_attributes_str(&vertex)?;
        writeln!(
            file,
            "\t{} [label=\"{}\" tooltip=\"{}\"];",
            label, label, tooltip
        )?;
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
