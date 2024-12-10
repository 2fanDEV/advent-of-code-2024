use std::thread::current;

use crate::{parse_rows, read_input};

pub fn main() -> u64 {
    let input = read_input("input/07");
    let rows = parse_rows(&input);
    println!(
        "part1: {}",
        check_calibration_equations_add_multiply(rows.clone())
    );
    check_calibration_equation_add_multiply_concat(rows)
}

pub fn check_calibration_equations_add_multiply(equations: Vec<String>) -> u64 {
    let operators = vec!['+', '*'];
    is_solvable_with_operators(equations, operators)
}

pub fn check_calibration_equation_add_multiply_concat(equations: Vec<String>) -> u64 {
    let operators = vec!['+', '*', '|'];
    is_solvable_with_operators(equations, operators)
}

fn is_solvable_with_operators(equations: Vec<String>, operators: Vec<char>) -> u64 {
    let mut sum = 0;
    for equation in equations {
        let (result, mut equation_parts) = parse_equation_rows(equation);
        if is_solvable(result, 0, &operators, 0, &mut equation_parts.as_mut()) {
            sum = sum + result;
        }
    }
    sum
}

fn parse_equation_rows(equation: String) -> (u64, Vec<u64>) {
    let (result, equation_parts) = equation
        .split_once(":")
        .map(|elements| {
            (
                elements.0.parse::<u64>().unwrap(),
                elements
                    .1
                    .to_string()
                    .trim()
                    .split(" ")
                    .map(|elem| elem.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .unwrap();
    (result, equation_parts)
}

fn is_solvable(
    test_value: u64,
    current_value: u64,
    operators: &Vec<char>,
    index: usize,
    numbers: &mut Vec<u64>,
) -> bool {
    if index == numbers.len() {
        return current_value == test_value;
    }
    for operator in operators {
        match operator {
            '+' => {
                if is_solvable(
                    test_value,
                    current_value + numbers[index],
                    operators,
                    index + 1,
                    numbers,
                ) {
                    return true;
                }
            }
            '*' => {
                if is_solvable(
                    test_value,
                    current_value * numbers[index],
                    operators,
                    index + 1,
                    numbers,
                ) {
                    return true;
                }
            }
            '|' => {
                let curr_val = current_value as u64 * 10u64.pow(numbers[index].ilog10() + 1)
                    + numbers[index] as u64;
                if is_solvable(
                    test_value,
                    curr_val,
                    operators,
                    0,
                    &mut numbers.split_at(index + 1).1.to_vec(),
                ) {
                    return true;
                }
                return false;
            }
            _ => {}
        }
    }
    current_value == test_value
}

#[cfg(test)]
mod tests {
    use super::*;

    static input: &'static str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn check_calibration_equations_test() {
        let rows = parse_rows(input);
        assert_eq!(check_calibration_equations_add_multiply(rows.clone()), 3749);
        assert_eq!(check_calibration_equation_add_multiply_concat(rows), 11387);
    }
}
