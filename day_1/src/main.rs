use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let inputs = env::args().collect::<Vec<_>>();
    if inputs.len() < 2 {
        println!("Supply puzzle input file as first parameter");
        std::process::exit(1);
    }

    let mut input_file = match File::open(&inputs[1]) {
        Ok(file) => file,
        Err(err) => {
            println!("Could not open input file \"{}\": {}", inputs[1], err);
            std::process::exit(1);
        }
    };

    let mut input_contents = String::new();
    if let Err(err) = input_file.read_to_string(&mut input_contents) {
        println!("Could not read input file \"{}\": {}", inputs[1], err);
        std::process::exit(1);
    }

    let sum: usize = input_contents
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
