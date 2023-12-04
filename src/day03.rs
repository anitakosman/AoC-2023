use std::fs;
use regex::Regex;

pub fn run() {
    let input = fs::read_to_string("resources/input03").expect("Should have been able to read the file");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[test]
fn test() {
    let input = fs::read_to_string("resources/test03").expect("Should have been able to read the file");
    assert_eq!(part1(&input), 4361);
    assert_eq!(part2(&input), 467835);
}

#[derive(PartialEq)]
struct Pos(i32, i32);

#[derive(PartialEq)]
struct PartNumberPos(u32, i32, i32, i32);

fn part1(input: &String) -> u32 {
    let regex = Regex::new(r"\d+").unwrap();
    let symbol_positions = get_symbol_positions(input);
    let mut result = 0;
    for (y, line) in input.lines().enumerate() {
        for m in regex.find_iter(line) {
            let start = m.start() as i32;
            let end = m.end() as i32;
            if has_symbol_as_neighbor(start, end, y as i32, &symbol_positions) {
                result += m.as_str().parse::<u32>().unwrap();
            }
        }
    }
    result
}

fn part2(input: &String) -> u32 {
    let part_number_positions = get_part_number_positions(input);
    let mut result = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                let neighboring_part_numbers = get_neighboring_part_numbers(x as i32, y as i32, &part_number_positions);
                if neighboring_part_numbers.len() == 2 {
                    result += neighboring_part_numbers.iter().product::<u32>();
                }
            }
        }
    }
    result
}

fn get_symbol_positions(input: &String) -> Vec<Pos> {
    let mut positions = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' && !c.is_digit(10) {
                positions.push(Pos(x as i32, y as i32));
            }
        }
    }
    positions
}

fn has_symbol_as_neighbor(start: i32, end: i32, y: i32, symbol_positions: &Vec<Pos>) -> bool {
    (start - 1..end + 1).any(|x| { symbol_positions.contains(&Pos(x, y - 1)) })
        || (start - 1..end + 1).any(|x| { symbol_positions.contains(&Pos(x, y)) })
        || (start - 1..end + 1).any(|x| { symbol_positions.contains(&Pos(x, y + 1)) })
}

fn get_part_number_positions(input: &String) -> Vec<PartNumberPos> {
    let regex = Regex::new(r"\d+").unwrap();
    let mut positions = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for m in regex.find_iter(line) {
            positions.push(PartNumberPos(m.as_str().parse::<u32>().unwrap(), y as i32, m.start() as i32, m.end() as i32))
        }
    }
    positions
}

fn get_neighboring_part_numbers(x: i32, y: i32, part_number_positions: &Vec<PartNumberPos>) -> Vec<u32> {
    part_number_positions.iter().filter_map(|PartNumberPos(value, part_y, start, end)| {
        if (part_y - 1 <= y && y <= part_y + 1) && (start - 1 <= x && x <= *end) { Some(*value) } else { None }
    }).collect()
}
