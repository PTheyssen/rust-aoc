use std::fs;
use std::collections::HashSet;

fn main () {
    let mut seen = HashSet::new();
    seen.insert(0);

    let contents = fs::read_to_string("input")
        .expect("Failed reading input");

    let v: Vec<i32> =  contents.split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut counter = 0;
    'outer: loop {
        for x in v.iter() {
            counter = counter + x;
            // println!("Counter: {}\n", counter);
            if seen.contains(&counter) {
                println!("Solution is: {}", counter);
                break 'outer;
            }
            seen.insert(counter);
        }
    }
}
