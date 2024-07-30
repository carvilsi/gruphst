use gruphst::{node::Node, graph::Graph, graphs::Graphs, *};
use std::collections::HashMap;
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

fn who_wins(wins: HashMap<String, Vec<Node>>, a_game: &Node, b_game: &Node) -> Option<String> {
    for (action, targets) in wins.iter() {
        for target in targets {
            if target.get_id() == b_game.get_id() {
                let res = format!("{} {} {}", a_game.get_label(), action, b_game.get_label());
                return Some(res);
            }
        }
    }
    None
}

fn resolve_game(player_one_game: Node, player_two_game: Node, rules: Graphs) -> String {
    let one = format!("player one plays: {}", player_one_game.get_label());
    let player_one_wins_to = player_one_game
        .get_relations_out_on_graph(rules.get_graphs(Some("game-rules")).unwrap())
        .unwrap();
    let player_two_wins_to = player_two_game
        .get_relations_out_on_graph(rules.get_graphs(Some("game-rules")).unwrap())
        .unwrap();
    let two = format!("player two plays: {}", player_two_game.get_label());

    let mut game_result = String::from("");
    if player_one_game.get_label() == player_two_game.get_label() {
        game_result = String::from("Tie");
    }

    let mut res = who_wins(player_one_wins_to, &player_one_game, &player_two_game);
    if res.is_some() {
        game_result = format!("{}\nPlayer one wins!", res.unwrap());
    }

    res = who_wins(player_two_wins_to, &player_two_game, &player_one_game);
    if res.is_some() {
        game_result = format!("{}\nPlayer two wins!", res.unwrap());
    }
    
    format!("{}\n{}\n{}", one, two, game_result)
}

fn main() {
    let rules = create_game_rules();

    // maybe now you want to persists the game rules 
    // to use it other day ;)
    let _ = rules.persists(); 

    // get the characters
    let characters = rules.get_uniq_nodes(None).unwrap();

    // Lets play a bit
    let mut rand_number: usize = rand::thread_rng().gen_range(0..characters.len()).try_into().unwrap();
    let player_one_game: &Node = &characters[rand_number];
    rand_number = rand::thread_rng().gen_range(0..characters.len()).try_into().unwrap();
    let player_two_game: &Node = &characters[rand_number];
    let result = resolve_game(player_one_game.clone(), player_two_game.clone(), rules);
    println!("{}", result);
}
