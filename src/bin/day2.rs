use std::fs;
use std::iter::zip;
use itertools::Itertools;
use std::collections::HashMap;

fn main () {
    task1();
    task2();
}

fn task1() {
    let contents = fs::read_to_string("input/day2.txt")
        .expect("Failed reading input");

    let v: Vec<&str> =  contents.split('\n')
        .collect();

    let result: Vec<(i32, i32)> = v.iter().map(|s| count_times(s)).collect();

    println!("result:\n{:?}", result);

    let mut num_twos = 0;
    let mut num_threes = 0;
    for (t1, t2) in result {
        if t1 > 0 {
            num_twos += 1;
        }
        if t2 > 0 {
            num_threes += 1;
        }
    }
    println!("solution day2 task1:\n{}", num_twos * num_threes);
}

fn task2() {
    let contents = fs::read_to_string("input/day2.txt")
        .expect("Failed reading input");

    let v: Vec<&str> = contents.split_terminator('\n')
        .collect();
    let pairs: Vec<Vec<&&str>> = v.iter().combinations(2).collect();

    let _result: Vec<i32> = pairs.iter()
        .map(|p| compute_char_distance(p.to_vec()))
        .collect();
}

fn count_times(s:&str) -> (i32, i32) {
    // set up hashmap
    let alphabet = String::from_utf8(
        (b'a'..=b'z').collect()
    ).unwrap();
    let mut char_counts = HashMap::new();

    for c in alphabet.chars() {
        char_counts.insert(c, 0);
    }

    // count up each letter in string
    for c in s.chars() {
        let old_count = char_counts.get_mut(&c).unwrap();
        *old_count += 1;
    }
    println!("char_counts:\n{:?}", char_counts);

    let mut twos = 0;
    let mut threes = 0;
    for x in char_counts.values() {
        if *x == 2 {
            twos += 1;
        }
        else if *x == 3 {
            threes += 1;
        }
    }
    (twos, threes)
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
