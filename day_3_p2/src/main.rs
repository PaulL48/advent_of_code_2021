use std::str;

const VALUE_LENGTH: usize = 12;

enum Criteria {
    MostCommon,
    LeastCommon,
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();

    // convert the list of strings into a list of char arrays
    let mut processed_lines: Vec<[char; VALUE_LENGTH]> = Vec::new();
    for line in input.lines() {
        let mut b: [char; VALUE_LENGTH] = [' '; VALUE_LENGTH];
        for (i, c) in line.chars().enumerate() {
            b[i] = c;
        }
        processed_lines.push(b);
    }
    let mut co2_list = processed_lines.clone();
    let mut o2_list = processed_lines;

    let mut i = 0;
    while o2_list.len() != 1 {
        o2_list = filter_set(o2_list, i, Criteria::MostCommon);
        i += 1;
    }

    let mut i = 0;
    while co2_list.len() != 1 {
        co2_list = filter_set(co2_list, i, Criteria::LeastCommon);
        i += 1;
    }

    let oxygen_rating: String = o2_list[0].iter().collect();
    let oxygen_rating: usize = usize::from_str_radix(&oxygen_rating, 2).unwrap();

    let co2_scrubber_rating: String = co2_list[0].iter().collect();
    let co2_scrubber_rating: usize = usize::from_str_radix(&co2_scrubber_rating, 2).unwrap();

    println!(
        "O2 rating: {}; CO2 Scrubber Rating: {}; Product: {}",
        oxygen_rating,
        co2_scrubber_rating,
        oxygen_rating * co2_scrubber_rating
    );
}

fn filter_set(
    set: Vec<[char; VALUE_LENGTH]>,
    index: usize,
    criteria: Criteria,
) -> Vec<[char; VALUE_LENGTH]> {
    let (on_tie, one_most_common, zero_most_common) = match criteria {
        Criteria::MostCommon => ('1', '1', '0'),
        Criteria::LeastCommon => ('0', '0', '1'),
    };

    let mut count = 0;
    for line in &set {
        if line[index] == '1' {
            count += 1;
        } else {
            count -= 1;
        }
    }

    let keep_on = if count < 0 {
        zero_most_common
    } else if count == 0 {
        on_tie
    } else {
        one_most_common
    };

    set.iter()
        .copied()
        .filter(|v| v[index] == keep_on)
        .collect()
}
