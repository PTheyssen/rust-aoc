use std::fs;

fn main () {
    let contents = fs::read_to_string("input")
        .expect("Failed reading input");

    let v: Vec<i32> =  contents.split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("v:\n{:?}", v);
    println!("v:\n{}", v.iter().sum::<i32>());    
}
