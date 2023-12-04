mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let day = std::env::args().nth(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    match day {
        1 => { day01::run(); }
        2 => { day02::run(); }
        3 => { day03::run(); }
        4 => { day04::run(); }
        _ => { println!("Please specify which day as command line arg") }
    }
}
