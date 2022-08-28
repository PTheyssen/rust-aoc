use std::fs;
use nom::{
  IResult,
  bytes::complete::{tag},
  combinator::map_res,
  multi::{many0, many1},
  combinator::recognize,
  sequence::terminated,
  character::complete::{char, one_of},
};
use std::collections::HashSet;


#[derive(Debug,PartialEq)]
pub struct Claim {
    pub left_edge:   i32,
    pub top_edge:    i32,
    pub width:       i32,
    pub height:      i32,
}

fn main () {
    task1();
}

fn task1() {
    let contents = fs::read_to_string("input/day3.txt")
        .expect("Failed reading input");

    let claims: Vec<&str> = contents.split_terminator('\n')
        .collect();

    let mut seen: HashSet<(i32,i32)> = HashSet::new();
    let mut seen_twice: HashSet<(i32,i32)> = HashSet::new();

    // To count how many square inches are overlapping
    // we use two sets,
    // seen_set -> every pair goes in here
    // seen_twice -> pair only if in seen_set
    // size of seen_twice is solution
    for claim in claims {
        let parse_result = parse_claim(claim);
        for p in generate_pairs(parse_result.unwrap().1) {
            if seen.contains(&p) {
                seen_twice.insert(p);
            } else {
                seen.insert(p);
            }
        }
    }
    println!("Solution day3, task1: {}", seen_twice.len());
}

fn generate_pairs(claim: Claim) -> Vec<(i32, i32)> {
    let mut pairs: Vec<(i32, i32)> = Vec::new();

    let start_pair: (i32, i32) = (claim.left_edge, claim.top_edge);
    for i in 0..claim.width {
        for j in 0..claim.height {
            pairs.push((start_pair.0 + i, start_pair.1 + j))
        }
    }
    pairs
}

/// Parses a claim using the nom parser combinator library.
///
/// For example consider the claim:  "#1290 @ 608,593: 17x20"
/// We want to return the struct
/// Claim {left_edge = 608, top_edge = 593, width = 17, height = 20}
fn parse_claim(input: &str) -> IResult<&str, Claim> {
    let (input, _) = tag("#")(input)?;
    let (input, _claim_id) = map_res(
        recognize(
            many1(
                terminated(one_of("0123456789"), many0(char('_')))
            )),
        |out: &str| i32::from_str_radix(out, 10)
    )(input)?;
    let (input, _) = tag(" @ ")(input)?;
    let (input, left_edge) = map_res(
        recognize(
            many1(
                terminated(one_of("0123456789"), many0(char('_')))
            )),
        |out: &str| i32::from_str_radix(out, 10)
    )(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, top_edge) = map_res(
        recognize(
            many1(
                terminated(one_of("0123456789"), many0(char('_')))
            )),
        |out: &str| i32::from_str_radix(out, 10)
    )(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, width) = map_res(
        recognize(
            many1(
                terminated(one_of("0123456789"), many0(char('_')))
            )),
        |out: &str| i32::from_str_radix(out, 10)
    )(input)?;
    let (input, _) = tag("x")(input)?;
    let (input, height) = map_res(
        recognize(
            many1(
                terminated(one_of("0123456789"), many0(char('_')))
            )),
        |out: &str| i32::from_str_radix(out, 10)
    )(input)?;

    Ok((input, Claim {left_edge, top_edge, width, height}))
}


fn test_parsing() {
    // Do parsing
    let claim1 = "#1290 @ 608,593: 17x20";
    let parsed_claim1 = parse_claim(claim1);
    println!("claims:\n{:?}", parsed_claim1);

    println!("pairs:\n{:?}", generate_pairs(parsed_claim1.unwrap().1));
}
