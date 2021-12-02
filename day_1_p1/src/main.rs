use std::str;

fn main() {
    let input =
        str::from_utf8(include_bytes!("../input/input.txt")).expect("File is not valid UTF-8");

    let sum: usize = input
        .lines()
        .map(|line| {
            line.parse::<i32>()
                .expect("Input file contains lines that cannot be parsed as integers")
        })
        .collect::<Vec<_>>()
        .windows(2)
        .map(|values| (values[0] < values[1]) as usize)
        .sum();

    println!("{}", sum);
}
