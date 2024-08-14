use gruphst::{edge::Edge, graphs::Graphs, vertex::Vertex};
use std::error::Error;

// The idea it's to create some graph related with 
// the Middle-Earth, relating some characters and
// places

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new vertex
    let frodo = Vertex::new("Frodo");
       
    // Let's create another vertex
    let mut gandalf = Vertex::new("Gandalf");
     
    // A vertex can have attributes
    gandalf.set_attr("known as", "The Gray");
    gandalf.set_attr("years old", 24000);

    // Now lets make a relation between these two friends
    // by creating an Edge
    let mut edge = Edge::create(&gandalf, "friend of", &frodo);

    // An Edge can have attributes
    edge.set_attr("duration in years", 42);

    // Now we need something to hold, and store the created Edge
    // and the new ones that we'll create later.
    // Lets init a Graphs, we could do this step at the begining 
    // of the main function.
    let mut graphs = Graphs::init("middle-earth");
    
    // Now we add the edge or relation between Gandalf and Frodo
    graphs.add_edge(&edge, None);

    // We can add another relation or Edge to the graphs
    // for these two friends, e.g.
    graphs.add_edge(&Edge::create(&frodo, "has best friend", &gandalf), None);

    // Lets create more vertices for places and characters and edges
    // for the relation between them
    let mut sam = Vertex::new("Samwise");
    sam.set_attr("surname", "Gamgee");
    graphs.add_edge(
        &Edge::create(
            &sam,
            "has best friend",
            &frodo),
        None);

    let mut vertex = Vertex::new("The Shire");

    // Vertices and Edges has a uuid generated on creation
    let id_vertex_the_shire = vertex.get_id();

    graphs.add_edge(&Edge::create(&frodo, "lives at", &vertex), None); 

    vertex = Vertex::new("Isengard");
    vertex.set_attr("type", "tower");

    graphs.add_edge(&Edge::create(&Vertex::new("Saruman"), "lives at", &vertex), None); 

    // we can use the id or the label to retrieve a Vertex that we have on Graph
    let the_shire = graphs.find_vertex_by_id(id_vertex_the_shire.as_str(), None)?;

    graphs.add_edge(&Edge::create(&sam, "lives at", &the_shire), None); 

    // Now we can do things like get stats of the Graphs
    let stats = graphs.get_stats();

    // and print it
    println!("{:#?}", stats);
    // GraphsStats {
    //    mem: 1578,
    //    total_edges: 6,
    //    total_graphs: 1,
    //    total_attr: 8,
    //    total_vertices: 12,
    //    uniq_rel: 3,
    //    max_mem: 104857600,
    // }
    
    // or get some value from stats
    // like the amount of vertices
    assert_eq!(stats.get_total_vertices(), 12);

    // We can print the current Graphs object
    println!("{:#?}", graphs);

    // We can retrieve the uniq relations from the graph
    let unique_relations_vertices = graphs.uniq_relations();
    assert_eq!(unique_relations_vertices, vec!["friend of", "has best friend", "lives at"]);

    // Also possible to retrieve the vertices that has a certain
    // relation in
    let vertices_with_relation_in = graphs.has_relation_in("lives at", None)?; 
    assert_eq!(vertices_with_relation_in[0].get_label(), "The Shire");
    assert_eq!(vertices_with_relation_in[1].get_label(), "Isengard");

    // Or get the edge that has a vertex with an attribute equals to
    let found = graphs.attr_equals_to("years old", 24000, None)?;
    assert_eq!(found[0].get_from_vertex().get_label(), "Gandalf");

    // Since we have a humble middle-earth network
    // we can persists it for another day
    // a file called "middle-earth.grphst" will be created, 
    // later we can load it with:
    // let loaded_graphs = Graphs::load("middle-earth.grphst")?;
    graphs.persists()?;

    Ok(())
}
