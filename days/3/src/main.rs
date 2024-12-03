use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("../input/input").expect("Failed to read input file");
    mul(input.clone());
    mul_only_enabled(input);
}
fn mul_only_enabled(input: String) -> i32 {
    let reg_ex = Regex::new(r"don't\(\).+?((do\(\)))|don't\(\).*?.*/gm");
    let collected = reg_ex
        .unwrap()
        .find_iter(&input)
        .map(|mth| mth.as_str())
        .collect::<Vec<&str>>();
    let mut input: String = input.clone();
    collected.iter().for_each(|&res| {
        input = input.replace(res, "");
    });
    mul(input)
}

fn mul(input: String) -> i32 {
    let regex = Regex::new(r"mul\([\d]+,[\d]+\)").expect("Failed to create regex");
    let find = regex
        .captures_iter(&input)
        .map(|captures| captures.extract::<0>().0)
        .collect::<Vec<&str>>();

    let collect = find
        .iter()
        .map(|&elem| {
            Regex::new(r"\d+")
                .unwrap()
                .captures_iter(elem)
                .map(|captures| captures.extract::<0>().0)
                .collect::<Vec<&str>>()
                .iter()
                .map(|&elem| elem.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut res = collect
        .iter()
        .map(|vec| vec[0] * vec[1])
        .collect::<Vec<i32>>();

    let reduce = res
        .iter_mut()
        .reduce(|acc, elem| {
            *acc = *acc + *elem;
            acc
        })
        .unwrap();
    println!("{}", reduce);
    *reduce
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main_test() {
        let input =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        let result = mul(input.clone());
        assert_eq!(161, result);
    }

    #[test]
    fn part2_test() {
        let input = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        let result = mul_only_enabled(input.clone());
        assert_eq!(48, result);
    }
}
