mod linear;

use grid::Grid;
use linear::Line;
use std::str;

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();
    let mut region: Grid<usize> = Grid::new(1000, 1000);

    for line in input.lines() {
        let line = Line::from(line);
        let traversal = line.traversal();
        if !traversal.is_diagonal() {
            for point in traversal {
                region[point.y][point.x] += 1;
            }
        }
    }

    let count = region.iter().filter(|v| **v >= 2).count();
    println!("Count: {}", count);
}
