use gruphst::edge::Edge;
use gruphst::graphs::Graphs;
use gruphst::vertex::Vertex;
use gruphst::exporter_importer::csv::*;
use std::fs::{read_to_string, File};

fn prepare_export_import_test() -> (Graphs, Edge, Edge) {
    let mut gru = Graphs::init("graphs-a");
    
    let mut vertex1 = Vertex::new("gandalf");
    vertex1.set_attr("name", "Gandalf");
    vertex1.set_attr("known as", "Gandalf the Gray");

    let mut vertex2 = Vertex::new("frodo");
    vertex2.set_attr("name", "Frodo Bolson");

    let edge = Edge::create(&vertex1, "friend of", &vertex2);

    gru.add_edge(&edge, None);

    let mut vertex3 = Vertex::new("sam");
    vertex3.set_attr("surname", "Gamgee");
    
    let edge2 = Edge::create(&vertex3, "best firend of", &vertex2);
    gru.add_edge(&edge2, None);

    (gru, edge, edge2)
}

fn assertion_persisted_graphs(grphs: Graphs, name: String, edge1: Edge, edge2: Edge) {
    let edges = grphs.get_edges(Some(name.as_str())).unwrap();
    assert_eq!(grphs.get_label(), name);
    assert_eq!(edges[0].get_relation(), edge1.get_relation());
    assert_eq!(edges[0].get_from_vertex().get_label(), "a edge");
    assert_eq!(edges[0].get_from_vertex().attrs_len(), 1);
    assert_eq!(edges[0].get_from_vertex().get_attr("foo").unwrap(), "bar");
    assert_eq!(edges[1], edge2);
}

#[test]
fn should_export_to_csv_custom_file_name_and_path() {
    let (gru, _edge1, _edge2) = prepare_export_import_test();
    
    export_to_csv_gruphst_format(&gru, Some("./tests/data/"), Some("export_custom")).unwrap();
    
    let csv_file_path = "./tests/data/export_custom.csv";
    let exported_file = File::open(csv_file_path).unwrap();
    assert!(exported_file.metadata().unwrap().len() != 0);
    
    let row1 = String::from("from_label;from_attributes;relation;to_label;to_attributes");
    let row3 = String::from("sam;surname: Gamgee;best firend of;frodo;name: Frodo Bolson");
    
    let binding = read_to_string(csv_file_path).unwrap(); 
    let mut csv_lines = binding.lines(); 
    
    assert_eq!(csv_lines.next().unwrap(), &row1);
    let second_row = csv_lines.next().unwrap();
    assert!(second_row.contains("|"));
    assert!(second_row.contains("name: Gandalf"));
    assert!(second_row.contains("gandalf;"));
    assert!(second_row.contains("known as: Gandalf the Gray"));
    assert!(second_row.contains(";friend of;frodo;name: Frodo Bolson"));
    assert_eq!(csv_lines.next().unwrap(), &row3);
    
}
