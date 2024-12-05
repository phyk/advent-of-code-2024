advent_of_code::solution!(4);

#[derive(Debug)]
enum Direction {
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
    Right,
    UpRight,
}

struct Input {
    n_rows: isize,
    n_cols: isize,
    content: String,
}

impl Input {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.split('\n').collect();
        let first_newline = lines[0].chars().count().try_into().unwrap();
        let length: isize = lines.len().try_into().unwrap();
        Input {
            n_rows: length - 1,
            n_cols: first_newline,
            content: input.into(),
        }
    }

    fn get_char(&self, x: isize, y: isize) -> &str {
        let position: usize = (x + ((self.n_cols + 1) * y)).try_into().unwrap();
        &self.content[position..position + 1]
    }

    fn is_valid(&self, x: isize, y: isize) -> bool {
        (x >= 0) && (y >= 0) && (x < self.n_cols) && (y < self.n_rows)
    }
}

fn get_available_directions(x: isize, y: isize, input: &Input) -> Vec<Direction> {
    let mut dirs = vec![];
    if input.is_valid(x, y - 3) {
        dirs.push(Direction::Up);
    }
    if input.is_valid(x - 3, y - 3) {
        dirs.push(Direction::UpLeft);
    }
    if input.is_valid(x - 3, y) {
        dirs.push(Direction::Left);
    }
    if input.is_valid(x - 3, y + 3) {
        dirs.push(Direction::DownLeft);
    }
    if input.is_valid(x, y + 3) {
        dirs.push(Direction::Down);
    }
    if input.is_valid(x + 3, y + 3) {
        dirs.push(Direction::DownRight);
    }
    if input.is_valid(x + 3, y) {
        dirs.push(Direction::Right);
    }
    if input.is_valid(x + 3, y - 3) {
        dirs.push(Direction::UpRight);
    }
    dirs
}

pub fn is_xmas(one: &str, two: &str, thre: &str, fou: &str) -> usize {
    if (one == "X") && (two == "M") && (thre == "A") && (fou == "S") {
        1
    } else {
        0
    }
}

pub fn is_x_mas(up_l: &str, up_r: &str, do_l: &str, do_r: &str) -> usize {
    if !matches!(up_l, "M" | "S")
        || !matches!(up_r, "M" | "S")
        || !matches!(do_l, "M" | "S")
        || !matches!(do_r, "M" | "S")
    {
        return 0;
    } else if (up_l == up_r) && (up_l != do_l) && (do_l == do_r) {
        return 1;
    } else if (up_l == do_l) && (up_l != up_r) && (up_r == do_r) {
        return 1;
    }
    return 0;
}

pub fn part_two(input: &str) -> Option<usize> {
    let parsed_input = Input::new(input);
    let mut num_xmas: usize = 0;
    for y_ in 0..parsed_input.n_rows {
        for x_ in 0..parsed_input.n_cols {
            let x: isize = x_.try_into().unwrap();
            let y: isize = y_.try_into().unwrap();
            if parsed_input.get_char(x, y) == "A" {
                if parsed_input.is_valid(x - 1, y - 1)
                    && parsed_input.is_valid(x - 1, y + 1)
                    && parsed_input.is_valid(x + 1, y + 1)
                    && parsed_input.is_valid(x + 1, y - 1)
                {
                    num_xmas += is_x_mas(
                        parsed_input.get_char(x - 1, y - 1),
                        parsed_input.get_char(x + 1, y - 1),
                        parsed_input.get_char(x - 1, y + 1),
                        parsed_input.get_char(x + 1, y + 1),
                    );
                }
            }
        }
    }
    Some(num_xmas)
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed_input = Input::new(input);
    let mut num_xmas: usize = 0;
    for y_ in 0..parsed_input.n_rows {
        for x_ in 0..parsed_input.n_cols {
            let x: isize = x_.try_into().unwrap();
            let y: isize = y_.try_into().unwrap();
            if parsed_input.get_char(x, y) == "X" {
                for direction in get_available_directions(x, y, &parsed_input) {
                    match direction {
                        Direction::Up => {
                            num_xmas += is_xmas(
                                parsed_input.get_char(x, y),
                                parsed_input.get_char(x, y - 1),
                                parsed_input.get_char(x, y - 2),
                                parsed_input.get_char(x, y - 3),
                            )
                        }
                        Direction::UpLeft => {
                            num_xmas += is_xmas(
                                parsed_input.get_char(x, y),
                                parsed_input.get_char(x - 1, y - 1),
                                parsed_input.get_char(x - 2, y - 2),
                                parsed_input.get_char(x - 3, y - 3),
                            )
                        }
                        Direction::Left => {
                            num_xmas += is_xmas(
                                parsed_input.get_char(x, y),
                                parsed_input.get_char(x - 1, y),
                                parsed_input.get_char(x - 2, y),
                                parsed_input.get_char(x - 3, y),
                            )
                        }
                        Direction::DownLeft => {
                            num_xmas += is_xmas(
                                parsed_input.get_char(x, y),
                                parsed_input.get_char(x - 1, y + 1),
                                parsed_input.get_char(x - 2, y + 2),
                                parsed_input.get_char(x - 3, y + 3),
                            )
                        }
                        Direction::Down => {
                            num_xmas += is_xmas(
                                parsed_input.get_char(x, y),
                                parsed_input.get_char(x, y + 1),
                                parsed_input.get_char(x, y + 2),
                                parsed_input.get_char(x, y + 3),
                            )
                        }
                        Direction::DownRight => {
                            num_xmas += is_xmas(
                                parsed_input.get_char(x, y),
                                parsed_input.get_char(x + 1, y + 1),
                                parsed_input.get_char(x + 2, y + 2),
                                parsed_input.get_char(x + 3, y + 3),
                            )
                        }
                        Direction::Right => {
                            num_xmas += is_xmas(
                                parsed_input.get_char(x, y),
                                parsed_input.get_char(x + 1, y),
                                parsed_input.get_char(x + 2, y),
                                parsed_input.get_char(x + 3, y),
                            )
                        }
                        Direction::UpRight => {
                            num_xmas += is_xmas(
                                parsed_input.get_char(x, y),
                                parsed_input.get_char(x + 1, y - 1),
                                parsed_input.get_char(x + 2, y - 2),
                                parsed_input.get_char(x + 3, y - 3),
                            )
                        }
                    }
                }
            }
        }
    }
    Some(num_xmas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
