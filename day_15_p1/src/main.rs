use std::str;
use petgraph::graph::{NodeIndex, DiGraph};
use petgraph::algo::dijkstra;
use grid::Grid;

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();
    let grid = Grid::from_vec(input.chars().filter(|c| *c != '\n').map(|c| c.to_digit(10).unwrap()).collect(), 100);
    let mut node_indices = Grid::<NodeIndex>::init(100, 100, Default::default());
    let mut graph = DiGraph::with_capacity(10000, 39_600);
    let mut start = None;
    let mut end = None;

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            node_indices[col][row] = graph.add_node(());
            if row == 0 && col == 0 {
                start = Some(node_indices[col][row]);
            }

            if row == grid.rows() - 1 && col == grid.cols() - 1 {
                end = Some(node_indices[col][row]);
            }
        }
    }

    for row in 0..grid.rows() {
        for col in 0..(grid.cols() - 1) {
            graph.add_edge(node_indices[col][row], node_indices[col + 1][row], grid[col + 1][row]);
            graph.add_edge(node_indices[col + 1][row], node_indices[col][row], grid[col][row]);
        }
    }

    for col in 0..grid.cols() {
        for row in 0..(grid.rows() - 1) {
            graph.add_edge(node_indices[col][row], node_indices[col][row + 1], grid[col][row + 1]);
            graph.add_edge(node_indices[col][row + 1], node_indices[col][row], grid[col][row]);
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();

    let paths = dijkstra(&graph, start, Some(end), |c| *c.weight());
    println!("{}", paths[&end]);
}
