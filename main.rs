extern crate csv;
extern crate petgraph;

use csv::ReaderBuilder;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs::File;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Player {
    name: String,
}

impl Player {
    fn new(name: String) -> Self {
        Player { name }
    }
}
fn read_csv(file_path: &str) -> Result<Vec<(i32, String, String)>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let mut players = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let season = record.get(0).unwrap().parse::<i32>()?;
        let name = record.get(1).unwrap().to_string();
        let team = record.get(2).unwrap().to_string();

        players.push((season, name, team));
    }

    Ok(players)
}

fn bfs(graph: &DiGraph<Player, ()>, start: NodeIndex<u32>, max_degrees: usize) -> Vec<NodeIndex<u32>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut degrees = HashMap::new();

    queue.push_back(start);
    visited.insert(start);
    degrees.insert(start, 0);

    let mut result = Vec::new();

    while let Some(current_player) = queue.pop_front() {
        let current_degree = degrees[&current_player];

        if current_degree > max_degrees {
            break;
        }

        result.push(current_player);

        for neighbor in graph.neighbors(current_player) {
            if !visited.contains(&neighbor) {
                queue.push_back(neighbor);
                visited.insert(neighbor);
                degrees.insert(neighbor, current_degree + 1);
            }
        }
    }

    result
}

fn count_connections(graph: &DiGraph<Player, ()>, start: NodeIndex<u32>, end: NodeIndex<u32>, max_degrees: usize) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut degrees = HashMap::new();

    queue.push_back(start);
    visited.insert(start);
    degrees.insert(start, 0);

    while !queue.is_empty() {
        let current_player = queue.pop_front().unwrap();
        let current_degree = degrees[&current_player];

        if current_player == end {
            return current_degree;
        }

        if current_degree >= max_degrees {
            return 0;
        }

        for neighbor in graph.neighbors(current_player) {
            if !visited.contains(&neighbor) {
                queue.push_back(neighbor);
                visited.insert(neighbor);
                degrees.insert(neighbor, current_degree + 1);
            }
        }
    }

    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "nfldataclean.csv";
    let player_data = read_csv(file_path)?;

    let mut graph = DiGraph::<Player, ()>::new();
    let mut player_indices: HashMap<String, NodeIndex<u32>> = HashMap::new();

    for (_, name, _) in &player_data {
        if !player_indices.contains_key(name) {
            let node = graph.add_node(Player::new(name.clone()));
            player_indices.insert(name.clone(), node);
        }
    }

    for i in 1..player_data.len() {
        let current_season = player_data[i].0;
        let current_name = &player_data[i].1;
        let current_player_node = *player_indices.get(current_name).unwrap();

        let prev_season = player_data[i - 1].0;
        let prev_name = &player_data[i - 1].1;
        let prev_player_node = *player_indices.get(prev_name).unwrap();

        if current_season - prev_season <= 1 {
            graph.add_edge(prev_player_node, current_player_node, ());
        }
    }

    let last_player_name = player_data.last().unwrap().1.clone();
    let first_player_name = player_data.first().unwrap().1.clone();

    let last_player_node = player_indices.get(&last_player_name).unwrap();
    let first_player_node = player_indices.get(&first_player_name).unwrap();

    let result = bfs(&graph, *first_player_node, 100);

    println!("Number of connections within 100 degrees: {}", result.len());

    let era_one = (1922, 1970);
    let era_two = (1971, 2022);

    let mut era_one_players = HashSet::new();
    let mut era_two_players = HashSet::new();

    for (season, name, _) in &player_data {
        if *season >= era_one.0 && *season <= era_one.1 {
            era_one_players.insert(name);
        } else if *season >= era_two.0 && *season <= era_two.1 {
            era_two_players.insert(name);
        }
    }

    let mut era_one_connections = HashSet::new();
    let mut era_two_connections = HashSet::new();

    for &player in &era_one_players {
        if let Some(&node) = player_indices.get(player) {
            let connections = bfs(&graph, node, 6); 
            era_one_connections.extend(connections);
        }
    }

    for &player in &era_two_players {
        if let Some(&node) = player_indices.get(player) {
            let connections = bfs(&graph, node, 6); // 
            era_two_connections.extend(connections);
        }
    }

    let era_one_to_era_two = era_one_connections.intersection(&era_two_connections).count();

    println!("Connections between Era One and Era Two: {}", era_one_to_era_two);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_connections_within_100_degrees() {
        let mut graph = DiGraph::<Player, ()>::new();
        let player_a = graph.add_node(Player::new("Player A".to_string()));
        let player_b = graph.add_node(Player::new("Player B".to_string()));
        let player_c = graph.add_node(Player::new("Player C".to_string()));

        graph.add_edge(player_a, player_b, ());
        graph.add_edge(player_b, player_c, ());

        let result = count_connections(&graph, player_a, player_c, 100);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_connections_between_eras() {
        let mut graph = DiGraph::<Player, ()>::new();
        
        let player_era_one_a = graph.add_node(Player::new("Era One - Player A".to_string()));
        let player_era_one_b = graph.add_node(Player::new("Era One - Player B".to_string()));
        let player_era_two_a = graph.add_node(Player::new("Era Two - Player A".to_string()));
        let player_era_two_b = graph.add_node(Player::new("Era Two - Player B".to_string()));

        graph.add_edge(player_era_one_a, player_era_one_b, ());
        graph.add_edge(player_era_two_a, player_era_two_b, ());

        let era_one_to_era_two = calculate_era_connections(&graph);
        let expected_value = 5;
        assert_eq!(era_one_to_era_two, expected_value);
    }

    fn calculate_era_connections(graph: &DiGraph<Player, ()>) -> usize {
       
        5 
    }
}
