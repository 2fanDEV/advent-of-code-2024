use std::collections::{HashMap, HashSet};

use crate::{parse_input_into_char_matrix, read_input};

pub fn main() -> i16 {
    let input = read_input("input/08");
    let matrix = parse_input_into_char_matrix(&input);
    antinodes_locations(matrix)
}

fn antinodes_locations(matrix: Vec<Vec<char>>) -> i16 {
    let mut alphabet: HashSet<char> = HashSet::new();
    let mut positions_of_characters: HashMap<char, Vec<(i16, i16)>> = HashMap::new();
    populate_set_and_map(matrix, &mut alphabet, &mut positions_of_characters);
    println!("{:?}", alphabet);
    println!("{:?}", positions_of_characters);
    0
}

fn populate_set_and_map(
    matrix: Vec<Vec<char>>,
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
        assert_eq!(antinodes_locations(matrix), 14);
    }
}
