use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/day04.txt");

#[derive(Debug)]
struct Grid {
    items: HashMap<(usize, usize), char>,
}

#[derive(Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn offset(&self) -> (i64, i64) {
        match self {
            Self::N => (0, -1),
            Self::NE => (1, -1),
            Self::E => (1, 0),
            Self::SE => (1, 1),
            Self::S => (0, 1),
            Self::SW => (-1, 1),
            Self::W => (-1, 0),
            Self::NW => (-1, -1),
        }
    }
}

impl Grid {
    fn num_rows(&self) -> usize {
        *self.items.keys().map(|(_, y)| y).max().unwrap() + 1
    }

    fn num_cols(&self) -> usize {
        *self.items.keys().map(|(x, _)| x).max().unwrap() + 1
    }

    fn at(&self, x: usize, y: usize) -> Option<char> {
        self.items.get(&(x, y)).copied()
    }

    fn direction_iterator(&self, x: usize, y: usize, direction: Direction) -> DirectionIterator {
        DirectionIterator {
            point: Some((x, y)),
            direction,
            grid: self,
        }
    }
}

struct DirectionIterator<'a> {
    point: Option<(usize, usize)>,
    direction: Direction,
    grid: &'a Grid,
}

impl<'a> Iterator for DirectionIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.point?;
        let item = self.grid.at(x, y)?;

        let (x_offset, y_offset) = self.direction.offset();
        let x = (x as i64).checked_add(x_offset).map(|n| n as usize);
        let y = (y as i64).checked_add(y_offset).map(|n| n as usize);

        self.point = if x.is_some() && y.is_some() {
            Some((x.unwrap(), y.unwrap()))
        } else {
            None
        };

        Some(item)
    }
}

fn parse_input(input: &str) -> Grid {
    let mut items = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            items.insert((x, y), ch);
        }
    }

    Grid { items }
}

fn part1(input: &str) -> u64 {
    let grid = parse_input(input);

    let mut result = 0;

    for x in 0..grid.num_cols() {
        for y in 0..grid.num_rows() {
            for direction in vec![
                Direction::N,
                Direction::NE,
                Direction::E,
                Direction::SE,
                Direction::S,
                Direction::SW,
                Direction::W,
                Direction::NW,
            ] {
                let mut iter = grid.direction_iterator(x, y, direction);
                if iter.next() == Some('X')
                    && iter.next() == Some('M')
                    && iter.next() == Some('A')
                    && iter.next() == Some('S')
                {
                    result += 1;
                }
            }
        }
    }

    result
}

fn part2(input: &str) -> u64 {
    let grid = parse_input(input);

    let mut result = 0;

    let mut foo = HashSet::new();
    foo.insert(Some('M'));
    foo.insert(Some('S'));

    for x in 0..grid.num_cols() {
        for y in 0..grid.num_rows() {
            if grid.at(x, y) != Some('A') {
                continue;
            }

            let mut set = HashSet::new();
            let mut iter = grid.direction_iterator(x, y, Direction::NW);
            iter.next();
            set.insert(iter.next());

            let mut iter = grid.direction_iterator(x, y, Direction::SE);
            iter.next();
            set.insert(iter.next());

            if set != foo {
                continue;
            }

            let mut set = HashSet::new();
            let mut iter = grid.direction_iterator(x, y, Direction::NE);
            iter.next();
            set.insert(iter.next());

            let mut iter = grid.direction_iterator(x, y, Direction::SW);
            iter.next();
            set.insert(iter.next());

            if set != foo {
                continue;
            }

            result += 1;
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
        let result = part1(include_str!("../../inputs/day04_example.txt"));

        assert_eq!(18, result);
    }

    #[test]
    fn test_part1() {
        let result = part1(include_str!("../../inputs/day04.txt"));

        assert_eq!(2569, result);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(include_str!("../../inputs/day04_example.txt"));

        assert_eq!(9, result);
    }

    #[test]
    fn test_part2() {
        let result = part2(include_str!("../../inputs/day04.txt"));

        assert_eq!(1998, result);
    }
}
