use std::fs;
use std::collections::HashSet;

fn main() {
    task1();
    task2();
}

fn task1() {
    let contents = fs::read_to_string("input/day1.txt")
        .expect("Failed reading input");

    let v: Vec<i32> = contents
        .split_terminator('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // println!("v:\n{:?}", v);
    println!("Solution day1, task1: {}", v.iter().sum::<i32>());
}

fn task2() {
    let mut seen = HashSet::new();
    seen.insert(0);

    let contents = fs::read_to_string("input/day1.txt")
        .expect("Failed reading input");

    let v: Vec<i32> = contents
        .split_terminator('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut counter = 0;
    'outer: loop {
        for x in v.iter() {
            counter = counter + x;
            if seen.contains(&counter) {
                println!("Solution day1, task2: {}", counter);
                break 'outer;
            }
            seen.insert(counter);
        }
    }
}
