use std::str;
use grid::Grid;
use std::ops::Add;

#[derive(Copy, Clone)]
struct Vec2u {
    pub x: usize,
    pub y: usize,
}

impl Vec2u {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y
        }
    }
}

impl Add for Vec2u {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct LocalMinimaKernel<'a> {
    grid: &'a Grid<usize>,
    top_left: Vec2u,
}

impl<'a> LocalMinimaKernel<'a> {
    const DIMENSION: usize = 3;
    fn apply_over(grid: &'a Grid<usize>) -> Self {
        Self {
            grid,
            top_left: Vec2u::new(0, 0),
        }
    }
}

impl<'a> Iterator for LocalMinimaKernel<'a> {
    type Item = Option<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let top = self.top_left + Vec2u::new(1, 0);
        let left = self.top_left + Vec2u::new(0, 1);
        let middle = self.top_left + Vec2u::new(1, 1);
        let right = self.top_left + Vec2u::new(2, 1);
        let bottom = self.top_left + Vec2u::new(1, 2);

        self.top_left.x += 1;

        if self.top_left.x + LocalMinimaKernel::DIMENSION > self.grid.cols() {
            self.top_left.x = 0;
            self.top_left.y += 1;
            if self.top_left.y + LocalMinimaKernel::DIMENSION > self.grid.rows() {
                return None;
            }
        }

        if self.grid[middle.y][middle.x] < self.grid[top.y][top.x] &&
        self.grid[middle.y][middle.x] < self.grid[left.y][left.x] &&
        self.grid[middle.y][middle.x] < self.grid[right.y][right.x] &&
        self.grid[middle.y][middle.x] < self.grid[bottom.y][bottom.x] {
            return Some(Some(self.grid[middle.y][middle.x] + 1));
        } else {
            return Some(None);
        }
    }
}


fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();
    let width = input.lines().next().unwrap().len();
    let input = input.chars().filter(|c| *c != '\n').map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>();
    let grid = Grid::from_vec(input, width);

    let sum_of_risk = LocalMinimaKernel::apply_over(&grid).filter_map(|v| v).sum::<usize>();

    println!("{}", sum_of_risk);
}
