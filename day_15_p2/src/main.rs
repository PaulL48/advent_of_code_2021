use std::str;
use petgraph::graph::{NodeIndex, DiGraph};
use petgraph::algo::dijkstra;
use grid::Grid;

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();
    let grid = Grid::from_vec(input.chars().filter(|c| *c != '\n').map(|c| c.to_digit(10).unwrap()).collect(), 100);
    let mut node_indices = Grid::<NodeIndex>::init(500, 500, Default::default());
    let mut graph = DiGraph::with_capacity(250000, 998000);
    let mut start = None;
    let mut end = None;

    let original_width = 100;
    let original_height = 100;

    for x_iteration in 0..5 {
        for y_iteration in 0..5 {
            for row in 0..grid.rows() {
                for col in 0..grid.cols() {
                    let adj_row = y_iteration * original_height + row;
                    let adj_col = x_iteration * original_width + col;

                    node_indices[adj_col][adj_row] = graph.add_node(());
                    if row == 0 && col == 0 && x_iteration == 0 && y_iteration == 0 {
                        start = Some(node_indices[adj_col][adj_row]);
                    }
        
                    if row == grid.rows() - 1 && col == grid.cols() - 1 && x_iteration == 4 && y_iteration == 4 {
                        end = Some(node_indices[adj_col][adj_row]);
                    }
                }
            }
        }
    }

    for row in 0..grid.rows() * 5 {
        for col in 0..(grid.cols() * 5 - 1) {
            let col_adj = col % original_width;
            let sub_col_adj = (col + 1) % original_width;
            let row_adj = row % original_height;

            let outgoing_weight_adjust = row / original_height + (col + 1) / original_width;
            let incoming_weight_adjust = row / original_height + col / original_width;

            let outgoing_weight = grid[sub_col_adj][row_adj] + outgoing_weight_adjust as u32;
            let incoming_weight = grid[col_adj][row_adj] + incoming_weight_adjust as u32;

            let outgoing_weight = outgoing_weight - (9 * (outgoing_weight / 10));
            let incoming_weight = incoming_weight - (9 * (incoming_weight / 10));
            graph.add_edge(node_indices[col][row], node_indices[col + 1][row], outgoing_weight);
            graph.add_edge(node_indices[col + 1][row], node_indices[col][row], incoming_weight);
        }
    }

    for col in 0..grid.cols() * 5 {
        for row in 0..(grid.rows() * 5 - 1) {
            let col_adj = col % original_width;
            let row_adj = row % original_height;
            let sub_row_adj = (row + 1) % original_height;

            let outgoing_weight_adjust = (row + 1) / original_height + col / original_width;
            let incoming_weight_adjust = row / original_height + col / original_width;

            let outgoing_weight = grid[col_adj][sub_row_adj] + outgoing_weight_adjust as u32;
            let incoming_weight = grid[col_adj][row_adj] + incoming_weight_adjust as u32;

            let outgoing_weight = outgoing_weight - (9 * (outgoing_weight / 10));
            let incoming_weight = incoming_weight - (9 * (incoming_weight / 10));
            graph.add_edge(node_indices[col][row], node_indices[col][row + 1], outgoing_weight);
            graph.add_edge(node_indices[col][row + 1], node_indices[col][row], incoming_weight);
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();

    let paths = dijkstra(&graph, start, Some(end), |c| *c.weight());
    println!("{}", paths[&end]);
}
