use gruphst::edge::Edge;
use gruphst::exporter_importer::csv::*;
use gruphst::graphs::Graphs;
use gruphst::vertex::Vertex;
use std::fs::{read_to_string, File};

fn prepare_export_import_csv_test() -> Graphs {
    let mut gru = Graphs::init("shire-friendships");

    let mut gandalf_v = Vertex::new("gandalf");
    gandalf_v.set_attr("name", "Gandalf");
    gandalf_v.set_attr("known as", "Gandalf the Gray");

    let mut frodo_v = Vertex::new("frodo");
    frodo_v.set_attr("name", "Frodo Bolson");

    let edge = Edge::create(&gandalf_v, "friend of", &frodo_v);

    gru.add_edge(&edge, None);

    let mut sam_v = Vertex::new("sam");
    sam_v.set_attr("surname", "Gamgee");

    let edge2 = Edge::create(&sam_v, "best friend of", &frodo_v);
    gru.add_edge(&edge2, None);

    gru.insert("middle-earth-enemies");
    let mut saruman = Vertex::new("saruman");
    saruman.set_attr("known as", "Saruman of Many Colours");
    let mut sauron = Vertex::new("sauron");
    sauron.set_attr("identified as", "Necromancer");

    let edge3 = Edge::create(&saruman, "ally of", &sauron);
    let edge4 = Edge::create(&sauron, "lord of", &saruman);

    gru.add_edges(&mut vec![edge3, edge4], None);

    gru
}

fn assertion_gandalf_line(line: &str) {
    assert!(line.contains("|"));
    assert!(line.contains("name: Gandalf"));
    assert!(line.contains("gandalf;"));
    assert!(line.contains("known as: Gandalf the Gray"));
    assert!(line.contains(";friend of;frodo;name: Frodo Bolson"));
}

fn assertion_exported_csv_file(csv_file_path: &str) {
    let exported_file = File::open(csv_file_path).unwrap();
    assert!(exported_file.metadata().unwrap().len() != 0);

    let row1 =
        String::from("graphs_vault;from_label;from_attributes;relation;to_label;to_attributes");
    let row2 = String::from("middle-earth-enemies;saruman;known as: Saruman of Many Colours;ally of;sauron;identified as: Necromancer");
    let row3 = String::from("middle-earth-enemies;sauron;identified as: Necromancer;lord of;saruman;known as: Saruman of Many Colours");
    let row5 = String::from(
        "shire-friendships;sam;surname: Gamgee;best friend of;frodo;name: Frodo Bolson",
    );

    let binding = read_to_string(csv_file_path).unwrap();
    let mut csv_lines = binding.lines();

    assert_eq!(csv_lines.next().unwrap(), &row1);
    let line = csv_lines.next().unwrap();
    if line == &row2 {
        assert_eq!(csv_lines.next().unwrap(), &row3);
        let fourth_row = csv_lines.next().unwrap();
        assertion_gandalf_line(fourth_row);
        assert_eq!(csv_lines.next().unwrap(), &row5);
    } else {
        assertion_gandalf_line(line);
        assert_eq!(csv_lines.next().unwrap(), row5);
        assert_eq!(csv_lines.next().unwrap(), row2);
        assert_eq!(csv_lines.next().unwrap(), row3);
    }
}

#[test]
fn should_export_to_csv_custom_file_name_and_path() {
    let gru = prepare_export_import_csv_test();

    export_to_csv_gruphst_format(&gru, Some("./tests/data/"), Some("export_custom")).unwrap();

    let csv_file_path = "./tests/data/export_custom.csv";

    assertion_exported_csv_file(csv_file_path);
}

#[test]
fn should_export_to_csv_default_file_name_and_path() {
    let gru = prepare_export_import_csv_test();

    export_to_csv_gruphst_format(&gru, Some("./tests/data/"), None).unwrap();

    let csv_file_path = "./tests/data/middle-earth-enemies.csv";
    assertion_exported_csv_file(csv_file_path);
}

#[test]
fn should_export_to_csv_default_file_name_and_default_path() {
    let gru = prepare_export_import_csv_test();

    export_to_csv_gruphst_format(&gru, None, None).unwrap();

    let csv_file_path = "middle-earth-enemies.csv";
    assertion_exported_csv_file(csv_file_path);
}

