use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec2u {
    pub x: usize,
    pub y: usize,
}

impl From<Vec2i> for Vec2u {
    fn from(input: Vec2i) -> Self {
        Self {
            x: input.x as usize,
            y: input.y as usize,
        }
    }
}

impl From<&str> for Vec2u {
    fn from(input: &str) -> Self {
        let mut digits = input.split(",");
        Self {
            x: digits.next().unwrap().parse().unwrap(),
            y: digits.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl AddAssign for Vec2i {
    fn add_assign(&mut self, other: Vec2i) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl Add for Vec2i {
    type Output = Vec2i;
    fn add(self, other: Vec2i) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<Vec2u> for Vec2i {
    fn from(input: Vec2u) -> Self {
        Self {
            x: input.x as i32,
            y: input.y as i32,
        }
    }
}

#[derive(Debug)]
pub struct Line {
    start: Vec2u,
    end: Vec2u,
}

impl Line {
    pub fn traversal(&self) -> LineTraversal {
        let mut direction = Vec2i::new(0, 0);

        if self.start.x < self.end.x {
            direction.x = 1;
        } else if self.start.x > self.end.x {
            direction.x = -1;
        }

        if self.start.y < self.end.y {
            direction.y = 1;
        } else if self.start.y > self.end.y {
            direction.y = -1;
        }

        LineTraversal {
            current: Vec2i::from(self.start),
            end: Vec2i::from(self.end) + direction,
            direction,
        }
    }
}

impl From<&str> for Line {
    fn from(input: &str) -> Self {
        let mut points = input.split(" -> ");
        let start = Vec2u::from(points.next().unwrap());
        let end = Vec2u::from(points.next().unwrap());

        Self { start, end }
    }
}

pub struct LineTraversal {
    current: Vec2i,
    end: Vec2i,
    direction: Vec2i,
}

impl Iterator for LineTraversal {
    type Item = Vec2u;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        }

        let previous = self.current;
        self.current += self.direction;
        Some(Vec2u::from(previous))
    }
}
