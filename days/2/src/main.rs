use core::error;
use std::{
    cmp::{max, min},
    fs,
    vec::Splice,
};

fn main() {
    let input = fs::read_to_string("input/input").expect("Failed to read input file");
    let mut cnt = input
        .lines()
        .filter(|&line| is_report_safe(parse_line(line)))
        .count();
    println!("{}", cnt);
    cnt = input
        .lines()
        .filter(|&line| problem_dampener(parse_line(line)))
        .count();
    println!("{}", cnt);
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|elem| elem.parse().unwrap())
        .collect::<Vec<i32>>()
}

fn is_report_safe(report: Vec<i32>) -> bool {
    let gradually_desc = if report[0] < report[1] { false } else { true };
    for i in 1..report.len() {
        let level_diff = (report[i] - report[i - 1]).abs();
        let level_desc = if report[i - 1] < report[i] {
            false
        } else {
            true
        };
        if level_diff < 1 || level_diff > 3 || !level_desc.eq(&gradually_desc) {
            return false;
        }
    }

    true
}

fn problem_dampener(mut report: Vec<i32>) -> bool {
    if is_report_safe(report.clone()) {
        return true;
    } else {
        for i in 0..report.len() {
            let mut report_clone = report.clone();
            report_clone.remove(i);
            if is_report_safe(report_clone) {
                return true;
            }
        }
    }
    false
}
