use gruphst::{edge::Edge, graphs::Graphs, vertex::Vertex};

// The idea it's to create some graph related with 
// the Middle-Earth, relating some characters and
// places

fn main() {
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
    graphs.add_edge(
        &Edge::create(
            &Vertex::new("Sam"),
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
    let the_shire = graphs.fin

    // We can print the current Graphs object
    println!("{:#?}", graphs);

}
