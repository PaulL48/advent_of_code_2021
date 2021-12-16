use grid::Grid;
use std::str;

fn fold(i: i32, line: i32) -> i32 {
    -(i - line).abs() + line
}

#[derive(Debug)]
enum Direction {
    X,
    Y,
}

impl From<&str> for Direction {
    fn from(input: &str) -> Direction {
        match input {
            "x" => Direction::X,
            "y" => Direction::Y,
            _ => panic!(),
        }
    }
}

fn fold_from_str(input: &str) -> (Direction, i32) {
    let expression = input.split(" ").collect::<Vec<_>>()[2];
    let mut it = expression.split("=");
    let direction = Direction::from(it.next().unwrap());
    let value = it.next().unwrap().parse().unwrap();
    (direction, value)
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Vec2i(i32, i32);

impl From<&str> for Vec2i {
    fn from(input: &str) -> Vec2i {
        let mut it = input.split(",");
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        Vec2i(x, y)
    }
}

fn apply_folds(folds: &Vec<(Direction, i32)>, mut p: Vec2i) -> Vec2i {
    for (direction, fold_line) in folds {
        p = match direction {
            Direction::X => Vec2i(fold(p.0, *fold_line), p.1),
            Direction::Y => Vec2i(p.0, fold(p.1, *fold_line)),
        };
    }
    p
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();
    let mut it = input.split("\n\n");
    let points = it.next().unwrap();
    let folds = it.next().unwrap();
    let folds = folds.lines().map(|s| fold_from_str(s)).collect::<Vec<_>>();

    let mut grid = Grid::new(6, 5 * 8);

    for point in points.lines().map(|s| Vec2i::from(s)) {
        let a = apply_folds(&folds, point);
        grid[a.1 as usize][a.0 as usize] = "█";
    }

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            print!("{:1}", grid[row][col]);
        }
        println!()
    }
}
