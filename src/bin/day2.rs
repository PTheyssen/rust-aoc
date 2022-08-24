use std::fs;
use itertools::Itertools;

fn main () {
    let contents = fs::read_to_string("input/day2.txt")
        .expect("Failed reading input");

    let v: Vec<&str> = contents.split('\n')
        .collect();

    let it = (1..5).combinations(3);
    // println!("v:\n{:?}", v);
    println!("it:\n{:?}", it);
}
