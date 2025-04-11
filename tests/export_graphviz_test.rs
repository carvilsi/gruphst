use gruphst::{edge::Edge, exporter_importer::graphviz::export_to_graphviz_format};
use gruphst::graphs::Graphs;
use gruphst::vertex::Vertex;
use std::fs::{read_to_string, File};

fn prepare_export_graphviz_test() -> Graphs {
    let mut gru = Graphs::init("shire-friendships");
    
    let mut gandalf = Vertex::new("gandalf");
    gandalf.set_attr("name", "Gandalf");
    gandalf.set_attr("known as", "Gandalf the Gray");

    let mut frodo = Vertex::new("frodo");
    frodo.set_attr("name", "Frodo Bolson");

    let edge = Edge::create(&gandalf, "friend of", &frodo);

    gru.add_edge(&edge, None);
    gru.add_edge(&Edge::create(&frodo, "friend of", &gandalf), None);

    let mut sam_v = Vertex::new("sam");
    sam_v.set_attr("surname", "Gamgee");
    
    let edge2 = Edge::create(&sam_v, "best friend of", &frodo);
    gru.add_edge(&edge2, None);

    gru.insert("middle-earth-enemies");
    let mut saruman = Vertex::new("saruman");
    saruman.set_attr("known as", "Saruman of Many Colours");
    let mut sauron = Vertex::new("sauron");
    sauron.set_attr("identified as", "Necromancer");

    let edge3 = Edge::create(&saruman, "ally of", &sauron);
    let edge4 = Edge::create(&sauron, "lord of", &saruman);

    gru.add_edges(&mut vec!(edge3, edge4), None);

    gru.add_edge(&Edge::create(&gandalf, "enemy of", &sauron), None);
    gru.add_edge(&Edge::create(&gandalf, "enemy of", &saruman), None);
    gru.add_edge(&Edge::create(&sauron, "wants to catch", &frodo), None);

    gru
}

fn assertion_exported_graphviz_file(gv_file_path: &str) {
    let exported_file = File::open(gv_file_path).unwrap();
    assert!(exported_file.metadata().unwrap().len() != 0);
    
    // let row1 = String::from("digraph {");
    // let row2 = String::from("gandalf [label=\"gandalf\" tooltip=\"name: Gandalf | known as: Gandalf the Gray\"];");
    // let row3 = String::from("saruman [label=\"saruman\" tooltip=\"known as: Saruman of Many Colours\"];");
    // let row4 = String::from("frodo [label=\"frodo\" tooltip=\"name: Frodo Bolson\"];");
    // let row5 = String::from("sam [label=\"sam\" tooltip=\"surname: Gamgee\"];");
    // let row6 = String::from("sauron [label=\"sauron\" tooltip=\"identified as: Necromancer\"];");
    // let row7 = String::from("gandalf -> frodo [label=\"friend of\"];");
    // let row8 = String::from("frodo -> gandalf [label=\"friend of\"];");
    // let row9 = String::from("sam -> frodo [label=\"best friend of\"];");
    // let row10 = String::from("saruman -> sauron [label=\"ally of\"];");
    // let row11 = String::from("sauron -> saruman [label=\"lord of\"];");
    // let row12 = String::from("gandalf -> sauron [label=\"enemy of\"];");
    // let row13 = String::from("gandalf -> saruman [label=\"enemy of\"];");
    // let row14 = String::from("sauron -> frodo [label=\"wants to catch\"];");
    // let row15 = String::from("}");
    
    // let binding = read_to_string(gv_file_path).unwrap(); 
    // let mut gv_lines = binding.lines(); 

    
    // assert_eq!(gv_lines.next().unwrap(), &row1);
    // assert_eq!(gv_lines.next().unwrap(), &row2);
    // assert_eq!(gv_lines.next().unwrap(), &row3);
    // assert_eq!(gv_lines.next().unwrap(), &row4);
    // assert_eq!(gv_lines.next().unwrap(), &row5);
    // assert_eq!(gv_lines.next().unwrap(), &row6);
    // assert_eq!(gv_lines.next().unwrap(), &row7);
    // assert_eq!(gv_lines.next().unwrap(), &row8);
    // assert_eq!(gv_lines.next().unwrap(), &row9);
    // assert_eq!(gv_lines.next().unwrap(), &row10);
    // assert_eq!(gv_lines.next().unwrap(), &row11);
    // assert_eq!(gv_lines.next().unwrap(), &row12);
    // assert_eq!(gv_lines.next().unwrap(), &row13);
    // assert_eq!(gv_lines.next().unwrap(), &row14);
    // assert_eq!(gv_lines.next().unwrap(), &row15);
}

#[test]
fn should_export_to_graphviz_custom_file_name_and_path() {
    let gru = prepare_export_graphviz_test();
    
    export_to_graphviz_format(&gru, Some("./tests/data/"), Some("export_custom_graphviz")).unwrap();
    
    let graphviz_file_path = "./tests/data/export_custom_graphviz.gv.txt";
    
    assertion_exported_graphviz_file(graphviz_file_path);
}

#[test]
fn should_export_to_graphviz_default_file_name_and_path() {
    let gru = prepare_export_graphviz_test();
    
    export_to_graphviz_format(&gru, Some("./tests/data/"), None).unwrap();
    
    let gv_file_path = "./tests/data/middle-earth-enemies.gv.txt";
    assertion_exported_graphviz_file(gv_file_path);
}

#[test]
fn should_export_to_graphviz_default_file_name_and_default_path() {
    let gru = prepare_export_graphviz_test();
    
    export_to_graphviz_format(&gru, None, None).unwrap();
    
    let csv_file_path = "middle-earth-enemies.gv.txt";
    assertion_exported_graphviz_file(csv_file_path);
}

#[test]
fn should_fail_export_to_graphviz_on_non_existent_path() {
    let gru = prepare_export_graphviz_test();

    assert!(export_to_graphviz_format(&gru, Some("/foobar"), None).is_err());
}

#[test]
fn should_fail_export_to_graphviz_on_empty_graph() {
    let gru = Graphs::init("empty");
    let e = export_to_graphviz_format(&gru, Some("./tests/data/"), None);
    assert!(e.is_err());
}
