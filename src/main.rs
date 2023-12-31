extern crate core;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod util;

fn main() {
    let day = std::env::args().nth(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    match day {
        1 => { day01::run(); }
        2 => { day02::run(); }
        3 => { day03::run(); }
        4 => { day04::run(); }
        5 => { day05::run(); }
        6 => { day06::run(); }
        7 => { day07::run(); }
        8 => { day08::run(); }
        9 => { day09::run(); }
        10 => { day10::run(); }
        _ => { println!("Please specify which day as command line arg") }
    }
}
