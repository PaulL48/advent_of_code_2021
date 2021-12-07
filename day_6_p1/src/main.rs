const INPUT: [usize; 300] = [
    5, 4, 3, 5, 1, 1, 2, 1, 2, 1, 3, 2, 3, 4, 5, 1, 2, 4, 3, 2, 5, 1, 4, 2, 1, 1, 2, 5, 4, 4, 4, 1,
    5, 4, 5, 2, 1, 2, 5, 5, 4, 1, 3, 1, 4, 2, 4, 2, 5, 1, 3, 5, 3, 2, 3, 1, 1, 4, 5, 2, 4, 3, 1, 5,
    5, 1, 3, 1, 3, 2, 2, 4, 1, 3, 4, 3, 3, 4, 1, 3, 4, 3, 4, 5, 2, 1, 1, 1, 4, 5, 5, 1, 1, 3, 2, 4,
    1, 2, 2, 2, 4, 1, 2, 5, 5, 1, 4, 5, 2, 4, 2, 1, 5, 4, 1, 3, 4, 1, 2, 3, 1, 5, 1, 3, 4, 5, 4, 1,
    4, 3, 3, 3, 5, 5, 1, 1, 5, 1, 5, 5, 1, 5, 2, 1, 5, 1, 2, 3, 5, 5, 1, 3, 3, 1, 5, 3, 4, 3, 4, 3,
    2, 5, 2, 1, 2, 5, 1, 1, 1, 1, 5, 1, 1, 4, 3, 3, 5, 1, 1, 1, 4, 4, 1, 3, 3, 5, 5, 4, 3, 2, 1, 2,
    2, 3, 4, 1, 5, 4, 3, 1, 1, 5, 1, 4, 2, 3, 2, 2, 3, 4, 1, 3, 4, 1, 4, 3, 4, 3, 1, 3, 3, 1, 1, 4,
    1, 1, 1, 4, 5, 3, 1, 1, 2, 5, 2, 5, 1, 5, 3, 3, 1, 3, 5, 5, 1, 5, 4, 3, 1, 5, 1, 1, 5, 5, 1, 1,
    2, 5, 5, 5, 1, 1, 3, 2, 2, 3, 4, 5, 5, 2, 5, 4, 2, 1, 5, 1, 4, 4, 5, 4, 4, 1, 2, 1, 1, 2, 3, 5,
    5, 1, 3, 1, 4, 2, 3, 3, 1, 4, 1, 1,
];
const INITIAL_TIMERS: [u128; 9] = initialize(INPUT);
const SUM: u128 = process(INITIAL_TIMERS);

fn main() {
    println!("Sum of fish: {}", SUM);
}

const fn initialize(inputs: [usize; 300]) -> [u128; 9] {
    let mut i = 0;
    let mut positions = [0u128; 9];
    while i < 300 {
        positions[inputs[i]] += 1;
        i += 1;
    }
    positions
}

const fn process(mut positions: [u128; 9]) -> u128 {
    let mut i = 0;
    while i < 80 {
        let position_zero = positions[0];
        positions[0] = positions[1];
        positions[1] = positions[2];
        positions[2] = positions[3];
        positions[3] = positions[4];
        positions[4] = positions[5];
        positions[5] = positions[6];
        positions[6] = position_zero + positions[7];
        positions[7] = positions[8];
        positions[8] = position_zero;
        i += 1;
    }
    positions[0]
        + positions[1]
        + positions[2]
        + positions[3]
        + positions[4]
        + positions[5]
        + positions[6]
        + positions[7]
        + positions[8]
}
