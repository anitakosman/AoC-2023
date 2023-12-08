use std::fs;
use itertools::Itertools;

pub fn run() {
    let input = fs::read_to_string("src/resources/input07").expect("Should have been able to read the file");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[test]
fn test() {
    let input = fs::read_to_string("src/resources/test07").expect("Should have been able to read the file");
    assert_eq!(part1(&input), 6440);
    assert_eq!(part2(&input), 5905);
}

fn part1(input: &String) -> u64 {
    get_total_winnings(input, false)
}

fn part2(input: &String) -> u64 {
    get_total_winnings(input, true)
}

fn get_total_winnings(input: &String, account_for_jokers: bool) -> u64 {
    input.lines()
        .map(|line| {
            let mut split = line.split(" ");
            let cards = split.next().unwrap();
            let bid = split.next().unwrap().parse::<u64>().unwrap();
            (get_hand_value(cards, account_for_jokers), bid)
        })
        .sorted_by_key(|(value, _)| *value)
        .enumerate()
        .map(|(i, (_, bid))| { (i as u64 + 1) * bid })
        .sum()
}

fn get_hand_value(cards: &str, account_for_jokers: bool) -> u64 {
    let mut chars = cards.chars();
    10000000000 * get_hand_type(cards, account_for_jokers)
        + 100000000 * get_card_value(chars.next().unwrap(), account_for_jokers)
        + 1000000 * get_card_value(chars.next().unwrap(), account_for_jokers)
        + 10000 * get_card_value(chars.next().unwrap(), account_for_jokers)
        + 100 * get_card_value(chars.next().unwrap(), account_for_jokers)
        + get_card_value(chars.next().unwrap(), account_for_jokers)
}

fn get_hand_type(cards: &str, account_for_jokers: bool) -> u64 {
    let counts: Vec<u64> = cards.chars()
        .counts()
        .iter()
        .filter(|(c, _)| !account_for_jokers || **c != 'J')
        .map(|(_, n)| { *n as u64 })
        .collect();
    let jokers = if account_for_jokers { cards.chars().filter(|c| *c == 'J').count() as u64 } else { 0 };
    if is_five_of_a_kind(&counts, jokers) {
        7
    } else if is_four_of_a_kind(&counts, jokers) {
        6
    } else if is_full_house(&counts, jokers) {
        5
    } else if is_three_of_a_kind(&counts, jokers) {
        4
    } else if is_two_pairs(&counts) {
        3
    } else if is_one_pair(&counts, jokers) {
        2
    } else {
        1
    }
}

fn is_five_of_a_kind(counts: &Vec<u64>, jokers: u64) -> bool {
    counts.contains(&5)
        || (counts.contains(&4) && jokers == 1)
        || (counts.contains(&3) && jokers == 2)
        || (counts.contains(&2) && jokers == 3)
        || (counts.contains(&1) && jokers == 4)
        || jokers == 5
}

fn is_four_of_a_kind(counts: &Vec<u64>, jokers: u64) -> bool {
    counts.contains(&4)
        || (counts.contains(&3) && jokers == 1)
        || (counts.contains(&2) && jokers == 2)
        || (counts.contains(&1) && jokers == 3)
}

fn is_full_house(counts: &Vec<u64>, jokers: u64) -> bool {
    counts.contains(&3) && counts.contains(&2)
        || counts.iter().filter(|n| **n == 2).count() == 2 && jokers == 1
}

fn is_three_of_a_kind(counts: &Vec<u64>, jokers: u64) -> bool {
    counts.contains(&3)
        || (counts.contains(&2) && jokers == 1)
        || (counts.contains(&1) && jokers == 2)
}

fn is_two_pairs(counts: &Vec<u64>) -> bool {
    counts.iter().filter(|n| **n == 2).count() == 2
}

fn is_one_pair(counts: &Vec<u64>, jokers: u64) -> bool {
    counts.contains(&2)
    || jokers == 1
}

fn get_card_value(c: char, account_for_jokers: bool) -> u64 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => if account_for_jokers { 0 } else { 11 },
        'T' => 10,
        c if c.is_numeric() => c.to_digit(10).unwrap().into(),
        _ => 0
    }
}
