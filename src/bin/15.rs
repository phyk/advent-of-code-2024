use std::fmt::Display;

advent_of_code::solution!(15);

#[derive(Debug,Clone, PartialEq)]
enum Item {
    Wall,
    Object,
    Empty
}

#[derive(Debug,Clone, PartialEq)]
enum ItemTwo {
    Wall,
    ObjectLeft,
    ObjectRight,
    Empty,
    Position
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Empty => {write!(f, ".")},
            Item::Object => {write!(f, "O")},
            Item::Wall => {write!(f, "#")}
        }
    }
}

impl Display for ItemTwo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemTwo::Empty => {write!(f, ".")},
            ItemTwo::ObjectLeft => {write!(f, "[")},
            ItemTwo::ObjectRight => {write!(f, "]")},
            ItemTwo::Wall => {write!(f, "#")},
            ItemTwo::Position => {write!(f, "@")}
        }
    }
}

#[derive(Debug, PartialEq)]
enum Move {
    Up,
    Down,
    Right,
    Left
}

struct Warehouse{
    contents: nalgebra::DMatrix<Item>,
    moves: Vec<Move>
}

fn parse_moves(input: &str) -> Vec<Move> {
    let mut moves = vec![];
    for char in input.chars() {
        match char {
            '^' => moves.push(Move::Up),
            '>' => moves.push(Move::Right),
            '<' => moves.push(Move::Left),
            'v' => moves.push(Move::Down),
            _ => ()
        }
    }
    moves
}

fn parse_input(input: &str) -> (Warehouse, (usize, usize)) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let warehouse_init: Vec<&str> = parts[0].split("\n").collect();
    let mut out_matrix = nalgebra::DMatrix::from_element(warehouse_init.len(), warehouse_init[0].len(), Item::Empty);
    let mut start_position = (0, 0);
    for (y, line) in warehouse_init.into_iter().enumerate() {
        for (x, char) in line.chars().into_iter().enumerate() {
            match char {
                '#' => *out_matrix.get_mut((y,x)).unwrap() = Item::Wall,
                'O' => *out_matrix.get_mut((y,x)).unwrap() = Item::Object,
                '@' => start_position = (y, x),
                _ => ()
            }
        }
    }
    (Warehouse{contents: out_matrix, moves: parse_moves(parts[1])}, start_position)
}

fn parse_input_two(input: &str) -> (nalgebra::Matrix<ItemTwo, nalgebra::Dyn, nalgebra::Dyn, nalgebra::VecStorage<ItemTwo, nalgebra::Dyn, nalgebra::Dyn>>, Vec<Move>, (usize, usize)) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let warehouse_init: Vec<&str> = parts[0].split("\n").collect();
    let mut out_matrix = nalgebra::DMatrix::from_element(warehouse_init.len(), warehouse_init[0].len() * 2, ItemTwo::Empty);
    let mut start_position = (0, 0);
    for (y, line) in warehouse_init.into_iter().enumerate() {
        for (x, char) in line.chars().into_iter().enumerate() {
            match char {
                '#' => {
                    *out_matrix.get_mut((y,x * 2)).unwrap() = ItemTwo::Wall;
                    *out_matrix.get_mut((y,x * 2 +1)).unwrap() = ItemTwo::Wall;
                },
                'O' => {
                    *out_matrix.get_mut((y,x * 2)).unwrap() = ItemTwo::ObjectLeft;
                    *out_matrix.get_mut((y,x * 2 + 1)).unwrap() = ItemTwo::ObjectRight;
                },
                '@' => start_position = (y, x * 2),
                _ => ()
            }
        }
    }
    (out_matrix, parse_moves(parts[1]), start_position)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (warehouse, mut start) = parse_input(input);
    let mut contents = warehouse.contents.clone();
    for move_ in &warehouse.moves {
        start = try_move(start, move_, &mut contents);
    }
    Some(calc_gps_score(contents))
}

fn calc_gps_score(contents: nalgebra::Matrix<Item, nalgebra::Dyn, nalgebra::Dyn, nalgebra::VecStorage<Item, nalgebra::Dyn, nalgebra::Dyn>>) -> u32 {
    let mut sum_score = 0;
    for y in 0..contents.nrows() {
        for x in 0..contents.ncols() {
            if contents[(y, x)] == Item::Object {
                sum_score += y * 100 + x;
            }
        }
    }
    sum_score.try_into().unwrap()
}
fn calc_gps_score_two(contents: nalgebra::Matrix<ItemTwo, nalgebra::Dyn, nalgebra::Dyn, nalgebra::VecStorage<ItemTwo, nalgebra::Dyn, nalgebra::Dyn>>) -> u32 {
    let mut sum_score = 0;
    for y in 0..contents.nrows() {
        for x in 0..contents.ncols() {
            if contents[(y, x)] == ItemTwo::ObjectLeft {
                sum_score += y * 100 + x;
            }
        }
    }
    sum_score.try_into().unwrap()
}

fn movement_target(position: (usize, usize), dir: &Move) -> (usize, usize) {
    match r#dir {
        Move::Down => {
            (position.0 + 1, position.1)
        },
        Move::Left => {
            (position.0, position.1 - 1)
        },
        Move::Right => {
            (position.0, position.1 + 1)
        },
        Move::Up => {
            (position.0 - 1, position.1)
        }
    }
}

