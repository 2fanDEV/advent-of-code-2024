use std::collections::HashMap;

use crate::{parse_input_into_char_matrix, read_input};

pub fn main() {
    let input = read_input("input/06");
    let mut matrix = parse_input_into_char_matrix(&input);
    let guard_position = find_guard_position(&matrix);
    distinct_positions_obstructions(&mut matrix, guard_position);
}

enum GuardMovements {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

fn distinct_postions(matrix: &mut Vec<Vec<char>>) -> i32 {
    let mut count: HashMap<(usize, usize), i32> = HashMap::new();
    let mut guard_position = find_guard_position(&matrix);
    matrix[guard_position.1][guard_position.0] = 'X';
    let mut movement_direction =
        find_current_movement_direction(&matrix[guard_position.0][guard_position.1]);
    let mut distinct_positions_count = 1;
    while guard_position.0 > 0
        && guard_position.0 < matrix[0].len() - 1
        && guard_position.1 > 0
        && guard_position.1 < matrix.len() - 1
    {
        match movement_direction {
            GuardMovements::UP => {
                if matrix[guard_position.1 - 1][guard_position.0] != '#' {
                    matrix[guard_position.1][guard_position.0] = 'X';
                    if matrix[guard_position.1 - 1][guard_position.0] != 'X' {
                        distinct_positions_count = distinct_positions_count + 1;
                    }
                    guard_position.1 = guard_position.1 - 1;
                } else {
                    movement_direction = GuardMovements::RIGHT;
                    match &mut count.get(&guard_position) {
                        Some(cnt) => {
                            if **cnt == 4 {
                                return -1;
                            }
                            count.insert(guard_position, **cnt + 1);
                        }
                        None => {
                            count.insert(guard_position, 0);
                        }
                    }
                }
            }
            GuardMovements::RIGHT => {
                if matrix[guard_position.1][guard_position.0 + 1] != '#' {
                    matrix[guard_position.1][guard_position.0] = 'X';
                    if matrix[guard_position.1][guard_position.0 + 1] != 'X' {
                        distinct_positions_count = distinct_positions_count + 1;
                    }
                    guard_position.0 = guard_position.0 + 1;
                } else {
                    movement_direction = GuardMovements::DOWN;
                    match &mut count.get(&guard_position) {
                        Some(cnt) => {
                            if **cnt == 4 {
                                return -1;
                            }
                            count.insert(guard_position, **cnt + 1);
                        }
                        None => {
                            count.insert(guard_position, 0);
                        }
                    }
                }
            }
            GuardMovements::DOWN => {
                if matrix[guard_position.1 + 1][guard_position.0] != '#' {
                    matrix[guard_position.1][guard_position.0] = 'X';
                    if matrix[guard_position.1 + 1][guard_position.0] != 'X' {
                        distinct_positions_count = distinct_positions_count + 1;
                    }
                    guard_position.1 = guard_position.1 + 1;
                } else {
                    movement_direction = GuardMovements::LEFT;
                    match &mut count.get(&guard_position) {
                        Some(cnt) => {
                            if **cnt == 4 {
                                return -1;
                            }
                            count.insert(guard_position, **cnt + 1);
                        }
                        None => {
                            count.insert(guard_position, 0);
                        }
                    }
                }
            }
            GuardMovements::LEFT => {
                if matrix[guard_position.1][guard_position.0 - 1] != '#' {
                    matrix[guard_position.1][guard_position.0] = 'X';
                    if matrix[guard_position.1][guard_position.0 - 1] != 'X' {
                        distinct_positions_count = distinct_positions_count + 1;
                    }
                    guard_position.0 = guard_position.0 - 1;
                } else {
                    movement_direction = GuardMovements::UP;
                    match &mut count.get(&guard_position) {
                        Some(cnt) => {
                            if **cnt == 4 {
                                return -1;
                            }
                            count.insert(guard_position, **cnt + 1);
                        }
                        None => {
                            count.insert(guard_position, 0);
                        }
                    }
                }
            }
        }
    }
    matrix[guard_position.1][guard_position.0] = 'X';
    distinct_positions_count
}

fn distinct_positions_obstructions(
    matrix: &mut Vec<Vec<char>>,
    start_position: (usize, usize),
) -> i32 {
    let mut loop_counter = 0;
    distinct_postions(matrix);
    matrix[start_position.1][start_position.0] = '^';
    for row in matrix.clone() {
        println!("{:?}", row);
    }
    for (idx, row) in matrix.clone().iter().enumerate() {
        let xs: Vec<(usize, &char)> = row
            .iter()
            .enumerate()
            .filter(|(_idx, &character)| character.eq(&'X'))
            .collect();

        for elem in xs {
            matrix[idx][elem.0] = '#';
            matrix[start_position.1][start_position.0] = '^';
            if distinct_postions(matrix) == -1 {
                loop_counter = loop_counter + 1;
            }

            matrix[idx][elem.0] = 'X';
        }
    }
    matrix[start_position.1 - 1][start_position.0] = '^';
    matrix[start_position.1][start_position.0] = '#';
    if distinct_postions(matrix) == -1 {
        loop_counter = loop_counter + 1;
    }
    loop_counter
}

fn find_current_movement_direction(movement_character: &char) -> GuardMovements {
    match movement_character {
        '^' => GuardMovements::UP,
        '>' => GuardMovements::RIGHT,
        '<' => GuardMovements::LEFT,
        'v' => GuardMovements::DOWN,
        _ => GuardMovements::UP,
    }
}

fn find_guard_position(input: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, _ix) in input.iter().enumerate() {
        for (j, _dx) in input.iter().enumerate() {
            if input[i][j] == '^' {
                return (j, i);
            }
        }
    }
    panic!("no guard position found")
}

#[cfg(test)]
mod tests {
    use super::*;

    static input: &'static str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn distinct_positions_test() {
        let mut matrix = parse_input_into_char_matrix(input);
        // assert_eq!(distinct_postions(&mut matrix), 41);
        assert_eq!(
            distinct_positions_obstructions(&mut matrix.clone(), find_guard_position(&matrix)),
            6
        );
    }
}
