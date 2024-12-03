use regex::Regex;

const INPUT: &str = include_str!("../../inputs/day03.txt");

fn part1(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();

    let mut sum = 0;

    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap();
    }

    sum
}

fn part2(input: &str) -> u64 {
    0
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(include_str!("../../inputs/day03_example.txt"));

        assert_eq!(161, result);
    }

    // #[test]
    // fn test_part2() {
    //     let result = part2(include_str!("../../inputs/day03_example.txt"));

    //     assert_eq!(4, result);
    // }
}
