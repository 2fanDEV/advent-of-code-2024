use std::{collections::HashMap, vec};

use crate::read_input;

pub fn main() -> i32 {
    let input = read_input("input/05");
    uncorrected_updates_sum_of_middle_paged_numbers(&input)
}

// part 2
fn uncorrected_updates_sum_of_middle_paged_numbers(input: &str) -> i32 {
    let (page_order_rules, updates) = page_order_rules_and_updates(input);
    let page_order_rules_map: HashMap<i32, Vec<i32>> = assemble_page_order_map(page_order_rules);
    let mut updates = assemble_vectors(&updates);
    let mut correct_updates: Vec<Vec<i32>> = Vec::new();
    for mut update in updates.clone() {
        let mut flag = false;
        for i in 0..update.len() {
            let page_order_for_i = page_order_rules_map.get(&update[i]);
            match page_order_for_i {
                Some(page_order) => {
                    let vec = update.clone();
                    let splitted = vec.split_at(i + 1);
                    let split_before = splitted.0;
                    let split = splitted.1;
                    for (j, num) in split.iter().enumerate() {
                        if page_order.contains(num) {
                            continue;
                        } else {
                            flag = true;
                        }
                    }

                    for (j, num) in split_before.iter().enumerate() {
                        let number = update[i];
                        let page_order_for_number = page_order_rules_map.get(&number);
                        if page_order_for_number.is_some() {
                            if page_order_for_number.unwrap().contains(num) {
                                update.swap(i, j);
                                flag = true;
                            } else {
                                continue;
                            }
                        }
                    }
                }
                None => {
                    if i == updates.len() - 1 {
                        break;
                    }
                    continue;
                }
            }
        }
        if flag {
            correct_updates.push(update.clone());
        }
    }

    let mut sum_of_middle_pages = 0;
    println!("Correct Updates: {:?}", correct_updates);
    for row in correct_updates {
        let row_length = row.len();
        sum_of_middle_pages = sum_of_middle_pages + row[row_length / 2];
    }
    sum_of_middle_pages
}

// part 1
fn correct_updates_sum_of_middle_paged_numbers(input: &str) -> i32 {
    let (page_order_rules, updates) = page_order_rules_and_updates(input);
    let page_order_rules_map: HashMap<i32, Vec<i32>> = assemble_page_order_map(page_order_rules);
    let updates = assemble_vectors(&updates);
    println!("{:?}", page_order_rules_map);
    println!("{:?}", updates);
    let mut correct_updates: Vec<Vec<i32>> = Vec::new();
    for update in updates.clone() {
        let mut flag = false;
        for i in 0..update.len() {
            let page_order_for_i = page_order_rules_map.get(&update[i]);
            match page_order_for_i {
                Some(page_order) => {
                    let splitted = update.split_at(i + 1);
                    let split_before = splitted.0;
                    let split = splitted.1;
                    for num in split {
                        if page_order.contains(num) {
                            continue;
                        } else {
                            flag = true;
                            break;
                        }
                    }

                    for num in split_before {
                        let number = update[i];
                        let page_order_for_number = page_order_rules_map.get(&number);
                        if (page_order_for_number.is_some()) {
                            if page_order_for_number.unwrap().contains(num) {
                                flag = true;
                                break;
                            } else {
                                continue;
                            }
                        }
                    }
                }
                None => {
                    if i == updates.len() - 1 {
                        break;
                    }
                    continue;
                }
            }
        }
        if flag {
            continue;
        } else {
            correct_updates.push(update.clone());
        }
    }

    let mut sum_of_middle_pages = 0;
    println!("Correct Updates: {:?}", correct_updates);
    for row in correct_updates {
        let row_length = row.len();
        sum_of_middle_pages = sum_of_middle_pages + row[row_length / 2];
    }
    sum_of_middle_pages
}

fn assemble_vectors(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n")
        .map(|row| {
            println!("{}", row);
            row.split(",")
                .map(|fsx| fsx.parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn assemble_page_order_map(page_order_rules: String) -> HashMap<i32, Vec<i32>> {
    let mut hashmap: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut pair_vec = page_order_rules
        .split("\n")
        .map(|line| line.split_once("|").expect("failed to split"))
        .collect::<Vec<(&str, &str)>>();

    for pair in pair_vec {
        let pair0 = pair.0.parse::<i32>().unwrap();
        let pair1 = pair.1.parse::<i32>().unwrap();
        let res_vec = hashmap.get(&pair0);
        match res_vec {
            Some(result) => {
                let get: Option<&mut Vec<i32>> = hashmap.get_mut(&pair0);
                get.unwrap().push(pair.1.parse::<i32>().unwrap());
            }
            None => {
                let mut vector = Vec::new();
                vector.push(pair.1.parse::<i32>().unwrap());
                hashmap.insert(pair.0.parse::<i32>().unwrap(), vector);
            }
        }
    }
    hashmap
}

fn page_order_rules_and_updates(input: &str) -> (String, String) {
    let split_at: Vec<String> = input.split("\n\n").map(|elem| String::from(elem)).collect();
    (split_at[0].clone(), split_at[1].clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    static input: &'static str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn outcome_test() {
        assert_eq!(correct_updates_sum_of_middle_paged_numbers(input), 143);
    }
}
