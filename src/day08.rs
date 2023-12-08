use num::integer;
use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("src/resources/input08").expect("Should have been able to read the file");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[test]
fn test() {
    let input1 = fs::read_to_string("src/resources/test08-1").expect("Should have been able to read the file");
    let input2 = fs::read_to_string("src/resources/test08-2").expect("Should have been able to read the file");
    let input3 = fs::read_to_string("src/resources/test08-3").expect("Should have been able to read the file");
    assert_eq!(part1(&input1), 2);
    assert_eq!(part1(&input2), 6);
    assert_eq!(part2(&input3), 6);
}

fn part1(input: &String) -> u64 {
    find_number_of_steps(input, |s| { s == "AAA" }, |s| { s == "ZZZ" })
}

fn part2(input: &String) -> u64 {
    find_number_of_steps(input, |s| { s.ends_with("A") }, |s| { s.ends_with("Z") })
}

fn find_number_of_steps(input: &String, is_start: fn(&str) -> bool, is_end: fn(&str) -> bool) -> u64 {
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    let n = directions.len();
    lines.next();

    let maps: HashMap<&str, (&str, &str)> = lines.map(|line| {
        (line.get(0..3).unwrap(), (line.get(7..10).unwrap(), line.get(12..15).unwrap()))
    }).collect();

    let start_positions: Vec<&str> = maps.keys().filter_map(|k| if is_start(*k) { Some(*k) } else {None} ).collect();
    start_positions.iter().map(|start| {
        let mut i = 0;
        let mut current = *start;
        while !is_end(current) {
            let j = i % n;
            current = match directions.chars().nth(j) {
                Some('R') => maps.get(current).unwrap().1,
                Some('L') => maps.get(current).unwrap().0,
                _ => current
            };
            i += 1;
        }
        i as u64
    }).reduce(|x, y| {integer::lcm(x,y) }).unwrap()
}
