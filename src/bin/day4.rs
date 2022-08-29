use std::fs;
use nom::{
  IResult,
  bytes::complete::{tag},
  combinator::map_res,
  character::complete::{digit1},
};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Action {
    WakeUp,
    FallAsleep,
    NewShift,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Record {
    pub datetime: NaiveDateTime,
    pub action: Action,
    pub guard: u32,
}

fn main () {
    task1();
    test_parsing();
}

fn task1() {
    let contents = fs::read_to_string("input/day4.txt")
        .expect("Failed reading input");

    let records: Vec<&str> = contents.split_terminator('\n')
        .collect();

    let mut parsed_records: Vec<Record> = records.iter()
        .map(|r| parse_record(r).unwrap().1)
        .collect();
    parsed_records.sort();

    
    println!("records:\n{:?}", parsed_records);
}

fn parse_uint(input: &str) -> IResult<&str, u32> {
  map_res(digit1, str::parse)(input)
}

fn parse_int(input: &str) -> IResult<&str, i32> {
  map_res(digit1, str::parse)(input)
}

/// Parses a date using the nom parser combinator library.
///
/// Example: [1518-09-19 00:42] wakes up
fn parse_record(input: &str) -> IResult<&str, Record> {
    let (input, _) = tag("[")(input)?;
    let (input, year) = parse_int(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, month) = parse_uint(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, day) = parse_uint(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, hours) = parse_uint(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, minutes) = parse_uint(input)?;
    let (input, _) = tag("]")(input)?;
    let mut action: Action = Action::WakeUp;
    let mut guard: u32 = 0;

    if input.contains("wakes") {
        action = Action::WakeUp;
    } else if input.contains("falls") {
        action = Action::FallAsleep;
    } else {
        let (input, _) = tag(" Guard #")(input)?;
        let (_input, guard_id) = parse_uint(input)?;
        action = Action::NewShift;
        guard = guard_id;
    }

    let datetime: NaiveDateTime = NaiveDate::from_ymd(year, month, day)
        .and_hms(hours, minutes, 0);
    Ok((input, Record {datetime, action, guard}))
}


fn test_parsing() {
    // Do parsing
    let date1 = "[1518-09-19 00:42] wakes up";
    let parsed_date1 = parse_record(date1);
    println!("date1:\n{:?}", parsed_date1);

    // println!("pairs:\n{:?}", generate_pairs(parsed_claim1.unwrap().1));
}
