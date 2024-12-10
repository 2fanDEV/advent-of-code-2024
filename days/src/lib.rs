use std::fs;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

fn read_input(input: &str) -> String {
    fs::read_to_string(input).expect("Failed to read with path")
}

fn parse_input_into_char_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
