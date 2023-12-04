use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("resources/input04").expect("Should have been able to read the file");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[test]
fn test() {
    let input = fs::read_to_string("resources/test04").expect("Should have been able to read the file");
    assert_eq!(part1(&input), 13);
    assert_eq!(part2(&input), 30);
}

fn part1(input: &String) -> u32 {
    input.lines().map(|line| {
        let mut card = line.split(": ").nth(1).unwrap().split(" | ");
        let winning_numbers: Vec<u32> = card.next().unwrap().split(" ").filter_map(|s| s.parse::<u32>().ok()).collect();
        let number_of_winning_numbers = card.next().unwrap().split(" ").filter_map(|s| s.parse::<u32>().ok()).filter(|x: &u32| winning_numbers.contains(&x)).count() as u32;
        if number_of_winning_numbers > 0 { 2_u32.pow(number_of_winning_numbers - 1) } else { 0 }
    }).sum()
}

fn part2(input: &String) -> u32 {
    let cards: Vec<&str> = input.lines().map(|line| {
        line.split(": ").nth(1).unwrap()
    }).collect();

    let mut scratch_cards: HashMap<u32, u32> = HashMap::with_capacity(cards.len());
    for i in 0_u32..(cards.len() as u32) {
        scratch_cards.insert(i, 1);
    }

    cards.iter().enumerate().for_each(|(i, card)| {
        let number_of_current_cards = scratch_cards.get(&(i as u32)).unwrap().clone();

        let mut card_split = card.split(" | ");
        let winning_numbers: Vec<u32> = card_split.next().unwrap().split(" ").filter_map(|s| s.parse::<u32>().ok()).collect();
        let number_of_winning_numbers = card_split.next().unwrap().split(" ").filter_map(|s| s.parse::<u32>().ok()).filter(|x: &u32| winning_numbers.contains(&x)).count();
        for j in (i + 1)..=(i + number_of_winning_numbers) {
            scratch_cards.insert(j as u32, scratch_cards.get(&(j as u32)).unwrap() + number_of_current_cards);
        }
    });

    scratch_cards.values().sum()
}