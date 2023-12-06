use std::cmp::min;
use std::fs;
use std::str::Split;

pub fn run() {
    let input = fs::read_to_string("src/resources/input05").expect("Should have been able to read the file");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[test]
fn test() {
    let input = fs::read_to_string("src/resources/test05").expect("Should have been able to read the file");
    assert_eq!(part1(&input), 35);
    assert_eq!(part2(&input), 46);
}

fn part1(input: &String) -> i64 {
    let mut input_groups = input.split("\r\n\r\n");
    let seeds: Vec<(i64, i64)> = input_groups.next().unwrap()
        .strip_prefix("seeds: ").unwrap()
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .map(|x| (x, x + 1))
        .collect();

    find_seed_locations(&mut input_groups, seeds)
}

fn part2(input: &String) -> i64 {
    let mut input_groups = input.split("\r\n\r\n");
    let seeds: Vec<(i64, i64)> = input_groups.next().unwrap()
        .strip_prefix("seeds: ").unwrap()
        .split(" ").collect::<Vec<&str>>().chunks(2)
        .map(|chunk| {
            let start = chunk[0].parse::<i64>().unwrap();
            let len = chunk[1].parse::<i64>().unwrap();
            (start, start + len)
        })
        .collect();

    find_seed_locations(&mut input_groups, seeds)
}

fn find_seed_locations(input_groups: &mut Split<&str>, input_seeds: Vec<(i64, i64)>) -> i64 {
    let mut seeds = input_seeds.clone();
    input_groups.for_each(|input_group| {
        let maps: Vec<(i64, i64, i64)> = input_group.lines().skip(1).map(|line| {
            let numbers: Vec<i64> = line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
            let (dest_start, src_start, len) = (numbers.get(0).unwrap(), numbers.get(1).unwrap(), numbers.get(2).unwrap());
            (*src_start, *src_start + *len, *dest_start - *src_start)
        }).collect();

        seeds = seeds.iter().map(|(x, y)| {
            let mut new_ranges: Vec<(i64, i64)> = Vec::new();
            let mut i = *x;
            while i < *y {
                let range_of_i = maps.iter()
                    .filter(|(_, end, _)| { i < *end })
                    .min_by_key(|(_, end, _)| {*end});
                match range_of_i {
                    None => {
                        new_ranges.push((i, *y));
                        i = *y;
                    }
                    Some((m, n, transformation)) => {
                        if i < *m {
                            new_ranges.push((i, min(*m, *y)));
                            i = min(*m, *y)
                        }

                        if i < *y {
                            new_ranges.push((i + transformation, min(*n, *y) + transformation));
                            i = min(*n, *y);
                        }
                    }
                }
            }
            new_ranges
        }).flatten().collect();
    });

    *seeds.iter().map(|(start, _)| start).min().unwrap()
}
