use std::{cmp::Reverse, fs};

pub fn main() -> i32 {
    let input: String = fs::read_to_string("input/04")
        .expect("Failed to read file")
        .to_string();

    println!("{}", count_xmas(input.clone()));
    count_x_mas(input)
}

fn count_xmas(input: String) -> i32 {
    let mut input = input
        .split("\n")
        .map(|row| row.trim())
        .map(|row| row.chars().collect())
        .collect::<Vec<Vec<char>>>();

    println!("width: {}, height: {}", input[0].len(), input.len());
    let mut test = String::from("");
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            test.clear();
            if j + 3 < input[i].len() {
                for x in 0..4 {
                    test.push(input[i][j + x]);
                    if &test == "XMAS" || &test.chars().rev().collect::<String>() == "XMAS" {
                        count = count + 1;
                    }
                }

                if i + 3 < input.len() {
                    test.clear();
                    for x in 0..4 {
                        test.push(input[i + x][j + x]);
                        if &test == "XMAS" || &test.chars().rev().collect::<String>() == "XMAS" {
                            count = count + 1;
                        }
                    }
                }
            }
            test.clear();
            if (j as i32) - 3 >= 0 && i + 3 < input.len() {
                for x in 0..4 {
                    test.push(input[i + x][j - x]);
                    if &test == "XMAS" || &test.chars().rev().collect::<String>() == "XMAS" {
                        count = count + 1;
                    }
                }
            }

            test.clear();
            if i + 3 < input.len() {
                for x in 0..4 {
                    test.push(input[i + x][j]);
                    if &test == "XMAS" || &test.chars().rev().collect::<String>() == "XMAS" {
                        count = count + 1;
                    }
                }
            }
        }
    }
    count
}

fn count_x_mas(input: String) -> i32 {
    let mut input = input
        .split("\n")
        .map(|row| row.trim())
        .map(|row| row.chars().collect())
        .collect::<Vec<Vec<char>>>();

    println!("width: {}, height: {}", input[0].len(), input.len());
    let mut test = String::from("");
    let mut count = 0;
    let mut flag1 = false;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            test.clear();
            flag1 = false;
            if i + 2 < input.len() && j + 2 < input.len() {
                test.clear();
                for x in 0..3 {
                    test.push(input[i + x][j + x]);
                    if &test == "MAS" || &test.chars().rev().collect::<String>() == "MAS" {
                        flag1 = true;
                    }
                }

                test.clear();
                for x in 0..3 {
                    test.push(input[i + x][(j + 2) - x]);
                    if &test == "MAS" || &test.chars().rev().collect::<String>() == "MAS" {
                        if flag1 {
                            count = count + 1;
                        }
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX";

    #[test]
    fn count_xmas_test() {
        assert_eq!(18, count_xmas(TEST_INPUT.to_string()));
    }
}
