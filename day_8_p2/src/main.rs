use std::str;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let mut iter = line.split(" | ");


        let encoding = iter.next().unwrap().split(" ");
        let mapping = get_mapping(encoding);

        let digits = iter.next().unwrap().split(" ");

        print!("decoding: ");
        for digit in digits {
            println!("{} ", digit);
            sum += mapping[digit];
        }
        println!();
    }

    println!("{}", sum);
}

// fn get_mapping<'a, T: Iterator<Item = &'a str> + Clone>(digits: T) -> HashMap<&'a str, i32> {
//     const ONE: i32 = 2;
//     const FOUR: i32 = 4;
//     const SEVEN: i32 = 3;
//     const EIGHT: i32 = 7;
//     let mut mappings = HashMap::new();
//     let mut digits_remaining = digits.clone().collect::<HashSet<_>>();

//     for string in digits_remaining.iter().cloned().collect::<Vec<_>>() {
//         let diff_one = string.len() as i32 - ONE;
//         if diff_one == 0 {
//             mappings.insert(string, 1);
//             digits_remaining.remove(string);
//         } else if diff_one == 1 {
//             mappings.insert(string, 7);
//             digits_remaining.remove(string);
//         } else if diff_one == 2 {
//             mappings.insert(string, 4);
//             digits_remaining.remove(string);
//         } else if diff_one == 5 {
//             mappings.insert(string, 6);
//             digits_remaining.remove(string);
//         }
//     }

//     for string in digits_remaining.iter().cloned().collect::<Vec<_>>() {
//         let diff_one = string.len() as i32 - ONE;
//         let diff_four = string.len() as i32 - FOUR;
//         let diff_seven = string.len() as i32 - SEVEN;
//         let diff_eight = EIGHT - string.len() as i32;
        
//         if diff_one == 0 {
//             print!("{} is {} ", string, 1);
//             mappings.insert(string, 1);
//             digits_remaining.remove(string);
//         } else if diff_one == 5 {
//             print!("{} is {} ", string, 6);
//             mappings.insert(string, 6);
//             digits_remaining.remove(string);
//         } 

//         if diff_four == 0 {
//             print!("{} is {} ", string, 4);
//             mappings.insert(string, 4);
//             digits_remaining.remove(string);
//         } else if diff_four == 1 {
//             print!("{} is {} ", string, 9);
//             mappings.insert(string, 9);
//             digits_remaining.remove(string);
//         }

//         if diff_seven == 0 {
//             print!("{} is {} ", string, 7);
//             mappings.insert(string, 7);
//             digits_remaining.remove(string);
//         }

//         if diff_eight == 0 {
//             print!("{} is {} ", string, 8);
//             mappings.insert(string, 8);
//             digits_remaining.remove(string);
//         }
//     }

//     for string in digits_remaining.iter().cloned().collect::<Vec<_>>() {
//         let diff_seven = string.len() as i32 - SEVEN;
//         let diff_eight = EIGHT - string.len() as i32;

//     }

//     for string in digits_remaining.iter().cloned().collect::<Vec<_>>() {
//         let diff_four = string.len() as i32 - FOUR;

//     }



//     let mut strings = digits.clone().collect::<HashSet<_>>();


    
//     print!("Encoding: ");
//     for string in digits {
//         let diff_one = string.len() as i32 - ONE;
//         let diff_four = string.len() as i32 - FOUR;
//         let diff_seven = string.len() as i32 - SEVEN;
//         let diff_eight = EIGHT - string.len() as i32;

//         if diff_one == 0 {
//             print!("{} is {} ", string, 1);
//             mappings.insert(string, 1);
//             strings.remove(string);
//         } else if diff_one == 5 {
//             print!("{} is {} ", string, 6);
//             mappings.insert(string, 6);
//             strings.remove(string);
//         } 

//         if diff_four == 0 {
//             print!("{} is {} ", string, 4);
//             mappings.insert(string, 4);
//             strings.remove(string);
//         } else if diff_four == 1 {
//             print!("{} is {} ", string, 9);
//             mappings.insert(string, 9);
//             strings.remove(string);
//         }

//         if diff_seven == 0 {
//             print!("{} is {} ", string, 7);
//             mappings.insert(string, 7);
//             strings.remove(string);
//         } else if diff_seven == 2 {
//             print!("{} is {} ", string, 3);
//             mappings.insert(string, 3);
//             strings.remove(string);
//         }

//         if diff_eight == 0 {
//             print!("{} is {} ", string, 8);
//             mappings.insert(string, 8);
//             strings.remove(string);
//         } else if diff_eight == 1 {
//             print!("{} is {} ", string, 0);
//             mappings.insert(string, 0);
//             strings.remove(string);
//         }
//     }

//     for string in strings {
//         let diff_four = string.len() as i32 - FOUR;

//         if diff_four == 3 {
//             print!("{} is {} ", string, 2);
//             mappings.insert(string, 2);
//         }

//         if diff_four == 2 {
//             print!("{} is {} ", string, 5);
//             mappings.insert(string, 5);
//         }
//     }
//     println!();

//     mappings
// }

const A: u8 = 0b00000001;
const B: u8 = 0b00000010;
const C: u8 = 0b00000100;
const D: u8 = 0b00001000;
const E: u8 = 0b00010000;
const F: u8 = 0b00100000;
const G: u8 = 0b01000000;

fn get_mapping<'a, T: Iterator<Item = &'a str> + Clone>(digits: T) -> HashMap<&'a str, i32> {

    let mut decoder = HashMap::new();
    for digit in digits {
        match digit.len() {
            2 => decoder.insert(digit, chars_to_maskmap(digit)),
            3 => (),
            4 => (),
            7 => ()
        }
    }



    const ONE: i32 = 2;
    const FOUR: i32 = 4;
    const SEVEN: i32 = 3;
    const EIGHT: i32 = 7;
    let mut digits_remaining = digits.clone().collect::<HashSet<_>>();


}

fn chars_to_maskmap(s: &str) -> u8 {
    let mask_map = {
        let h = HashMap::new();
        h.insert('a', A);
        h.insert('b', B);
        h.insert('c', C);
        h.insert('d', D);
        h.insert('e', E);
        h.insert('f', F);
        h.insert('g', G);
        h
    };


    let mut result = 0;
    for c in s.chars() {
        result |= mask_map[&c];
    }
    result
}

