use std::str;

fn apply(i: &[char; 2], counts: &mut HashMap<char, u128>, n: usize, productions: &HashMap<[char; 2], [char; 3]>) {
    if n == 0 {
        return;
    }

    let o = productions[i];
    // println!("{}: {:?} -> {:?}", n, i, o);
    *counts.get_mut(&o[1]).unwrap() += 1;

    let lhs = [o[0], o[1]];
    apply(&lhs, counts, n - 1, productions);

    let rhs = [o[1], o[2]];
    apply(&rhs, counts, n - 1, productions);
}

use std::collections::HashMap;

fn main() {
    let input = str::from_utf8(include_bytes!("../input/input.txt")).unwrap();
    
    let mut it = input.split("\n\n");
    let sequence = it.next().unwrap().chars().collect::<Vec<_>>();
    let productions = it.next()
        .unwrap()
        .lines()
        .map(|s| {
            let mut it = s.split(" -> ");
            let lhs = it.next().unwrap().chars().take(2).collect::<Vec<_>>();
            let rhs = it.next().unwrap().chars().next().unwrap();
            ([lhs[0], lhs[1]], [lhs[0], rhs, lhs[1]])
        })
        .collect::<HashMap<[char; 2], [char; 3]>>();
    // let mut counts = HashMap::new();


    let mut counts = HashMap::new();
    for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        counts.insert(c, 0);
    }

    for c in &sequence {
        *counts.get_mut(&c).unwrap() += 1;
    }

    for w in sequence.windows(2) {
        let start = [w[0], w[1]];
        apply(&start, &mut counts, 10, &productions);
    }

    // let countsa = counts.iter().filter(|(k, v)| **v != 0).collect::<HashMap<_,_>>();

    // println!("{:?}", countsa);

    let max = counts.iter().map(|(_, v)| v).max().unwrap();
    let min = counts.iter().map(|(_, v)| v).filter(|v| **v != 0).min().unwrap();

    println!("{}", max - min);
    /*
        IF the productions are deterministically cyclic,
        then we can build a table of the number 

    */


    // There is a strong relation between the rules and the number of symbols produced
    // in that 
    /*
        FV -> H 
        really means 
        FV -> FHV
        which then decomposes into
        FH -> P [FH -> FPH]
        and
        HV -> N [HV -> HNV]
    */

    /*
    so lets write out the rules correctly and consider a single starting FV
    FV -> FHV
        FH -> FPH
            FP -> FNP
            PH -> PPH
        HV -> HNV
            HN -> HPN
            NV -> NSV

    H: 1
    P: 3
    N: 2
    S: 1



    */

    /*

        if we subdivide the input into windows(2)
        then we can expand the doublets based on the relation of the rule and sum across all the windows(2)
    

        worst case I can windows(2) and brute force each rule
        and sum across those

        there is a SMALL problem with overcounting the starting symbols
        where if you count them in each windows(2) you'll count them n times
        but this can be solved by multiplying their counts by n - 1 and subtracting it from the totals

    */

    /*
        another potential solution is to recursively do the windows 2 thing
        (which is effectively what I'm doing anyways)
        then what you can do is only add to the count of the single letter it produces
        while still creating the string
    */


    println!("Hello, world!");
}
