const INPUT: &str = include_str!("../../inputs/day01.txt");

fn part1(input: &str) -> u64 {
    let mut first = Vec::new();
    let mut second = Vec::new();

    for line in input.lines() {
        let mut split = line.split_whitespace();
        first.push(split.next().unwrap().parse::<u64>().unwrap());
        second.push(split.next().unwrap().parse::<u64>().unwrap());
    }

    first.sort();
    second.sort();

    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut first = Vec::new();
    let mut second = Vec::new();

    for line in input.lines() {
        let mut split = line.split_whitespace();
        first.push(split.next().unwrap().parse::<u64>().unwrap());
        second.push(split.next().unwrap().parse::<u64>().unwrap());
    }

    let mut sum = 0;

    for i in first {
        for j in &second {
            if i == *j {
                sum += i;
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
        let result = part1(include_str!("../../inputs/day01_example.txt"));

        assert_eq!(11, result);
    }

    #[test]
    fn test_part1() {
        let result = part1(include_str!("../../inputs/day01.txt"));

        assert_eq!(1319616, result);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(include_str!("../../inputs/day01_example.txt"));

        assert_eq!(31, result);
    }

    #[test]
    fn test_part2() {
        let result = part2(include_str!("../../inputs/day01.txt"));

        assert_eq!(27267728, result);
    }
}
