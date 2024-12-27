use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/day05.txt");

fn part1(input: &str) -> u64 {
    let mut preceeding_pages_map: HashMap<u64, HashSet<u64>> = HashMap::new();

    let mut lines = input.lines();

    for line in &mut lines {
        if line.trim() == "" {
            break;
        }

        let (n1, n2) = line.split_once('|').unwrap();
        let n1: u64 = str::parse(n1).unwrap();
        let n2: u64 = str::parse(n2).unwrap();

        let preceeding_pages = preceeding_pages_map.entry(n2).or_default();
        preceeding_pages.insert(n1);
    }

    let mut result = 0;

    for line in lines {
        let update: Vec<u64> = line.split(',').map(|n| str::parse(n).unwrap()).collect();
        let update_set: HashSet<u64> = update.clone().into_iter().collect();

        let mut num_preceeding_pages: HashMap<u64, usize> = HashMap::new();

        for page in &update_set {
            let preceeding_pages_cnt = preceeding_pages_map
                .entry(*page)
                .or_default()
                .intersection(&update_set)
                .count();
            num_preceeding_pages.insert(*page, preceeding_pages_cnt);
        }

        let mut foo = update.clone();
        foo.sort_by_key(|n| *num_preceeding_pages.entry(*n).or_default());

        if foo == update {
            result += foo[foo.len() / 2];
        }
    }

    result
}

fn part2(input: &str) -> u64 {
    let mut preceeding_pages_map: HashMap<u64, HashSet<u64>> = HashMap::new();

    let mut lines = input.lines();

    for line in &mut lines {
        if line.trim() == "" {
            break;
        }

        let (n1, n2) = line.split_once('|').unwrap();
        let n1: u64 = str::parse(n1).unwrap();
        let n2: u64 = str::parse(n2).unwrap();

        let preceeding_pages = preceeding_pages_map.entry(n2).or_default();
        preceeding_pages.insert(n1);
    }

    let mut result = 0;

    for line in lines {
        let update: Vec<u64> = line.split(',').map(|n| str::parse(n).unwrap()).collect();
        let update_set: HashSet<u64> = update.clone().into_iter().collect();

        let mut num_preceeding_pages: HashMap<u64, usize> = HashMap::new();

        for page in &update_set {
            let preceeding_pages_cnt = preceeding_pages_map
                .entry(*page)
                .or_default()
                .intersection(&update_set)
                .count();
            num_preceeding_pages.insert(*page, preceeding_pages_cnt);
        }

        let mut foo = update.clone();
        foo.sort_by_key(|n| *num_preceeding_pages.entry(*n).or_default());

        if foo != update {
            result += foo[foo.len() / 2];
        }
    }

    result
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

        assert_eq!(6242, result);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(include_str!("../../inputs/day05_example.txt"));

        assert_eq!(123, result);
    }

    #[test]
    fn test_part2() {
        let result = part2(include_str!("../../inputs/day05.txt"));

        assert_eq!(5169, result);
    }
}
