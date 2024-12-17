advent_of_code::solution!(14);

struct Robot {
    position: (i64, i64),
    speed: (i64, i64),
}

fn get_xy(input: &str) -> (i64, i64) {
    let without_left: Vec<&str> = input.split("=").collect();
    let parts: Vec<&str> = without_left[1].split(",").collect();
    (
        i64::from_str_radix(parts[0], 10).unwrap(),
        i64::from_str_radix(parts[1], 10).unwrap(),
    )
}

fn parse_robot(input: &str) -> Robot {
    let mut parts = input.split(" ");
    Robot {
        position: get_xy(parts.next().unwrap()),
        speed: get_xy(parts.next().unwrap()),
    }
}

fn move_robot(robot: &Robot, num_moves: i64) -> (i64, i64) {
    let position = robot.position;
    (
        position.0 + robot.speed.0 * num_moves,
        position.1 + robot.speed.1 * num_moves,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    // let area = (11, 7);
    let area = (101, 103);
    let mut quadrant_robots = vec![0; 4];
    for robot_input in input.lines() {
        let robot = parse_robot(robot_input);
        let mut position = move_robot(&robot, 100);
        position.0 = position.0 % area.0;
        if position.0 < 0 {
            position.0 += area.0;
        }
        position.1 = position.1 % area.1;
        if position.1 < 0 {
            position.1 += area.1;
        }
        let mut index = 0;
        if position.0 == area.0 / 2 || position.1 == area.1 / 2 {
            continue;
        }
        if position.0 > area.0 / 2 {
            index += 1;
        }
        if position.1 > area.1 / 2 {
            index += 2;
        }
        *quadrant_robots.get_mut(index).unwrap() += 1;
    }
    Some(quadrant_robots[0] * quadrant_robots[1] * quadrant_robots[2] * quadrant_robots[3])
}

fn show_iteration(
    robots: &Vec<Robot>,
    time: u32,
    area: (usize, usize),
) -> nalgebra::Matrix<
    char,
    nalgebra::Dyn,
    nalgebra::Dyn,
    nalgebra::VecStorage<char, nalgebra::Dyn, nalgebra::Dyn>,
> {
    let area_i: (i64, i64) = (area.0.try_into().unwrap(), area.1.try_into().unwrap());
    let mut matrix = nalgebra::DMatrix::from_element(area.1, area.0, '.');
    for robot in robots {
        let mut position = move_robot(robot, time.into());
        position.0 = position.0 % area_i.0;
        if position.0 < 0 {
            position.0 += area_i.0;
        }
        position.1 = position.1 % area_i.1;
        if position.1 < 0 {
            position.1 += area_i.1;
        }
        let position_matrix: (usize, usize) = (
            position.1.try_into().unwrap(),
            position.0.try_into().unwrap(),
        );
        *matrix.get_mut(position_matrix).unwrap() = '#';
    }
    matrix
}

pub fn part_two(input: &str) -> Option<u32> {
    // let area = (11, 7);
    let area = (101, 103);
    let robots: Vec<Robot> = input.lines().map(|l| parse_robot(l)).collect();
    let mut result = 0;
    for moves in 0..10000 {
        let matrix = show_iteration(&robots, moves, area);
        if find_area(&matrix) {
            result = moves;
            // println!("{}", matrix);
        }
    }
    Some(result)
}

fn find_area(
    matrix: &nalgebra::Matrix<
        char,
        nalgebra::Dyn,
        nalgebra::Dyn,
        nalgebra::VecStorage<char, nalgebra::Dyn, nalgebra::Dyn>,
    >,
) -> bool {
    for y in 0..matrix.nrows() - 4 {
        for x in 0..matrix.ncols() - 1 {
            if matrix[(y, x)] == '#'
                && matrix[(y + 1, x)] == '#'
                && matrix[(y + 2, x)] == '#'
                && matrix[(y + 3, x)] == '#'
                && matrix[(y + 4, x)] == '#'
                && matrix[(y, x + 1)] == '#'
                && matrix[(y + 1, x + 1)] == '#'
                && matrix[(y + 2, x + 1)] == '#'
                && matrix[(y + 3, x + 1)] == '#'
                && matrix[(y + 4, x + 1)] == '#'
            {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