#[test]
fn should_fail_export_to_csv_on_non_existent_path() {
    let gru = prepare_export_import_csv_test();

    assert!(export_to_csv_gruphst_format(&gru, Some("/foobar"), None).is_err());
}

#[test]
fn should_fail_export_to_csv_on_empty_graph() {
    let gru = Graphs::init("empty");
    let e = export_to_csv_gruphst_format(&gru, Some("./tests/data/"), None);
    assert!(e.is_err());
}

#[test]
fn should_import_from_csv_file() {
    let csv_file_path = "./tests/data/exported.csv";
    let graphs: Graphs = import_from_csv_gruphst_format(csv_file_path).unwrap();
    let mut edges: Vec<Edge> = graphs.get_edges(Some("shire-friendships")).unwrap();

    assert_eq!(edges[0].get_relation(), "friend of");
    assert_eq!(edges[0].get_from_vertex().get_label(), "gandalf");
    assert_eq!(
        edges[0].get_from_vertex().get_attr("known as").unwrap(),
        "Gandalf the Gray".to_string()
    );
    assert_eq!(
        edges[0].get_from_vertex().get_attr("name").unwrap(),
        "Gandalf".to_string()
    );
    assert_eq!(edges[0].get_to_vertex().get_label(), "frodo");

    assert_eq!(edges[1].get_relation(), "best friend of");
    assert_eq!(edges[1].get_from_vertex().get_label(), "sam");
    assert_eq!(
        edges[1].get_from_vertex().get_attr("surname").unwrap(),
        "Gamgee".to_string()
    );
    assert_eq!(edges[1].get_to_vertex().get_label(), "frodo");
    assert_eq!(
        edges[1].get_to_vertex().get_attr("name").unwrap(),
        "Frodo Bolson".to_string()
    );

    assert_eq!(edges[2].get_relation(), "friend of");
    assert_eq!(edges[2].get_to_vertex().get_label(), "galadriel");

    edges = graphs.get_edges(Some("middle-earth-enemies")).unwrap();
    assert_eq!(edges[0].get_label(), "ally of");
    assert_eq!(
        edges[0].get_from_vertex().get_label(),
        "saruman".to_string()
    );
    assert_eq!(edges[0].get_to_vertex().get_label(), "sauron".to_string());
    assert_eq!(
        edges[0].get_from_vertex().get_attr("known as").unwrap(),
        "Saruman of Many Colours".to_string()
    );

    assert_eq!(edges[1].get_label(), "lord of");
    assert_eq!(edges[1].get_from_vertex().get_label(), "sauron".to_string());
    assert_eq!(edges[1].get_to_vertex().get_label(), "saruman".to_string());
    assert_eq!(
        edges[1]
            .get_from_vertex()
            .get_attr("identified as")
            .unwrap(),
        "Necromancer".to_string()
    );

    assert_eq!(edges[2].get_relation(), "seeks");
    assert_eq!(edges[2].get_to_vertex().get_label(), "".to_string());
    assert_eq!(
        edges[2].get_to_vertex().get_attr("name").unwrap(),
        "Smeagol".to_string()
    );
}

#[test]
fn should_fail_import_from_csv_file_since_file_does_not_exists() {
    let csv_file_path = "./tmp/foobar.csv";
    let e = import_from_csv_gruphst_format(csv_file_path);
    assert!(e.is_err());
}

#[test]
fn should_fail_import_from_csv_file_since_file_is_empty() {
    let csv_file_path = "./tests/data/empty.csv";
    let e = import_from_csv_gruphst_format(csv_file_path);
    assert!(e.is_err());
}

#[test]
fn should_fail_import_from_csv_file_since_row_miss_a_relation() {
    let csv_file_path = "./tests/data/exported-wrong.csv";
    let e = import_from_csv_gruphst_format(csv_file_path);
    println!("----- {:#?}", e);

    assert!(e.is_err());
}

#[test]
fn should_fail_import_from_csv_file_since_one_header_value_is_missing() {
    let csv_file_path = "./tests/data/exported-wrong-header.csv";
    let e = import_from_csv_gruphst_format(csv_file_path);
    assert!(e.is_err());
}
