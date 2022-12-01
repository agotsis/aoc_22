use aoc_helper::{AocDay, Puzzle};
use array2d::Array2D;
use itertools::Itertools;
use std::env;

const DAY: u8 = DATEHERE;

// struct S {
//     x: i32,
//     y: i32,
// }

// enum E {
//     A(String)
// }

// use S as Parsed;
type Parsed = String;

fn parse(inp: String) -> Parsed {
    inp
}

fn one(inp: Parsed) -> u32 {
    return 0;
}

fn two(inp: Parsed) -> u32 {
    return 0;
}

fn main() {
    dotenv::dotenv().expect("Failed to load .env");

    let mut day = AocDay::new_with_serializer(2022, DAY, parse)
        .with_session_id(env::var("AOC_SESSION_ID").unwrap().as_str());

    let ex = [r""];

    let ps = vec![
        Puzzle::new(1, one).with_examples(&ex),
        Puzzle::new(2, two).with_examples(&ex),
    ];

    for p in ps {
        day.test(&p);
        day.run(&p).unwrap();
    }
}
