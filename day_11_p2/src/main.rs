use std::str;
use grid::Grid;
use std::collections::HashSet;

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();
    let input = input.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
    let mut grid = Grid::from_vec(input, 10);

    for step in 0..1000 {
        let mut flashed = HashSet::new();

        for element in grid.iter_mut() {
            *element += 1;
        }

        let mut flash = true;
        while flash {
            flash = false;
            for x in 0..grid.cols() {
                for y in 0..grid.rows() {
                    if grid[y][x] > 9 && !flashed.contains(&(x, y)) {
                        increase_surrounding(&mut grid, x as i32, y as i32);
                        flashed.insert((x, y));
                        flash = true;
                    }
                }
            }
        }

        for (x, y) in &flashed {
            grid[*y][*x] = 0;
        }

        if flashed.len() == 100 {
            println!("Step {}", step);
            return;
        }
    }

}

fn increase_surrounding(grid: &mut Grid<u32>, x: i32, y: i32) {
    if grid_contains(grid, x - 1, y - 1) {
        grid[(y - 1) as usize][(x - 1) as usize] += 1;
    }

    if grid_contains(grid, x, y - 1) {
        grid[(y - 1) as usize][x as usize] += 1;
    }

    if grid_contains(grid, x + 1, y - 1) {
        grid[(y - 1) as usize][(x + 1) as usize] += 1;
    }

    if grid_contains(grid, x - 1, y) {
        grid[y as usize][(x - 1) as usize] += 1;
    }

    if grid_contains(grid, x + 1, y) {
        grid[y as usize][(x + 1) as usize] += 1;
    }

    if grid_contains(grid, x - 1, y + 1) {
        grid[(y + 1) as usize][(x- 1) as usize] += 1;
    }

    if grid_contains(grid, x, y + 1) {
        grid[(y + 1) as usize][x as usize] += 1;
    }

    if grid_contains(grid, x + 1, y + 1) {
        grid[(y + 1) as usize][(x + 1) as usize] += 1;
    }
}

fn grid_contains(grid: &Grid<u32>, x: i32, y: i32) -> bool {
    x >= 0 && x < grid.cols() as i32 && y < grid.rows() as i32 && y >= 0
}
