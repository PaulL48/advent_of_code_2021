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
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4
    };
}

fn is_open_bracket(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();
    let mut scores = input
        .lines()
        .filter_map(|l| get_incomplete_score(l))
        .collect::<Vec<_>>();
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}

fn get_incomplete_score(line: &str) -> Option<usize> {
    let mut bracket_stack = Vec::with_capacity(DEFAULT_CAPACITY);
    for c in line.chars() {
        if is_open_bracket(c) {
            bracket_stack.push(c);
        } else {
            let top = bracket_stack.pop().unwrap();
            if top != BRACKET_PAIRS[&c] {
                // line is corrupted, discard it
                return None;
            }
        }
    }

    if !bracket_stack.is_empty() {
        let mut score = 0;
        for remaining in bracket_stack.iter().rev() {
            score *= 5;
            score += POINT_VALUES[&remaining];
        }
        return Some(score);
    }

    None
}
