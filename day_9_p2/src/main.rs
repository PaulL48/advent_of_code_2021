use grid::Grid;
use std::collections::HashSet;
use std::ops::Add;
use std::str;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Vec2u {
    pub x: usize,
    pub y: usize,
}

impl Vec2u {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
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
    type Item = Option<Vec2u>;

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

        if self.grid[middle.y][middle.x] < self.grid[top.y][top.x]
            && self.grid[middle.y][middle.x] < self.grid[left.y][left.x]
            && self.grid[middle.y][middle.x] < self.grid[right.y][right.x]
            && self.grid[middle.y][middle.x] < self.grid[bottom.y][bottom.x]
        {
            return Some(Some(middle));
        } else {
            return Some(None);
        }
    }
}

struct GradientAscent<'a> {
    grid: &'a Grid<usize>,
    visited: HashSet<Vec2u>,
    queue: Vec<Vec2u>,
}

impl<'a> GradientAscent<'a> {
    fn from_point(point: Vec2u, grid: &'a Grid<usize>) -> Self {
        let queue = vec![point];
        Self {
            grid,
            visited: HashSet::new(),
            queue,
        }
    }
}

impl<'a> Iterator for GradientAscent<'a> {
    type Item = Vec2u;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            return None;
        }

        let mut current = self.queue.pop().unwrap();
        while self.visited.contains(&current) && !self.queue.is_empty() {
            current = self.queue.pop().unwrap();
        }

        if self.visited.contains(&current) {
            return None;
        }

        self.visited.insert(current);

        let top = Vec2u::new(current.x, current.y - 1);
        let left = Vec2u::new(current.x - 1, current.y);
        let right = Vec2u::new(current.x + 1, current.y);
        let bottom = Vec2u::new(current.x, current.y + 1);

        if !self.visited.contains(&top) && self.grid[top.y][top.x] != 9 {
            self.queue.push(top);
        }

        if !self.visited.contains(&left) && self.grid[left.y][left.x] != 9 {
            self.queue.push(left);
        }

        if !self.visited.contains(&right) && self.grid[right.y][right.x] != 9 {
            self.queue.push(right);
        }

        if !self.visited.contains(&bottom) && self.grid[bottom.y][bottom.x] != 9 {
            self.queue.push(bottom);
        }

        Some(current)
    }
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();
    let width = input.lines().next().unwrap().len();
    let input = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let grid = Grid::from_vec(input, width);

    let minima = LocalMinimaKernel::apply_over(&grid)
        .filter_map(|v| v)
        .collect::<Vec<_>>();

    let mut basin_sizes = minima
        .iter()
        .map(|p| GradientAscent::from_point(*p, &grid).count())
        .collect::<Vec<_>>();
    basin_sizes.sort_unstable();
    let largest = basin_sizes.iter().rev().take(3).product::<usize>();

    println!("{}", largest);
}
