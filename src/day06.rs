use std::fs;

pub fn run() {
    let input = fs::read_to_string("src/resources/input06").expect("Should have been able to read the file");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[test]
fn test() {
    let input = fs::read_to_string("src/resources/test06").expect("Should have been able to read the file");
    assert_eq!(part1(&input), 288);
    assert_eq!(part2(&input), 71503);
}

fn part1(input: &String) -> u64 {
    let mut lines = input.lines();
    let ts = get_numbers_from_line(lines.next().unwrap());
    let ds = get_numbers_from_line(lines.next().unwrap());
    let races = ts.iter().zip(ds.iter());
    races.map(|(t, d)| {
        let range = get_win_range(*t, *d);
        range
    }).product()
}

fn part2(input: &String) -> u64 {
    let mut lines = input.lines();
    let ts = get_numbers_from_line(lines.next().unwrap()
        .strip_prefix("Time: ").unwrap()
        .replace(" ", "").as_str());
    let ds = get_numbers_from_line(lines.next().unwrap()
        .strip_prefix("Distance: ").unwrap()
        .replace(" ", "").as_str());
    get_win_range(*ts.get(0).unwrap(), *ds.get(0).unwrap())
}

fn get_win_range(t: u64, d: u64) -> u64 {
    let sqrt = ((t * t - 4 * d) as f64).sqrt();
    let max_x = (((t as f64 + sqrt) / 2.0) - 1.0).ceil() as u64;
    let min_x = (((t as f64 - sqrt) / 2.0) + 1.0).floor() as u64;
    max_x - min_x + 1
}

fn get_numbers_from_line(line: &str) -> Vec<u64> {
    line.split(" ")
        .filter_map(|s| { s.parse::<u64>().ok() })
        .collect()
}
