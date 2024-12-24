const INPUT: &str = include_str!("../../inputs/day05.txt");

fn part1(input: &str) -> u64 {
    0
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
    fn test_part1_example() {
        let result = part1(include_str!("../../inputs/day05_example.txt"));

        assert_eq!(143, result);
    }

    #[test]
    fn test_part1() {
        let result = part1(include_str!("../../inputs/day05.txt"));

        assert_eq!(0, result);
    }

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(include_str!("../../inputs/day05_example.txt"));

    //     assert_eq!(0, result);
    // }

    // #[test]
    // fn test_part2() {
    //     let result = part2(include_str!("../../inputs/day05.txt"));

    //     assert_eq!(0, result);
    // }
}
