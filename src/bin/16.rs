advent_of_code::solution!(16);
use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display, usize};

use nalgebra;

#[derive(Debug, Clone, PartialEq)]
enum State {
    Wall,
    Empty,
    Position,
    End
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Wall => write!(f, "#"),
            State::Empty => write!(f, "."),
            State::Position => write!(f, "^"),
            State::End => write!(f, "E"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}

#[derive(PartialEq, Debug)]
enum PathProperty {
    DeadEnd,
    End,
    Crossing
}

#[derive(Clone, Debug)]
struct Path {
    position: (usize, usize),
    direction: Direction,
    cost: usize,
    previous_positions: Vec<(usize, usize)>
}

fn move_position(position: &(usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Down => (position.0 + 1, position.1),
        Direction::Left => (position.0, position.1 - 1),
        Direction::Right => (position.0, position.1 + 1),
        Direction::Up => (position.0 - 1, position.1)
    }
}

fn turn_left(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
    }
}
fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
    }
}

impl Path {
    fn move_till_next_crossing(&mut self, grid: &nalgebra::DMatrix<State>) -> PathProperty {
        let possible_movement: Vec<((usize, usize), Direction)> = [
            (&self.position, self.direction.clone()),
            (&self.position, turn_left(&self.direction)),
            (&self.position, turn_right(&self.direction))
        ].iter().map(|(pos, d)| (move_position(*pos, d), d.clone()))
            .filter(|(p, _)| grid[*p] != State::Wall).collect();
        let num_possibilities = possible_movement.len();
        if grid[self.position] == State::End {
            return PathProperty::End;
        }
        
        if num_possibilities == 0 {
            return PathProperty::DeadEnd;
        }
        if num_possibilities > 1 {
            return PathProperty::Crossing;
        }
        if possible_movement[0].1 != self.direction {
            self.direction = possible_movement[0].1.clone();
            self.cost += 1000;
        }
        self.move_forward();

        return self.move_till_next_crossing(grid);
    }
    fn turn_left(&mut self) {
        self.cost += 1000;
        self.direction = turn_left(&self.direction);
    }
    fn turn_right(&mut self) {
        self.cost += 1000;
        self.direction = turn_right(&self.direction);
    }
    fn move_forward(&mut self) {
        self.cost += 1;
        self.position = move_position(&self.position, &self.direction);
        self.previous_positions.push(self.position);
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.direction == other.direction && self.cost == other.cost
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}
impl Eq for Path {}

fn parse_input(input: &str) -> (nalgebra::Matrix<State, nalgebra::Dyn, nalgebra::Dyn, nalgebra::VecStorage<State, nalgebra::Dyn, nalgebra::Dyn>>, (usize, usize)) {
    let lines: Vec<&str> = input.lines().collect();
    let mut matrix = nalgebra::DMatrix::from_element(lines.len(), lines[0].len(), State::Empty);
    let mut starting_position = (0, 0);
    for (y, line) in lines.into_iter().enumerate() {
        for (x, char_) in line.chars().into_iter().enumerate() {
            match char_ {
                '#' => {*matrix.get_mut((y, x)).unwrap() = State::Wall},
                'E' => {*matrix.get_mut((y, x)).unwrap() = State::End},
                'S' => {
                    *matrix.get_mut((y, x)).unwrap() = State::Position;
                    starting_position = (y, x);
                },
                _ => ()
            }
        }
    }
    (matrix, starting_position)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (matrix, position) = parse_input(input);
    let mut cost_matrix = nalgebra::DMatrix::from_element(matrix.nrows(), matrix.ncols(), usize::MAX);
    let mut min_cost = usize::MAX;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Path{position, direction: Direction::Right, cost: 0, previous_positions: vec![]}));
    while !heap.is_empty() && heap.peek().unwrap().0.cost < min_cost {
        let current_path = heap.pop().unwrap().0;
        if cost_matrix[current_path.position] < current_path.cost {
            continue;
        } else {
            *cost_matrix.get_mut(current_path.position).unwrap() = current_path.cost;
        }
        let mut new_path = current_path.clone();
        let next_pos = move_position(&new_path.position, &new_path.direction);
        new_path.position = next_pos;
        new_path.cost += 1;
        if matrix[next_pos] != State::Wall {
            let result = new_path.move_till_next_crossing(&matrix);
            if result == PathProperty::End {
                if new_path.cost < min_cost {
                    min_cost = new_path.cost;
                }
            } else if result == PathProperty::Crossing {
                heap.push(Reverse(new_path));
            }
        }
        new_path = current_path.clone();
        new_path.turn_right();
        new_path.move_forward();
        if matrix[new_path.position] != State::Wall {
            let result = new_path.move_till_next_crossing(&matrix);
            if result == PathProperty::End {
                if new_path.cost < min_cost {
                    min_cost = new_path.cost;
                }
            } else if result == PathProperty::Crossing {
                heap.push(Reverse(new_path));
            }
        }
        new_path = current_path.clone();
        new_path.turn_left();
        new_path.move_forward();
        if matrix[new_path.position] != State::Wall {
            let result = new_path.move_till_next_crossing(&matrix);
            if result == PathProperty::End {
                if new_path.cost < min_cost {
                    min_cost = new_path.cost;
                }
            } else if result == PathProperty::Crossing {
                heap.push(Reverse(new_path));
            }
        }
    }
    Some(min_cost)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (matrix, position) = parse_input(input);
    let mut cost_matrix = nalgebra::DMatrix::from_element(matrix.nrows(), matrix.ncols(), usize::MAX);
    let mut min_cost = usize::MAX;
    let mut min_cost_paths = vec![];
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Path{position, direction: Direction::Right, cost: 0, previous_positions: vec![position]}));
    while !heap.is_empty() && heap.peek().unwrap().0.cost < min_cost {
        let current_path = heap.pop().unwrap().0;
        if cost_matrix[current_path.position] != usize::MAX && cost_matrix[current_path.position] + 1000 < current_path.cost {
            continue;
        } else {
            *cost_matrix.get_mut(current_path.position).unwrap() = current_path.cost;
        }
        let mut new_path = current_path.clone();
        new_path.move_forward();
        if matrix[new_path.position] != State::Wall {
            let result = new_path.move_till_next_crossing(&matrix);
            if result == PathProperty::End {
                if new_path.cost < min_cost {
                    min_cost = new_path.cost;
                    min_cost_paths = vec![new_path.previous_positions];
                } else if new_path.cost == min_cost {
                    min_cost_paths.push(new_path.previous_positions);
                }
            } else if result == PathProperty::Crossing {
                heap.push(Reverse(new_path));
            }
        }
        new_path = current_path.clone();
        new_path.turn_right();
        new_path.move_forward();
        if matrix[new_path.position] != State::Wall {
            let result = new_path.move_till_next_crossing(&matrix);
            if result == PathProperty::End {
                if new_path.cost < min_cost {
                    min_cost = new_path.cost;
                    min_cost_paths = vec![new_path.previous_positions];
                } else if new_path.cost == min_cost {
                    min_cost_paths.push(new_path.previous_positions);
                }
            } else if result == PathProperty::Crossing {
                heap.push(Reverse(new_path));
            }
        }
        new_path = current_path.clone();
        new_path.turn_left();
        new_path.move_forward();
        if matrix[new_path.position] != State::Wall {
            let result = new_path.move_till_next_crossing(&matrix);
            if result == PathProperty::End {
                if new_path.cost < min_cost {
                    min_cost = new_path.cost;
                    min_cost_paths = vec![new_path.previous_positions];
                } else if new_path.cost == min_cost {
                    min_cost_paths.push(new_path.previous_positions);
                }
            } else if result == PathProperty::Crossing {
                heap.push(Reverse(new_path));
            }
        }
    }
    let mut path_matrix = nalgebra::DMatrix::from_element(matrix.nrows(), matrix.ncols(), 0);
    for path in min_cost_paths {
        for position in path {
            path_matrix[position] = 1;
        }
    }
    Some(path_matrix.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
