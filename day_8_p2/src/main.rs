use std::collections::{HashMap, HashSet};
use std::str;

use std::ops::{BitAnd, Sub};

const DA: Digit = Digit::new(0b00000001);
const DB: Digit = Digit::new(0b00000010);
const DC: Digit = Digit::new(0b00000100);
const DD: Digit = Digit::new(0b00001000);
const DE: Digit = Digit::new(0b00010000);
const DF: Digit = Digit::new(0b00100000);
const DG: Digit = Digit::new(0b01000000);

const ALL_COMPONENTS: [Digit; 7] = [DA, DB, DC, DD, DE, DF, DG];

const A: u8 = 0b00000001;
const B: u8 = 0b00000010;
const C: u8 = 0b00000100;
const D: u8 = 0b00001000;
const E: u8 = 0b00010000;
const F: u8 = 0b00100000;
const G: u8 = 0b01000000;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Segment {
    Top,
    TopLeft,
    BottomLeft,
    Bottom,
    BottomRight,
    TopRight,
    Middle,
}

use lazy_static::lazy_static;
use maplit::hashset;

lazy_static! {
    static ref ZERO: HashSet<Segment> = hashset! {
        Segment::Top,
        Segment::TopLeft,
        Segment::TopRight,
        Segment::BottomLeft,
        Segment::BottomRight,
        Segment::Bottom,
    };
    static ref ONE: HashSet<Segment> = hashset! {
        Segment::TopRight,
        Segment::BottomRight
    };
    static ref TWO: HashSet<Segment> = hashset! {
        Segment::Top,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomLeft,
        Segment::Bottom,
    };
    static ref THREE: HashSet<Segment> = hashset! {
        Segment::Top,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomRight,
        Segment::Bottom,
    };
    static ref FOUR: HashSet<Segment> = hashset! {
        Segment::TopRight,
        Segment::TopLeft,
        Segment::Middle,
        Segment::BottomRight,
    };
    static ref FIVE: HashSet<Segment> = hashset! {
        Segment::Top,
        Segment::TopLeft,
        Segment::Middle,
        Segment::BottomRight,
        Segment::Bottom,
    };
    static ref SIX: HashSet<Segment> = hashset! {
        Segment::Top,
        Segment::TopLeft,
        Segment::Middle,
        Segment::BottomLeft,
        Segment::BottomRight,
        Segment::Bottom,
    };
    static ref SEVEN: HashSet<Segment> = hashset! {
        Segment::Top,
        Segment::TopRight,
        Segment::BottomRight,
    };
    static ref EIGHT: HashSet<Segment> = hashset! {
        Segment::Top,
        Segment::TopLeft,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomLeft,
        Segment::BottomRight,
        Segment::Bottom,
    };
    static ref NINE: HashSet<Segment> = hashset! {
        Segment::Top,
        Segment::TopLeft,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomRight,
        Segment::Bottom,
    };
}

#[derive(Clone, Copy, Debug)]
struct Digit {
    pub components: u8,
}

impl Digit {
    const fn new(d: u8) -> Self {
        Self { components: d }
    }

    fn len(&self) -> u32 {
        self.components.count_ones()
    }

    fn decode(&self, decoder: &HashMap<u8, Segment>) -> u32 {
        println!("{:?}", decoder);
        let mut segments = HashSet::new();
        for component in ALL_COMPONENTS {
            if self.contains(&component) {
                segments.insert(decoder[&component.components]);
            }
        }
        println!("segs {:?}", segments);
        if segments == *ZERO {
            return 0;
        } else if segments == *ONE {
            return 1;
        } else if segments == *TWO {
            return 2;
        } else if segments == *THREE {
            return 3;
        } else if segments == *FOUR {
            return 4;
        } else if segments == *FIVE {
            return 5;
        } else if segments == *SIX {
            return 6;
        } else if segments == *SEVEN {
            return 7;
        } else if segments == *EIGHT {
            return 8;
        } else if segments == *NINE {
            return 9;
        }

        panic!();
    }

    fn contains(&self, d: &Digit) -> bool {
        self.components & d.components == d.components
    }
}

impl BitAnd for Digit {
    type Output = Self;
    fn bitand(self, rhs: Digit) -> Self {
        Self {
            components: self.components & rhs.components,
        }
    }
}

impl Sub for Digit {
    type Output = Self;
    fn sub(self, rhs: Digit) -> Self {
        Self {
            components: self.components & !rhs.components,
        }
    }
}

impl From<&str> for Digit {
    fn from(input: &str) -> Self {
        Self {
            components: chars_to_maskmap(input),
        }
    }
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let mut iter = line.split(" | ");

        let encoding = iter.next().unwrap().split(' ').map(Digit::from);
        let decoder = get_mapping(encoding);

        let digits = iter
            .next()
            .unwrap()
            .split(' ')
            .map(|d| Digit::from(d).decode(&decoder))
            .collect::<Vec<_>>();
        let mut real_digits = 0;
        let mut radix = 1;
        for d in digits.iter().rev() {
            real_digits += d * radix;
            radix *= 10;
        }

        sum += real_digits;
    }

    println!("{}", sum);
}

fn get_mapping<T: Iterator<Item = Digit> + Clone>(digits: T) -> HashMap<u8, Segment> {
    // get the known values and divide the rest into their groups
    let mut mappings = HashMap::new();
    let mut len_six = Vec::with_capacity(3);
    for digit in digits {
        match digit.len() {
            2 => {
                mappings.insert(1, digit);
            }
            3 => {
                mappings.insert(7, digit);
            }
            4 => {
                mappings.insert(4, digit);
            }
            6 => len_six.push(digit),
            7 => {
                mappings.insert(8, digit);
            }
            _ => (),
        }
    }
    let len_6_intersect = len_six.iter().cloned().reduce(|acc, d| acc & d).unwrap();
    let mut decoder: HashMap<u8, Segment> = HashMap::new();

    let br = mappings[&1] & len_6_intersect;
    let tl = (len_6_intersect - mappings[&7]) & mappings[&4];
    let bottom = (mappings[&8] - mappings[&4] - mappings[&7]) & len_6_intersect;

    decoder.insert(br.components, Segment::BottomRight);
    decoder.insert((mappings[&1] - br).components, Segment::TopRight);
    decoder.insert(tl.components, Segment::TopLeft);
    decoder.insert(
        (mappings[&4] - mappings[&1] - tl).components,
        Segment::Middle,
    );
    decoder.insert((mappings[&7] - mappings[&1]).components, Segment::Top);
    decoder.insert(bottom.components, Segment::Bottom);
    decoder.insert(
        (mappings[&8] - mappings[&4] - mappings[&7] - bottom).components,
        Segment::BottomLeft,
    );

    decoder
}

fn chars_to_maskmap(s: &str) -> u8 {
    let mask_map = {
        let mut h = HashMap::new();
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
