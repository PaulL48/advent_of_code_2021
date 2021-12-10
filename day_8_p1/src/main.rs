use std::str;

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let mut iter = line.split(" | ");
        let _ = iter.next().unwrap();
        let display = iter.next().unwrap();

        for digit in display.split(" ") {
            if digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7 {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}
