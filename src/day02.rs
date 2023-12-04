use std::cmp::max;
use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("resources/input02").expect("Should have been able to read the file");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[test]
fn test() {
    let input = fs::read_to_string("resources/test02").expect("Should have been able to read the file");
    assert_eq!(part1(&input), 8);
    assert_eq!(part2(&input), 2286);
}

fn part1(input: &String) -> u32 {
    let available_cubes = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    input.lines().enumerate().map(|(game, line)| {
        if line.split(": ").nth(1).unwrap_or_default().split("; ").all(|set| {
            set.split(", ").all(|cubes| {
                let mut split = cubes.split(" ");
                let n = split.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or_default();
                let color = split.next().unwrap_or_default();
                available_cubes.get(color).unwrap_or(&0) >= &n
            })
        }) { game as u32 + 1 } else { 0 }
    }).sum()
}

fn part2(input: &String) -> u32 {
    input.lines().map(|line| {
        let mut cubes_per_color = HashMap::new();
        for set in line.split(": ").nth(1).unwrap_or_default().split("; ") {
            for cubes in set.split(", ") {
                let mut split = cubes.split(" ");
                let n = split.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or_default();
                let color = split.next().unwrap_or_default();
                let current_max = cubes_per_color.get(color).unwrap_or(&0);
                cubes_per_color.insert(color, max(*current_max, n));
            }
        }
        cubes_per_color.values().product::<u32>()
    }).sum()
}