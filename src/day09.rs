use std::fs;
use itertools::Itertools;
use crate::util::get_numbers_from_line;

pub fn run() {
    let input = fs::read_to_string("src/resources/input09").expect("Should have been able to read the file");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[test]
fn test() {
    let input = fs::read_to_string("src/resources/test09").expect("Should have been able to read the file");
    assert_eq!(part1(&input), 114);
    assert_eq!(part2(&input), 2);
}

fn part1(input: &String) -> i32 {
    input.lines().map(|line| { get_numbers_from_line::<i32>(line) }).map(calculate_next).sum()
}

fn part2(input: &String) -> i32 {
    input.lines().map(|line| { get_numbers_from_line::<i32>(line) }).map(calculate_previous).sum()
}

fn calculate_next(v: Vec<i32>) -> i32 {
    if v.is_empty() {
        0
    } else if v.iter().all_equal() {
        v[0]
    } else {
        v.last().unwrap() + calculate_next(v.windows(2).map(|w| w[1] - w[0]).collect())
    }
}

fn calculate_previous(v: Vec<i32>) -> i32 {
    if v.iter().all_equal() {
        v[0]
    } else {
        v[0] - calculate_previous(v.windows(2).map(|w| w[1] - w[0]).collect())
    }
}