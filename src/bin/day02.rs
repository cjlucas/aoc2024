const INPUT: &str = include_str!("../../inputs/day02.txt");

struct Line {
    ns: Vec<u64>,
}

impl Line {
    fn new(line: &str) -> Self {
        let ns = line
            .split_whitespace()
            .map(|n| u64::from_str_radix(n, 10).unwrap())
            .collect();

        Self { ns }
    }

    fn is_safe(&self) -> bool {
        if !(self.ns.windows(2).all(|pair| pair[0] > pair[1])
            || self.ns.windows(2).all(|pair| pair[0] < pair[1]))
        {
            return false;
        }

        for pair in self.ns.windows(2) {
            let diff = pair[0].abs_diff(pair[1]);
            if diff < 1 || diff > 3 {
                return false;
            }
        }

        true
    }
}

fn part1(input: &str) -> usize {
    input.lines().map(Line::new).filter(Line::is_safe).count()
}

fn part2(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let line = Line::new(line);

        if (0..line.ns.len()).any(|idx| {
            let mut ns = line.ns.clone();
            ns.remove(idx);
            let line2 = Line { ns };

            line2.is_safe()
        }) {
            sum += 1;
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
        let result = part1(include_str!("../../inputs/day02_example.txt"));

        assert_eq!(2, result);
    }

    #[test]
    fn test_part1() {
        let result = part1(include_str!("../../inputs/day02.txt"));

        assert_eq!(236, result);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(include_str!("../../inputs/day02_example.txt"));

        assert_eq!(4, result);
    }

    #[test]
    fn test_part2() {
        let result = part2(include_str!("../../inputs/day02.txt"));

        assert_eq!(308, result);
    }
}
