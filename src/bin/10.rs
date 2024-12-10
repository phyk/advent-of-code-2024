advent_of_code::solution!(10);
use std::iter::zip;

use nalgebra::DMatrix;
use std::collections::HashSet;

fn parse_input(input: &str) -> DMatrix<u8> {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut outmatrix = DMatrix::from_element(lines.len() - 1, lines[0].len(), 0);
    for (y, line) in zip(0..lines.len(), lines) {
        if line.len() == 0 {
            continue;
        }
        for x in 0..line.len() {
            outmatrix[(y, x)] = u8::from_str_radix(&line[x..x + 1], 10).unwrap();
        }
    }
    outmatrix
}

fn neighbor_sum<'a>(
    coords: (usize, usize),
    in_matrix: &'a DMatrix<u8>,
    state_matrix: &'a mut DMatrix<Option<HashSet<(usize, usize)>>>,
) -> &'a HashSet<(usize, usize)> {
    let mut local_set = HashSet::new();
    if in_matrix[coords] == 9 {
        let mut set = HashSet::new();
        set.insert(coords);
        state_matrix[coords] = Some(set);
        return state_matrix[coords].as_ref().unwrap();
    }
    if coords.0 > 0 {
        let check_coords = (coords.0 - 1, coords.1);
        if in_matrix[check_coords] == in_matrix[coords] + 1 {
            match &state_matrix[check_coords] {
                Some(value) => local_set.extend(value),
                None => local_set.extend(neighbor_sum(check_coords, in_matrix, state_matrix)),
            }
        }
    }
    if coords.0 + 1 < in_matrix.nrows() {
        let check_coords = (coords.0 + 1, coords.1);
        if in_matrix[check_coords] == in_matrix[coords] + 1 {
            match &state_matrix[check_coords] {
                Some(value) => local_set.extend(value),
                None => local_set.extend(neighbor_sum(check_coords, in_matrix, state_matrix)),
            }
        }
    }
    if coords.1 > 0 {
        let check_coords = (coords.0, coords.1 - 1);
        if in_matrix[check_coords] == in_matrix[coords] + 1 {
            match &state_matrix[check_coords] {
                Some(value) => local_set.extend(value),
                None => local_set.extend(neighbor_sum(check_coords, in_matrix, state_matrix)),
            }
        }
    }
    if coords.1 + 1 < in_matrix.ncols() {
        let check_coords = (coords.0, coords.1 + 1);
        if in_matrix[check_coords] == in_matrix[coords] + 1 {
            match &state_matrix[check_coords] {
                Some(value) => local_set.extend(value),
                None => local_set.extend(neighbor_sum(check_coords, in_matrix, state_matrix)),
            }
        }
    }
    state_matrix[coords] = Some(local_set);
    return state_matrix[coords].as_ref().unwrap();
}

pub fn part_one(input: &str) -> Option<u32> {
    let in_matrix = parse_input(input);
    let mut state_matrix = DMatrix::from_element(in_matrix.nrows(), in_matrix.ncols(), None);
    let mut out_sum: u32 = 0;
    for y in 0..in_matrix.nrows() {
        for x in 0..in_matrix.ncols() {
            if in_matrix[(y, x)] == 0 {
                let length: usize = neighbor_sum((y, x), &in_matrix, &mut state_matrix).len();
                out_sum += TryInto::<u32>::try_into(length).unwrap();
            }
        }
    }
    Some(out_sum)
}


fn neighbor_sum_two<'a>(
    coords: (usize, usize),
    in_matrix: &'a DMatrix<u8>,
    state_matrix: &'a mut DMatrix<Option<u32>>,
) -> &'a u32 {
    let mut local_set = 0;
    if in_matrix[coords] == 9 {
        state_matrix[coords] = Some(1);
        return state_matrix[coords].as_ref().unwrap();
    }
    if coords.0 > 0 {
        let check_coords = (coords.0 - 1, coords.1);
        if in_matrix[check_coords] == in_matrix[coords] + 1 {
            match &state_matrix[check_coords] {
                Some(value) => local_set += value,
                None => local_set += neighbor_sum_two(check_coords, in_matrix, state_matrix),
            }
        }
    }
    if coords.0 + 1 < in_matrix.nrows() {
        let check_coords = (coords.0 + 1, coords.1);
        if in_matrix[check_coords] == in_matrix[coords] + 1 {
            match &state_matrix[check_coords] {
                Some(value) => local_set += value,
                None => local_set += neighbor_sum_two(check_coords, in_matrix, state_matrix),
            }
        }
    }
    if coords.1 > 0 {
        let check_coords = (coords.0, coords.1 - 1);
        if in_matrix[check_coords] == in_matrix[coords] + 1 {
            match &state_matrix[check_coords] {
                Some(value) => local_set += value,
                None => local_set += neighbor_sum_two(check_coords, in_matrix, state_matrix),
            }
        }
    }
    if coords.1 + 1 < in_matrix.ncols() {
        let check_coords = (coords.0, coords.1 + 1);
        if in_matrix[check_coords] == in_matrix[coords] + 1 {
            match &state_matrix[check_coords] {
                Some(value) => local_set += value,
                None => local_set += neighbor_sum_two(check_coords, in_matrix, state_matrix),
            }
        }
    }
    state_matrix[coords] = Some(local_set);
    return state_matrix[coords].as_ref().unwrap();
}

pub fn part_two(input: &str) -> Option<u32> {
    let in_matrix = parse_input(input);
    let mut state_matrix = DMatrix::from_element(in_matrix.nrows(), in_matrix.ncols(), None);
    let mut out_sum: u32 = 0;
    for y in 0..in_matrix.nrows() {
        for x in 0..in_matrix.ncols() {
            if in_matrix[(y, x)] == 0 {
                out_sum += neighbor_sum_two((y, x), &in_matrix, &mut state_matrix);
            }
        }
    }
    Some(out_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
