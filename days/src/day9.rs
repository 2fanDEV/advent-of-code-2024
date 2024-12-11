use crate::read_input;

pub fn main() {
    let input = read_input("input/09");
    println!("{}", input.lines().count());
}

struct File {
    id: usize,
    space: i32,
}

struct DiskMap {
    file_blocks: Vec<File>,
}

fn part1(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static input1: &'static str = "12345";
    static input2: &'static str = "2333133121414131402";

    #[test]
    fn part1_test() {
        assert_eq!(part1(input2), 1928)
    }
}
