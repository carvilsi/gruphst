use gruphst::{edge::Edge, graphs::Graphs, vertex::Vertex};
use rand::Rng;
use std::collections::HashMap;

fn create_game_rules() -> Graphs {
    // Create the sets of edges and add some attributes
    let mut rock = Vertex::new("Rock");
    rock.set_attr("description", "A strong stone");

    let mut paper = Vertex::new("Paper");
    paper.set_attr("description", "A blank paper");

    let mut scissors = Vertex::new("Scissors");
    scissors.set_attr("description", "Quite sharp");

    let mut lizard = Vertex::new("Lizard");
    lizard.set_attr("description", "A reptile");

    let mut spock = Vertex::new("Spock");
    spock.set_attr("description", "A Mr. from Vulcan");

    // Lets create a Graphs vault to store the rules
    let mut graphs = Graphs::init("game-rules");

    // Now we create the first relation between two edges
    // and we'll add to the vault's default graph
    let vertex = Edge::create(&rock, "crushes", &lizard);
    graphs.add_edge(&vertex, None);

    // More relations will be added
    graphs.add_edge(&Edge::create(&rock, "crushes", &scissors), None);
    graphs.add_edge(&Edge::create(&lizard, "poisons", &spock), None);
    graphs.add_edge(&Edge::create(&lizard, "eats", &paper), None);
    graphs.add_edge(&Edge::create(&spock, "smashes", &scissors), None);
    graphs.add_edge(&Edge::create(&spock, "vaporizes", &rock), None);
    graphs.add_edge(&Edge::create(&scissors, "cuts", &paper), None);
    graphs.add_edge(&Edge::create(&scissors, "decapites", &lizard), None);
    graphs.add_edge(&Edge::create(&paper, "covers", &rock), None);
    graphs.add_edge(&Edge::create(&paper, "disproves", &spock), None);

    graphs
}

fn who_wins(wins: HashMap<String, Vec<Vertex>>, a_game: &Vertex, b_game: &Vertex) -> Option<String> {
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

fn resolve_game(player_one_game: Vertex, player_two_game: Vertex, rules: Graphs) -> String {
    let one = format!("Player one plays: {}", player_one_game.get_label());
    let player_one_wins_to = player_one_game
        .get_relations_out_on_edges(rules.get_edges(Some("game-rules")).unwrap())
        .unwrap();
    let player_two_wins_to = player_two_game
        .get_relations_out_on_edges(rules.get_edges(Some("game-rules")).unwrap())
        .unwrap();
    let two = format!("Player two plays: {}", player_two_game.get_label());

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
    let _ = rules.persists(None);

    // get the characters
    let characters = rules.get_uniq_vertices(None).unwrap();

    // Lets play a bit
    // player one game
    let mut rand_number: usize = rand::thread_rng()
        .gen_range(0..characters.len())
        .try_into()
        .unwrap();
    let player_one_game: &Vertex = &characters[rand_number];

    // player two game
    rand_number = rand::thread_rng()
        .gen_range(0..characters.len())
        .try_into()
        .unwrap();
    let player_two_game: &Vertex = &characters[rand_number];

    // Lets see who won
    let result = resolve_game(player_one_game.clone(), player_two_game.clone(), rules);
    println!("{}", result);
}
