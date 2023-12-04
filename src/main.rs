mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let day = std::env::args().nth(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    match day {
        1 => { day1::run(); }
        2 => { day2::run(); }
        3 => { day3::run(); }
        4 => { day4::run(); }
        _ => { println!("Please specify which day as command line arg") }
    }
}
