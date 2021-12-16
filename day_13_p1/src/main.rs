use std::collections::HashSet;
use std::str;

fn fold(i: i32, line: i32) -> i32 {
    (i - line).abs() + line
}
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

struct Vec2i(i32, i32);

impl From<&str> for Vec2i {
    fn from(input: &str) -> Vec2i {
        let mut it = input.split(",");
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        Vec2i(x, y)
    }
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();
    let mut it = input.split("\n\n");
    let points = it.next().unwrap();
    let folds = it.next().unwrap();

    let (direction, fold_line) = fold_from_str(folds.lines().next().unwrap());

    let mut grid = HashSet::new();

    for point in points.lines().map(|s| Vec2i::from(s)) {
        let (x, y) = match direction {
            Direction::X => (fold(point.0, fold_line), point.1),
            Direction::Y => (point.0, fold(point.1, fold_line)),
        };
        grid.insert((x, y));
    }
    println!("{}", grid.len());
}
