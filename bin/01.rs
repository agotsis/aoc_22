use std::convert::TryFrom;
use std::env;

use aoc_helper::{AocDay, Puzzle};

const DAY: u8 = 1;

fn one(x: String) -> u32 {
    let mut max: u32 = 0;

    for l in x.split("\n\n") {
        let num: Vec<u32> = l.split("\n").map(|x| x.parse::<u32>().unwrap()).collect();
        let sum = num.iter().sum();
        max = if sum > max {sum} else {max};
    }

    max
}

fn two(x: String) -> u32 {

    let mut sums: Vec<u32> = x.split("\n\n").map(|l| l.split("\n").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>().iter().sum()).collect();

    sums.sort();

    sums[sums.len() - 3 .. sums.len()].iter().sum()
}

fn main() {
    dotenv::dotenv().expect("Failed to load .env");

    let mut day =
        AocDay::new(2022, DAY).with_session_id(env::var("AOC_SESSION_ID").unwrap().as_str());

    let ps = vec![
        Puzzle::new(1, one),
        Puzzle::new(2, two),
    ];

    for p in ps {
        day.run(&p).unwrap();
    }
}
