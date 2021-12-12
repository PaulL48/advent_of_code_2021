use petgraph::{graph::NodeIndex, Graph, Undirected};
use std::collections::{HashMap, HashSet};
use std::str;
use std::thread;

#[derive(Debug, Copy, Clone, Hash)]
struct Cave<'a> {
    pub id: &'a str,
    pub small: bool,
}

impl<'a> From<&'a str> for Cave<'a> {
    fn from(input: &'a str) -> Cave<'a> {
        let small = input == input.to_ascii_lowercase();
        Cave { id: input, small }
    }
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();
    let mut graph = Graph::<Cave, (), Undirected>::default();
    let mut nodes = HashMap::new();

    let mut start_node = None;
    let mut end_node = None;

    for line in input.lines() {
        let mut iter = line.split("-");
        let lhs = iter.next().unwrap();
        let rhs = iter.next().unwrap();

        let lhs_index = if let Some(index) = nodes.get(lhs) {
            *index
        } else {
            // println!("{:?}", Cave::from(lhs));
            let index = graph.add_node(Cave::from(lhs));
            nodes.insert(lhs, index);
            if lhs == "start" {
                start_node = Some(index);
            }
            index
        };

        let rhs_index = if let Some(index) = nodes.get(rhs) {
            *index
        } else {
            // println!("{:?}", Cave::from(rhs));
            let index = graph.add_node(Cave::from(rhs));
            nodes.insert(rhs, index);
            if rhs == "end" {
                end_node = Some(index);
            }
            index
        };

        graph.add_edge(lhs_index, rhs_index, ());
    }

    let start_node = start_node.unwrap();
    let end_node = end_node.unwrap();
    let visited = HashSet::new();
    let output = unique_paths(&graph, visited, start_node, Vec::new(), end_node, false);

    println!("{}", output.len());
}

fn unique_paths<'a>(
    graph: &'a Graph<Cave, (), Undirected>,
    mut visited: HashSet<NodeIndex>,
    current: NodeIndex,
    mut current_path: Vec<NodeIndex>,
    end: NodeIndex,
    mut small_twice: bool,
) -> Vec<Vec<NodeIndex>> {
    // Vec<Path>
    // If the current node is small, add it to visited
    if graph[current].small {
        visited.insert(current);
    }

    let mut possible_paths = Vec::new();
    current_path.push(current);

    // If the node we're investigating is the end, there is only the one possible path
    if current == end {
        return vec![current_path];
    }

    // For every neighbor
    for neighbor in graph.neighbors(current) {
        if graph[neighbor].id == "start" {
            continue;
        }

        // If its not a visited small cave (ie. an unvisited small cave or a large cave)
        if !visited.contains(&neighbor) {
            for possible_path in unique_paths(
                graph,
                visited.clone(),
                neighbor,
                current_path.clone(),
                end,
                small_twice,
            ) {
                possible_paths.push(possible_path);
            }
        } else if !small_twice {
            for possible_path in unique_paths(
                graph,
                visited.clone(),
                neighbor,
                current_path.clone(),
                end,
                true,
            ) {
                possible_paths.push(possible_path);
            }
        }
    }

    possible_paths
}
