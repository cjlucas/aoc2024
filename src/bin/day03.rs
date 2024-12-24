use regex::Regex;

const INPUT: &str = include_str!("../../inputs/day04.txt");

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    fn at(&self, row: usize, col: usize) -> Option<char> {
        if row > self.rows.len() {
            return None;
        }

        if col > self.rows[0].len() {
            return None;
        }

        Some(self.rows[row][col])
    }
}

fn parse_input(input: &str) -> Grid {
    let rows = input.lines().map(|line| line.chars().collect()).collect();

    Grid { rows }
}

fn part1(input: &str) -> u64 {
    let grid = parse_input(input);
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
        let result = part1(include_str!("../../inputs/day04_example.txt"));

        assert_eq!(18, result);
    }

    // #[test]
    // fn test_part1() {
    //     let result = part1(include_str!("../../inputs/day04.txt"));

    //     assert_eq!(0, result);
    // }

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(include_str!("../../inputs/day04p2_example.txt"));

    //     assert_eq!(0, result);
    // }

    // #[test]
    // fn test_part2() {
    //     let result = part2(include_str!("../../inputs/day04.txt"));

    //     assert_eq!(0, result);
    // }
}
