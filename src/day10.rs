use std::fs;
use itertools::Itertools;
use crate::day10::Direction::{East, North, South, West};

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Clone)]
struct Position(usize, usize);

pub fn run() {
    let input = fs::read_to_string("src/resources/input10").expect("Should have been able to read the file");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[test]
fn test() {
    let input1 = fs::read_to_string("src/resources/test10-1").expect("Should have been able to read the file");
    let input2 = fs::read_to_string("src/resources/test10-2").expect("Should have been able to read the file");
    let input3 = fs::read_to_string("src/resources/test10-3").expect("Should have been able to read the file");
    let input4 = fs::read_to_string("src/resources/test10-4").expect("Should have been able to read the file");
    let input5 = fs::read_to_string("src/resources/test10-5").expect("Should have been able to read the file");
    assert_eq!(part1(&input1), 4);
    assert_eq!(part1(&input2), 8);
    assert_eq!(part2(&input3), 4);
    assert_eq!(part2(&input4), 8);
    assert_eq!(part2(&input5), 10);
}

fn part1(input: &String) -> usize {
    let (l, _) = find_loop(input);
    (l.len() + 1) / 2
}

fn part2(input: &String) -> usize {
    let (l, s) = find_loop(input);
    let loop_positions: Vec<Position> = l.iter().map(|(p, _)| p.clone()).collect();
    (0..input.lines().count()).map(|y| {
        (0..input.lines().next().unwrap().len()).filter(|x| {
            !loop_positions.contains(&Position(*x, y)) && is_horizontally_within_loop(&l, *x, y, s) && is_vertically_within_loop(&l, *x, y, s)
        }).count()
    }).sum()
}

fn find_loop(input: &String) -> (Vec<(Position, char)>, &str) {
    let lines: Vec<&str> = input.lines().collect();
    let s = find_s(&lines);
    let (mut p, mut d, starting_direction) = find_first_move(&lines, &s);


    let mut l: Vec<(Position, char)> = Vec::new();
    l.push((p.clone(), lines.get(p.1).unwrap().chars().nth(p.0).unwrap()));

    while p.0 != s.0 || p.1 != s.1 {
        p = match d {
            North => { Position(p.0, p.1 - 1) }
            East => { Position(p.0 + 1, p.1) }
            South => { Position(p.0, p.1 + 1) }
            West => { Position(p.0 - 1, p.1) }
        };
        let c = lines.get(p.1).unwrap().chars().nth(p.0).unwrap();
        d = match c {
            '7' => if d == North { West } else { South },
            'J' => if d == South { West } else { North },
            'F' => if d == North { East } else { South },
            'L' => if d == South { East } else { North },
            _ => d
        };
        l.push((p.clone(), lines.get(p.1).unwrap().chars().nth(p.0).unwrap()));
    }

    let s = find_pipe_piece(d, starting_direction);

    (l, s)
}

fn find_s(lines: &Vec<&str>) -> Position {
    let (y, line) = lines.iter().find_position(|line| line.contains("S")).unwrap();
    let x = line.find("S").unwrap();
    Position(x, y)
}

fn find_first_move(lines: &Vec<&str>, s: &Position) -> (Position, Direction, Direction) {
    if s.1 > 0 {
        let c = lines.get(s.1 - 1).unwrap().chars().nth(s.0).unwrap();
        if c == '|' {
            return (Position(s.0, s.1 - 1), North, North);
        } else if c == 'F' {
            return (Position(s.0, s.1 - 1), East, North);
        } else if c == '7' {
            return (Position(s.0, s.1 - 1), West, North);
        }
    }

    if s.0 > 0 {
        let c = lines.get(s.1).unwrap().chars().nth(s.0 - 1).unwrap();
        if c == '-' {
            return (Position(s.0 - 1, s.1), West, West);
        } else if c == 'F' {
            return (Position(s.0 - 1, s.1), South, West);
        } else if c == 'L' {
            return (Position(s.0 - 1, s.1), North, West);
        }
    }

    if s.0 + 1 < lines.get(0).unwrap().len() {
        let c = lines.get(s.1).unwrap().chars().nth(s.0 + 1).unwrap();
        if c == '-' {
            return (Position(s.0 + 1, s.1), East, East);
        } else if c == 'J' {
            return (Position(s.0 + 1, s.1), North, East);
        } else if c == '7' {
            return (Position(s.0 + 1, s.1), South, East);
        }
    }

    if s.1 + 1 < lines.len() {
        let c = lines.get(s.1 + 1).unwrap().chars().nth(s.0).unwrap();
        if c == '|' {
            return (Position(s.0, s.1 + 1), South, South);
        } else if c == 'J' {
            return (Position(s.0, s.1 + 1), West, South);
        } else if c == 'L' {
            return (Position(s.0, s.1 + 1), East, South);
        }
    }
    panic!("Could not determine start");
}

fn find_pipe_piece(from: Direction, to: Direction) -> &'static str {
    match (from, to) {
        (North, North) => { "|" },
        (North, West) => { "7" },
        (North, East) => { "F" },
        (South, South) => { "|" },
        (South, West) => { "J" },
        (South, East) => { "L" },
        (East, East) => { "-" },
        (East, North) => { "J" },
        (East, South) => { "7" },
        (West, West) => { "-" },
        (West, North) => { "L" },
        (West, South) => { "F" },
        (_, _) => "."
    }
}

fn is_horizontally_within_loop(l: &Vec<(Position, char)>, x: usize, y: usize, s: &str) -> bool {
    let horizontal_loop_boundaries = l.iter()
        .filter(|(Position(a, b), _)| {
            *a < x && *b == y
        })
        .sorted_by_key(|(Position(a, _), _)| a)
        .map(|(_, c)| c)
        .join("")
        .replace("S", s)
        .replace("-", "")
        .replace("FJ", "|")
        .replace("L7", "|")
        .len();
    horizontal_loop_boundaries % 2 == 1
}

fn is_vertically_within_loop(l: &Vec<(Position, char)>, x: usize, y: usize, s: &str) -> bool {
    let vertical_loop_boundaries = l.iter()
        .filter(|(Position(a, b), _)| {
            *a == x && *b < y
        })
        .sorted_by_key(|(Position(_, b), _)| b)
        .map(|(_, c)| c)
        .join("")
        .replace("S", s)
        .replace("|", "")
        .replace("FJ", "-")
        .replace("7L", "-")
        .len();
    vertical_loop_boundaries % 2 == 1
}
