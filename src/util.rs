use std::str::FromStr;

pub fn get_numbers_from_line<F: FromStr>(line: &str) -> Vec<F> {
    line.split(" ")
        .filter_map(|s| { s.parse::<F>().ok() })
        .collect()
}
