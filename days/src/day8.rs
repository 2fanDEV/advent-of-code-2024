use std::collections::{HashMap, HashSet};

use crate::{parse_input_into_char_matrix, read_input};

pub fn main() -> i16 {
    let input = read_input("input/08");
    let matrix = parse_input_into_char_matrix(&input);
    println!("part1: {}", antinodes_locations(matrix.clone()));
    antinodes_part2(matrix)
}

fn antinodes_locations(matrix: Vec<Vec<char>>) -> i16 {
    let sub = |(x, y), (x2, y2)| -> (i16, i16) { (x2 - x, y2 - y) };
    let add = |(x, y): (i16, i16), (x2, y2): (i16, i16)| -> (i16, i16) { (x2 + x, y2 + y) };
    let mut alphabet: HashSet<char> = HashSet::new();
    let mut positions_of_characters: HashMap<char, Vec<(i16, i16)>> = HashMap::new();
    let mut unique_positions: HashSet<(i16, i16)> = HashSet::new();
    populate_set_and_map(&matrix, &mut alphabet, &mut positions_of_characters);
    for character in alphabet {
        let positions = positions_of_characters.get(&character).unwrap();
        if positions.len() > 1 {
            for i in 0..positions.len() - 1 {
                let pos1 = positions[i];
                for j in i..positions.len() - 1 {
                    let pos2 = positions[j + 1];
                    let sub1 = sub(sub(pos1, pos2), pos1);
                    let sub2 = sub(sub(pos2, pos1), pos2);
                    if check_bounds(sub1, matrix.len() as i16, matrix[i].len() as i16).is_some() {
                        unique_positions.insert(sub1);
                    }
                    if check_bounds(sub2, matrix.len() as i16, matrix[i].len() as i16).is_some() {
                        unique_positions.insert(sub2);
                    };
                }
            }
        }
    }
    unique_positions.len() as i16
}

fn antinodes_part2(matrix: Vec<Vec<char>>) -> i16 {
    let sub = |(x, y), (x2, y2)| -> (i16, i16) { (x2 - x, y2 - y) };
    let add = |(x, y): (i16, i16), (x2, y2): (i16, i16)| -> (i16, i16) { (x2 + x, y2 + y) };
    let mut alphabet: HashSet<char> = HashSet::new();
    let mut positions_of_characters: HashMap<char, Vec<(i16, i16)>> = HashMap::new();
    let mut unique_positions: HashSet<(i16, i16)> = HashSet::new();
    populate_set_and_map(&matrix, &mut alphabet, &mut positions_of_characters);
    for character in alphabet {
        let positions = positions_of_characters.get(&character).unwrap();
        if positions.len() > 1 {
            for i in 0..positions.len() - 1 {
                let mut antenna_1 = positions[i];
                for j in i..positions.len() - 1 {
                    let mut antenna_2 = positions[j + 1];
                    let diff = sub(antenna_1, antenna_2);

                    while check_bounds(antenna_2, matrix.len() as i16, matrix[i].len() as i16)
                        .is_some()
                    {
                        unique_positions.insert(antenna_2);
                        antenna_2 = add(diff, antenna_2);
                    }

                    let mut antenna = antenna_1;

                    while check_bounds(antenna, matrix.len() as i16, matrix[i].len() as i16)
                        .is_some()
                    {
                        unique_positions.insert(antenna);
                        antenna = sub(diff, antenna);
                    }
                }
            }
        }
    }
    unique_positions.len() as i16
}

fn check_bounds(tuple: (i16, i16), x_len: i16, y_len: i16) -> Option<(i16, i16)> {
    if tuple.1 < x_len && tuple.1 >= 0 && tuple.0 < y_len && tuple.0 >= 0 {
        return Some(tuple);
    }

    None
}

fn populate_set_and_map(
    matrix: &Vec<Vec<char>>,
    alphabet: &mut HashSet<char>,
    positions_of_characters: &mut HashMap<char, Vec<(i16, i16)>>,
) {
    for i in matrix.iter().enumerate() {
        for j in matrix[i.0].iter().enumerate() {
            let character = matrix[i.0][j.0];
            if character.is_alphanumeric() {
                alphabet.insert(character);
                match positions_of_characters.get_mut(&character) {
                    Some(vec) => {
                        vec.push((i.0 as i16, j.0 as i16));
                    }
                    None => {
                        let vec = vec![(i.0 as i16, j.0 as i16)];
                        positions_of_characters.insert(character, vec);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static input: &'static str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    #[test]
    fn unique_locations_antinodes_test() {
        let matrix = parse_input_into_char_matrix(input);
        assert_eq!(antinodes_locations(matrix.clone()), 14);
        assert_eq!(antinodes_part2(matrix), 34);
    }
}
