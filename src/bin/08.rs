advent_of_code::solution!(8);

use itertools::Itertools;
use std::{collections::HashMap, iter::zip};

use nalgebra::{self, DMatrix};

fn parse_input(input: &str) -> (DMatrix<char>, HashMap<char, Vec<(isize, isize)>>) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut locations = nalgebra::DMatrix::from_element(lines.len() - 1, lines[0].len(), '.');
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (y, line) in zip(0..lines.len(), lines) {
        for (idx, char_) in zip(0..line.len(), line.chars()) {
            locations[(y, idx)] = char_;
            if char_ != '.' {
                if !antennas.contains_key(&char_) {
                    antennas.insert(
                        char_,
                        vec![(y.try_into().unwrap(), idx.try_into().unwrap())],
                    );
                } else {
                    antennas
                        .get_mut(&char_)
                        .unwrap()
                        .push((y.try_into().unwrap(), idx.try_into().unwrap()));
                }
            }
        }
    }
    (locations, antennas)
}

fn in_bounds(pos: (isize, isize), bounds: (usize, usize)) -> bool {
    pos.0 >= 0
        && pos.1 >= 0
        && pos.0 < bounds.0.try_into().unwrap()
        && pos.1 < bounds.1.try_into().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (locations, antennas) = parse_input(input);
    let bounds = (locations.nrows(), locations.ncols());
    let mut antinodes = nalgebra::DMatrix::from_element(bounds.0, bounds.1, 0);
    for (_, positions) in antennas {
        for pos in positions.into_iter().combinations(2) {
            let a = pos[0];
            let b = pos[1];
            let diff = (a.0 - b.0, a.1 - b.1);
            let check_1 = (a.0 + diff.0, a.1 + diff.1);
            let check_2 = (b.0 - diff.0, b.1 - diff.1);
            if in_bounds(check_1, bounds) {
                let convert = (check_1.0.try_into().unwrap(), check_1.1.try_into().unwrap());
                antinodes[convert] = 1;
            }
            if in_bounds(check_2, bounds) {
                let convert = (check_2.0.try_into().unwrap(), check_2.1.try_into().unwrap());
                antinodes[convert] = 1;
            }
        }
    }
    println!("{}", antinodes);
    Some(antinodes.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (locations, antennas) = parse_input(input);
    let bounds = (locations.nrows(), locations.ncols());
    let mut antinodes = nalgebra::DMatrix::from_element(bounds.0, bounds.1, 0);
    for (_, positions) in antennas {
        for pos in positions.into_iter().combinations(2) {
            let a = pos[0];
            let b = pos[1];
            let diff = (a.0 - b.0, a.1 - b.1);
            let mut check = (a.0, a.1);
            while in_bounds(check, bounds) {
                let convert = (check.0.try_into().unwrap(), check.1.try_into().unwrap());
                antinodes[convert] = 1;
                check = (check.0 + diff.0, check.1 + diff.1);
            }
            check = (b.0, b.1);
            while in_bounds(check, bounds) {
                let convert = (check.0.try_into().unwrap(), check.1.try_into().unwrap());
                antinodes[convert] = 1;
                check = (check.0 - diff.0, check.1 - diff.1);
            }
        }
    }
    println!("{}", antinodes);
    Some(antinodes.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
