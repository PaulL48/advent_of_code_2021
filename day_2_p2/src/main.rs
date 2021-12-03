use std::str;

fn main() {
    let input =
        str::from_utf8(include_bytes!("../input/input.txt")).expect("File is not valid UTF-8");

    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for line in input.lines() {
        let mut pair = line.split(" ");
        let direction = pair.next().unwrap();
        let value = pair.next().unwrap().parse::<i32>().expect("File contains invalid integers");

        match direction {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            },
            "down" => aim += value,
            "up" => aim -= value,
            other => panic!("Invalid direction: {}", other),
        }
    }

    println!("depth {}; horizontal {}; multiplied: {}", depth, horizontal, depth * horizontal);
}
