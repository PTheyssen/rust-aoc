use std::fs;
use std::iter::zip;
use itertools::Itertools;

fn main () {
    task1()
}

fn task1() {
    let contents = fs::read_to_string("input/day2.txt")
        .expect("Failed reading input");

    let v: Vec<&str> = contents.split_terminator('\n')
        .collect();
    let pairs: Vec<Vec<&&str>> = v.iter().combinations(2).collect();

    let result: Vec<i32> = pairs.iter()
        .map(|p| compute_char_distance(p.to_vec()))
        .collect();
}

fn compute_char_distance(p: Vec<&&str>) -> i32 {
    let s1 = p[0];
    let s2 = p[1];

    let mut diffs = 0;
    for (c1, c2) in zip(s1.chars(), s2.chars()) {
        if c1 != c2 {
            diffs += 1;
        }
    }
    if diffs == 1 {
        println!("solution: {}, {}", s1, s2);
        // solution: cvgywxqubnuaefmsldjdrpfzyi, cvgywxqubnuaefmslkjdrpfzyi
        // -> cvgywxqubnuaefmsljdrpfzyi
    }
    diffs
}
