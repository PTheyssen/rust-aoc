use std::fs;
use nom::{
  IResult,
  bytes::complete::{tag, take_while_m_n},
  combinator::map_res,
  multi::{many0, many1},
  combinator::recognize,
  sequence::terminated,
  character::complete::{char, one_of},
};

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

    println!("claims:\n{:?}", claims);

    // Do parsing
    let claim1 = "#1290 @ 608,593: 17x20";
    println!("claims:\n{:?}", parse_claim(claim1));

    // how many square inches are overlapping

    // use two sets,
    // seen_set -> every pair goes in here
    // seen_twice -> pair only if in seen_set
    // size of seen_twice is solution

}



// need to parse claims and generate pairs of indexes (positions)
// ex:  "#1290 @ 608,593: 17x20"
fn parse_claim(input: &str) -> IResult<&str, i32> {
    let (input, _) = tag("#")(input)?;
    let (input, claim_id) = map_res(
        recognize(
            many1(
                terminated(one_of("0123456789"), many0(char('_')))
            )),
        |out: &str| i32::from_str_radix(out, 10)
    )(input)?;
    // let (
    Ok((input, claim_id))
}
