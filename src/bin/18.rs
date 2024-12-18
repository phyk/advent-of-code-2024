use std::i32;

advent_of_code::solution!(18);

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut out = vec![];
    for line in input.lines() {
        let mut coords = line.split(",");
        let x = usize::from_str_radix(coords.next().unwrap(), 10).unwrap();
        let y = usize::from_str_radix(coords.next().unwrap(), 10).unwrap();
        out.push((y, x));
    }
    out
}

fn visit(coords: (usize, usize), cost: i32, matrix: &mut nalgebra::DMatrix<i32>) {
    *matrix.get_mut(coords).unwrap() = cost;
    let mut neighbors = vec![];
    if coords.0 > 0 {
        neighbors.push((coords.0 - 1, coords.1));
    }
    if coords.0 < matrix.nrows() - 1 {
        neighbors.push((coords.0 + 1, coords.1));
    }
    if coords.1 > 0 {
        neighbors.push((coords.0, coords.1 - 1));
    }
    if coords.1 < matrix.ncols() - 1 {
        neighbors.push((coords.0, coords.1 + 1));
    }
    for neighbor in neighbors {
        if matrix[neighbor] != -1 && matrix[neighbor] > cost + 1 {
            visit(neighbor, cost + 1, matrix);
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let dimsize = 7;
    let blocknum = 12;
    // let blocknum = 1024;
    // let dimsize = 71;
    let mut matrix = nalgebra::DMatrix::from_element(dimsize, dimsize, i32::MAX);
    let blocks = parse_input(input);
    for block in &blocks[..blocknum] {
        *matrix.get_mut(*block).unwrap() = -1;
    }
    visit((0, 0), 0, &mut matrix);

    Some(matrix[(dimsize - 1, dimsize - 1)])
}

pub fn part_two(input: &str) -> Option<String> {
    let dimsize = 7;
    // let dimsize = 71;
    let matrix = nalgebra::DMatrix::from_element(dimsize, dimsize, i32::MAX);
    let blocks = parse_input(input);
    let mut upper_bound_reachable = 0;
    let mut lower_bound_unreachable = blocks.len();
    let mut iteration = 0;
    while upper_bound_reachable + 1 != lower_bound_unreachable {
        let blocknum = (lower_bound_unreachable - upper_bound_reachable) / 2;
        let mut current_matrix = matrix.clone();
        for block in &blocks[..upper_bound_reachable + blocknum] {
            *current_matrix.get_mut(*block).unwrap() = -1;
        }
        visit((0, 0), 0, &mut current_matrix);
        if current_matrix[(dimsize - 1, dimsize - 1)] < i32::MAX
            && current_matrix[(dimsize - 1, dimsize - 1)] > -1
        {
            upper_bound_reachable += blocknum;
        } else {
            lower_bound_unreachable = upper_bound_reachable + blocknum;
        }
        iteration += 1;
        if iteration == 20 {
            break;
        }
    }
    let critical_block = blocks[upper_bound_reachable];

    Some(format!("{},{}", critical_block.1, critical_block.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".into()));
    }
}
