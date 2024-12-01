use std::{fs, i32};

fn main() {
    let mut location_id_1: Vec<i32> = Vec::new();
    let mut location_id_2: Vec<i32> = Vec::new();

    let input = fs::read_to_string("../input/input").expect("Failed to read input file");
    for line in input.lines() {
        let split_line = line.split_once(" ").unwrap();
        location_id_1.push(split_line.0.trim().parse().expect("Failed to parse number"));
        location_id_2.push(split_line.1.trim().parse().expect("Failed to parse number"));
    }
    // part1(&mut location_id_1, &mut location_id_2);
    part2(location_id_1, location_id_2);
}

fn part2(location_id_1: Vec<i32>, location_id_2: Vec<i32>) {
    let mut similarity_score = 0;

    for number in location_id_1 {
        let count_in_loc2 = location_id_2.iter().filter(|&a| a.eq(&number)).count();
        similarity_score = similarity_score + ((number as usize) * count_in_loc2);
    }
    println!("{}", similarity_score);
}

fn part1(location_id_1: &mut Vec<i32>, location_id_2: &mut Vec<i32>) {
    location_id_1.sort();
    location_id_2.sort();

    let mut sum = 0;
    for idx in 0..location_id_1.len() {
        sum = sum
            + (i32::max(location_id_1[idx], location_id_2[idx])
                - (i32::min(location_id_1[idx], location_id_2[idx])));
    }
    println!("{}", sum);
}
