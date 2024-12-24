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
    let mut enabled = true;
    let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don\'t\(\))").unwrap();

    let mut sum = 0;

    for m in re.find_iter(input) {
        match m.as_str() {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            s => {
                if enabled {
                    let re = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();
                    let cap = re.captures(s).unwrap();
                    sum += cap[1].parse::<u64>().unwrap() * cap[2].parse::<u64>().unwrap();
                }
            }
        }
    }

    sum
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = part1(include_str!("../../inputs/day03_example.txt"));

        assert_eq!(161, result);
    }

    #[test]
    fn test_part1() {
        let result = part1(include_str!("../../inputs/day03.txt"));

        assert_eq!(188741603, result);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(include_str!("../../inputs/day03p2_example.txt"));

        assert_eq!(48, result);
    }

    #[test]
    fn test_part2() {
        let result = part2(include_str!("../../inputs/day03.txt"));

        assert_eq!(67269798, result);
    }
}
