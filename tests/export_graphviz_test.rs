use gruphst::{edge::Edge, exporter_importer::graphviz::export_to_graphviz_format};
use gruphst::graphs::Graphs;
use gruphst::vertex::Vertex;
use std::fs::{read_to_string, File};
use std::vec;

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
    
    let mut vector = Vec::new();
    vector.push(String::from("digraph {"));
    vector.push(String::from("gandalf [label=\"gandalf\" tooltip=\"name: Gandalf | known as: Gandalf the Gray\"];"));
    vector.push(String::from("gandalf [label=\"gandalf\" tooltip=\"known as: Gandalf the Gray | name: Gandalf\"];"));
    vector.push(String::from("saruman [label=\"saruman\" tooltip=\"known as: Saruman of Many Colours\"];"));
    vector.push(String::from("frodo [label=\"frodo\" tooltip=\"name: Frodo Bolson\"];"));
    vector.push(String::from("sam [label=\"sam\" tooltip=\"surname: Gamgee\"];"));
    vector.push(String::from("sauron [label=\"sauron\" tooltip=\"identified as: Necromancer\"];"));
    vector.push(String::from("gandalf -> frodo [label=\"friend of\"];"));
    vector.push(String::from("frodo -> gandalf [label=\"friend of\"];"));
    vector.push(String::from("sam -> frodo [label=\"best friend of\"];"));
    vector.push(String::from("saruman -> sauron [label=\"ally of\"];"));
    vector.push(String::from("sauron -> saruman [label=\"lord of\"];"));
    vector.push(String::from("gandalf -> sauron [label=\"enemy of\"];"));
    vector.push(String::from("gandalf -> saruman [label=\"enemy of\"];"));
    vector.push(String::from("sauron -> frodo [label=\"wants to catch\"];"));
    vector.push(String::from("}"));
    
    let lines= read_to_string(gv_file_path).unwrap(); 
    let mut count = 0;
    for line in vector.iter() {
        if count == 1 || count == 2 {
            assert_eq!(lines.contains(&vector[1]) || lines.contains(&vector[2]), true);
        } else {
            assert!(lines.contains(line)); 
        }
        count += 1;
    } 
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
