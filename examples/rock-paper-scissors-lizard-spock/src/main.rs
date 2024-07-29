use gruphst::{node::Node, graph::Graph, graphs::Graphs, *};
use rand::Rng;

fn create_game_rules() -> Graphs {
    // Create the sets of nodes and add some attributes
    let mut rock = Node::new("Rock");
    rock.set_attr("description", "A strong stone");

    let mut paper = Node::new("Paper");
    paper.set_attr("description", "A blank paper");

    let mut scissors = Node::new("Scissors");
    scissors.set_attr("description", "Quite sharp");

    let mut lizard = Node::new("Lizard");
    lizard.set_attr("description", "A reptile");

    let mut spock = Node::new("Spock");
    spock.set_attr("description", "A Mr. from Vulcan");

    // Lets create a Graphs vault to store the rules
    let mut graphs = Graphs::init("game-rules");

    // Now we create the first relation between two nodes
    // and we'll add to the vault's default graph 
    let graph = Graph::create(&rock, "crushes", &lizard);
    graphs.add_graph(&graph, None);

    // More relations will be added
    graphs.add_graph(&Graph::create(&rock, "crushes", &scissors), None);
    graphs.add_graph(&Graph::create(&lizard, "poisons", &spock), None);
    graphs.add_graph(&Graph::create(&lizard, "eats", &paper), None);
    graphs.add_graph(&Graph::create(&spock, "smashes", &scissors), None);
    graphs.add_graph(&Graph::create(&spock, "vaporizes", &rock), None);
    graphs.add_graph(&Graph::create(&scissors, "cuts", &paper), None);
    graphs.add_graph(&Graph::create(&scissors, "decapites", &lizard), None);
    graphs.add_graph(&Graph::create(&paper, "covers", &rock), None);
    graphs.add_graph(&Graph::create(&paper, "disproves", &spock), None);

    graphs
}

fn main() {
    let rules = create_game_rules();

    // maybe now you want to persists the game rules 
    let _ = rules.persists(); 

    let characters = rules.get_uniq_nodes(None).unwrap();
    // Lets play a bit
    let mut rand_number: usize = rand::thread_rng().gen_range(0..characters.len()).try_into().unwrap();
    let player_one_game: &Node = &characters[rand_number];
    println!("player one game {:#?}", player_one_game);
    rand_number = rand::thread_rng().gen_range(0..characters.len()).try_into().unwrap();
    let player_two_game: &Node = &characters[rand_number];
    println!("player second game {:#?}", player_two_game);

}