fn try_move(start: (usize, usize), dir: &Move, contents: &mut nalgebra::DMatrix<Item>) -> (usize, usize) {
    let new_position = movement_target(start, &dir);
    match contents[new_position] {
        Item::Wall => {
            return start;
        },
        Item::Empty => {
            return new_position;
        },
        Item::Object => {
            let mut empty_pos = new_position;
            while contents[empty_pos] == Item::Object {
                empty_pos = movement_target(empty_pos, &dir);
            }
            if contents[empty_pos] == Item::Empty {
                *contents.get_mut(new_position).unwrap() = Item::Empty;
                *contents.get_mut(empty_pos).unwrap() = Item::Object;
                return new_position;
            } else {
                return start;
            }
        }
    }
}
fn try_move_two(start: (usize, usize), dir: &Move, contents: &mut nalgebra::DMatrix<ItemTwo>) -> (usize, usize) {
    let new_position = movement_target(start, &dir);
    match contents[new_position] {
        ItemTwo::Wall => {
            return start;
        },
        ItemTwo::Empty => {
            return new_position;
        },
        ItemTwo::ObjectRight => {
            let empty_pos_l = (new_position.0, new_position.1 - 1);
            let empty_pos_r = new_position;
            if can_move(empty_pos_l, empty_pos_r, dir, contents) {
                move_object(empty_pos_l, new_position, dir, contents);
                return new_position;
            } else {
                return start;
            }
        },
        ItemTwo::ObjectLeft => {
            let empty_pos_l = new_position;
            let empty_pos_r = (new_position.0, new_position.1 + 1);
            if can_move(empty_pos_l, empty_pos_r, dir, contents) {
                move_object(empty_pos_l, empty_pos_r, dir, contents);
                return new_position;
            } else {
                return start;
            }
        }
        ItemTwo::Position => {
            return new_position;
        }
    }
}

fn can_move(empty_pos_l: (usize, usize), empty_pos_r: (usize, usize), dir: &Move, contents: &nalgebra::DMatrix<ItemTwo>) -> bool {
    let next_position_l  = movement_target(empty_pos_l, dir);
    let next_position_r  = movement_target(empty_pos_r, dir);
    if contents[next_position_l] == ItemTwo::Wall || contents[next_position_r] == ItemTwo::Wall {
        return false;
    }
    if contents[next_position_l] == ItemTwo::Empty && contents[next_position_r] == ItemTwo::Empty {
        return true;
    }
    if contents[next_position_l] == ItemTwo::ObjectLeft && contents[next_position_r] == ItemTwo::ObjectRight {
        return can_move(next_position_l, next_position_r, dir, contents);
    }
    let mut can_move_both = true;
    if contents[next_position_l] == ItemTwo::ObjectRight && *dir != Move::Right {
        can_move_both = can_move_both && can_move((next_position_l.0, next_position_l.1 - 1), next_position_l, dir, contents);
    }
    if contents[next_position_r] == ItemTwo::ObjectLeft && *dir != Move::Left {
        can_move_both = can_move_both && can_move(next_position_r, (next_position_r.0, next_position_r.1 + 1), dir, contents);
    }
    return can_move_both;
}

fn move_object(position_l: (usize, usize), position_r: (usize, usize), dir: &Move, contents: &mut nalgebra::DMatrix<ItemTwo>) {
    let next_position_l  = movement_target(position_l, dir);
    let next_position_r  = movement_target(position_r, dir);
    if contents[next_position_l] == ItemTwo::Wall || contents[next_position_r] == ItemTwo::Wall {
        println!("{:?} {:?}", next_position_l, next_position_r);
        panic!("Should not happen");
    }
    if contents[next_position_l] == ItemTwo::Empty && contents[next_position_r] == ItemTwo::Empty {
        *contents.get_mut(position_l).unwrap() = ItemTwo::Empty;
        *contents.get_mut(position_r).unwrap() = ItemTwo::Empty;
        *contents.get_mut(next_position_l).unwrap() = ItemTwo::ObjectLeft;
        *contents.get_mut(next_position_r).unwrap() = ItemTwo::ObjectRight;
        return
    }
    if contents[next_position_l] == ItemTwo::ObjectLeft && contents[next_position_r] == ItemTwo::ObjectRight {
        move_object(next_position_l, next_position_r, dir, contents);
    } else {
        if contents[next_position_l] == ItemTwo::ObjectRight && *dir != Move::Right {
            move_object((next_position_l.0, next_position_l.1 - 1), next_position_l, dir, contents);
        }
        if contents[next_position_r] == ItemTwo::ObjectLeft && *dir != Move::Left {
            move_object(next_position_r, (next_position_r.0, next_position_r.1 + 1), dir, contents);
        }
    }
    *contents.get_mut(position_l).unwrap() = ItemTwo::Empty;
    *contents.get_mut(position_r).unwrap() = ItemTwo::Empty;
    *contents.get_mut(next_position_l).unwrap() = ItemTwo::ObjectLeft;
    *contents.get_mut(next_position_r).unwrap() = ItemTwo::ObjectRight;

}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut contents, moves, mut start) = parse_input_two(input);
    for move_ in &moves {
        let mut for_print = contents.clone();
        *for_print.get_mut(start).unwrap() = ItemTwo::Position;
        start = try_move_two(start, move_, &mut contents);
    }
    Some(calc_gps_score_two(contents))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
