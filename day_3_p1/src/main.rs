use std::str;

fn main() {
    let input =
        str::from_utf8(include_bytes!("../input/input.txt")).expect("File is not valid UTF-8");
    let mut counts: [i32; 12] = [0; 12];

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                counts[i] += 1;
            } else {
                counts[i] -= 1;
            }
        }
    }

    let bitmask = 0b111111111111usize;
    let gamma = binary_array_to_integral(&counts) as usize;
    let epsilon = !gamma & bitmask;

    println!("gamma: {}; epsilon: {}; product: {}", gamma, epsilon, gamma.saturating_mul(epsilon));
}

fn binary_array_to_integral(array: &[i32]) -> i32 {
    let mut value = 0;
    let mut radix = 1;
    for digit in array.iter().map(|v| v.clamp(&0, &1)).rev() {
        value += *digit * radix;
        radix *= 2;
    }
    value
}
