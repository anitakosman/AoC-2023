use std::fs;
use regex::Regex;

struct Digit {
    name: String,
    value: u32,
}

pub fn run(){
    let input = fs::read_to_string("src/resources/input01").expect("Should have been able to read the file");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[test]
fn test() {
    let input = fs::read_to_string("src/resources/test01-1").expect("Should have been able to read the file");
    assert_eq!(part1(&input), 142);
    let input2 = fs::read_to_string("src/resources/test01-2").expect("Should have been able to read the file");
    assert_eq!(part2(&input2), 281);
}

fn part1(input: &String) -> u32 {
    let mut result = Vec::new();
    let re = Regex::new(r"\d").unwrap();
    for line in input.lines() {
        let mut matches = re.find_iter(line);
        let a = matches.next().unwrap().as_str().parse::<u32>().unwrap();
        let b = matches.last().and_then(|m| Some(m.as_str())).and_then(|s| Some(s.parse::<u32>().unwrap())).unwrap_or(a);
        result.push(a * 10 + b);
    }
    result.iter().sum()
}

fn part2(input: &String) -> u32 {
    let digits = vec![
        Digit { name: String::from("one"), value: 1 },
        Digit { name: String::from("two"), value: 2 },
        Digit { name: String::from("three"), value: 3 },
        Digit { name: String::from("four"), value: 4 },
        Digit { name: String::from("five"), value: 5 },
        Digit { name: String::from("six"), value: 6 },
        Digit { name: String::from("seven"), value: 7 },
        Digit { name: String::from("eight"), value: 8 },
        Digit { name: String::from("nine"), value: 9 },
    ];

    let mut result = Vec::new();
    let all_digits = digits.iter().map(|d| d.name.as_str()).collect::<Vec<&str>>().join("|");
    let r = Regex::new(&*format!(r"\d|{}", all_digits)).unwrap();
    let r2 = Regex::new(&*format!(r"\d|{}", reverse(all_digits))).unwrap();
    for line in input.lines() {
        let m1 = r.find(line).unwrap().as_str();
        let reversed_line = &*reverse(String::from(line));
        let m2 = reverse(String::from(r2.find(reversed_line).unwrap().as_str()));
        let a = digits.iter().find(|d| d.name == m1 || d.value.to_string() == m1).unwrap_or_else(|| {println!("{}", line); panic!("")}).value;
        let b = digits.iter().find(|d| d.name == m2 || d.value.to_string() == m2).unwrap_or_else(|| {println!("{}", line); panic!("")}).value;
        result.push(a * 10 + b);
    }
    result.iter().sum()
}

fn reverse(s: String) -> String {
    s.chars().rev().collect()
}