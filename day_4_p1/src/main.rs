use std::collections::HashMap;
use std::str;

const BOARD_WIDTH: usize = 5;
const BOARD_HEIGHT: usize = 5;

enum BoardState {
    Win,
    Undecided,
}

struct Board<'a, const WIDTH: usize, const HEIGHT: usize> {
    numbers: HashMap<&'a str, (usize, usize)>,
    row_totals: [usize; WIDTH],
    column_totals: [usize; HEIGHT],
    sum: i32,
}

impl<'a, const WIDTH: usize, const HEIGHT: usize> Board<'a, WIDTH, HEIGHT> {
    pub fn call_number(&mut self, number_str: &str, number: i32) -> BoardState {
        if !self.numbers.contains_key(number_str) {
            return BoardState::Undecided;
        }

        let (x, y) = self.numbers[number_str];
        self.sum -= number;
        self.row_totals[y] += 1;
        self.column_totals[x] += 1;

        if self.row_totals[y] == WIDTH || self.column_totals[x] == HEIGHT {
            BoardState::Win
        } else {
            BoardState::Undecided
        }
    }

    pub fn score(&self) -> i32 {
        self.sum
    }
}

impl<'a, const WIDTH: usize, const HEIGHT: usize> From<&'a str> for Board<'a, WIDTH, HEIGHT> {
    fn from(input: &'a str) -> Self {
        let mut numbers = HashMap::new();
        let mut board_sum = 0;

        for (row, line) in input.lines().enumerate() {
            for (col, number) in line.split(" ").filter(|n| !n.is_empty()).enumerate() {
                numbers.insert(number, (col, row));
                board_sum += number.parse::<i32>().unwrap();
            }
        }

        Self {
            numbers,
            row_totals: [0; WIDTH],
            column_totals: [0; HEIGHT],
            sum: board_sum,
        }
    }
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();
    let mut input_iter = input.split("\n\n");
    let called_numbers = input_iter.next().unwrap().split(",").collect::<Vec<_>>();
    let mut boards = input_iter
        .map(|board_str| Board::<BOARD_WIDTH, BOARD_HEIGHT>::from(board_str))
        .collect::<Vec<_>>();

    println!("Calling numbers");
    for number in called_numbers {
        let numeric = number.parse::<i32>().unwrap();
        for board in &mut boards {
            if let BoardState::Win = board.call_number(number, numeric) {
                println!(
                    "Winning board score: {}; Called on: {}; Product: {}",
                    board.score(),
                    numeric,
                    board.score() * numeric
                );
                std::process::exit(0);
            }
        }
    }
}
