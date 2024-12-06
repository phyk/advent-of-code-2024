advent_of_code::solution!(6);

use std::{collections::HashSet, iter::zip, vec};

use nalgebra::{self, DMatrix};
#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn increase (&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Eq for Direction {}
impl std::hash::Hash for Direction{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

fn next_obstacle(position: &(usize, usize), direction: &Direction, obstacles: &DMatrix<bool>) -> Option<(usize, usize)> {
    match direction {
        Direction::Down => {
            let obstacle_search = obstacles.column(position.1);
            let obstacle_position = (position.0 + 1..obstacle_search.len()).find(|i| obstacle_search[*i]);
            if obstacle_position.is_some() {
                Some((obstacle_position.unwrap() - 1, position.1))
            } else {
                None
            }
        },
        Direction::Up => {
            let obstacle_search = obstacles.column(position.1);
            let obstacle_position = (0..position.0-1).rev().find(|i| obstacle_search[*i]);
            if obstacle_position.is_some() {
                Some((obstacle_position.unwrap() + 1, position.1))
            } else {
                None
            }
        },
        Direction::Left => {
            let obstacle_search = obstacles.row(position.0);
            let obstacle_position = (0..position.1-1).rev().find(|i| obstacle_search[*i]);
            if obstacle_position.is_some() {
                Some((position.0, obstacle_position.unwrap() + 1))
            } else {
                None
            }
        },
        Direction::Right => {
            let obstacle_search = obstacles.row(position.0);
            let obstacle_position = (position.1+1..obstacle_search.len()).find(|i| obstacle_search[*i]);
            if obstacle_position.is_some() {
                Some((position.0, obstacle_position.unwrap() - 1))
            } else {
                None
            }
        }
    }
}


fn parse_input(input: &str) -> (nalgebra::DMatrix<bool>, (usize,usize)) {
    let lines: Vec<&str> = input.split("\n").collect();
    let line_length = lines[0].len();
    let n_lines = lines.len() - 1;
    let mut matrix = nalgebra::DMatrix::from_element(n_lines, line_length, false);
    let mut initial_position = (0,0);
    for (line, idx) in zip(lines, 0..n_lines) {
        for (c, pos) in zip(line.chars().into_iter(), 0..line_length) {
            if c == '#' {
                matrix[(idx, pos)] = true;
            }
            if c == '^' {
                initial_position = (idx, pos);
            }
        }
    }
    (matrix, initial_position)
}

fn expand_path(position: &(usize, usize), till: &Option<(usize, usize)>, direction: &Direction, bounds: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut path = vec![];
    match direction {
        Direction::Up => {
            let mut dest = 0;
            if till.is_some() {
                dest = till.unwrap().0 + 1;
            }
            for y in dest..position.0 + 1 {
                path.push((y, position.1));
            }
        },
        Direction::Down => {
            for y in position.0..till.unwrap_or(*bounds).0 {
                path.push((y, position.1));
            }
        },
        Direction::Left => {
            let mut dest = 0;
            if till.is_some() {
                dest = till.unwrap().1 + 1;
            }
            for x in dest..position.1 + 1 {
                path.push((position.0, x));
            }
        },
        Direction::Right => {
            for x in position.1..till.unwrap_or(*bounds).1 {
                path.push((position.0, x));
            }
        }
    }
    path
}

fn visit_till_next_obstacle(position: &(usize, usize), till: &Option<(usize, usize)>, direction: &Direction, visited: &mut DMatrix<i32>) {
    let bounds = visited.shape();
    for pos in expand_path(position, till, direction, &bounds) {
        visited[pos] = 1;
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let (obstacles,mut position) = parse_input(input);
    let bounds = (obstacles.nrows(), obstacles.ncols());
    let mut visited = nalgebra::DMatrix::from_element(bounds.0, bounds.1, 0);
    let mut direction = Direction::Up;

    loop {
        let before_next_obstacle = next_obstacle(&position, &direction, &obstacles);
        visit_till_next_obstacle(&position, &before_next_obstacle,&direction, &mut visited);
        if before_next_obstacle.is_none() {
            break;
        } else {
            direction = direction.increase();
            position = before_next_obstacle.unwrap();
        }
    }
    println!("{}", visited);
    Some(visited.sum())
}

fn is_cycle(position: &(usize, usize), direction: &Direction, obstacles: &DMatrix<bool>) {

}

pub fn part_two(input: &str) -> Option<u32> {
    let (obstacles,mut position) = parse_input(input);
    let bounds = (obstacles.nrows(), obstacles.ncols());
    let mut path = HashSet::new();
    let mut direction = Direction::Up;
    let mut cycles: u32 = 0;

    loop {
        let before_next_obstacle = next_obstacle(&position, &direction, &obstacles);
        if before_next_obstacle.is_none() {
            break;
        } else {
            path.insert((before_next_obstacle.unwrap().0, before_next_obstacle.unwrap().1, direction.clone()));
            direction = direction.increase();
            position = before_next_obstacle.unwrap();
        }
    }

    Some(cycles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
