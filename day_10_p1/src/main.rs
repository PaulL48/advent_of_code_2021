use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;
use std::str;

const DEFAULT_CAPACITY: usize = 32;

lazy_static! {
    static ref BRACKET_PAIRS: HashMap<char, char> = hashmap! {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<'
    };
    static ref POINT_VALUES: HashMap<char, usize> = hashmap! {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137
    };
}

fn is_open_bracket(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();
    let sum = input
        .lines()
        .filter_map(|l| get_error_score(l))
        .sum::<usize>();
    println!("{}", sum);
}

fn get_error_score(line: &str) -> Option<usize> {
    let mut bracket_stack = Vec::with_capacity(DEFAULT_CAPACITY);
    for c in line.chars() {
        if is_open_bracket(c) {
            bracket_stack.push(c);
        } else {
            let top = bracket_stack.pop().unwrap();
            if top != BRACKET_PAIRS[&c] {
                return Some(POINT_VALUES[&c]);
            }
        }
    }
    None
}
