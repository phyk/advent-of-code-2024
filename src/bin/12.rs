advent_of_code::solution!(12);

use std::collections::VecDeque;

use nalgebra;

fn parse_input(
    input: &str,
) -> nalgebra::Matrix<
    char,
    nalgebra::Dyn,
    nalgebra::Dyn,
    nalgebra::VecStorage<char, nalgebra::Dyn, nalgebra::Dyn>,
> {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut matrix = nalgebra::DMatrix::from_element(lines.len() - 1, lines[0].len(), '.');
    for (y, line) in lines.into_iter().enumerate() {
        if line.len() == 0 {
            continue;
        }
        for (x, char) in line.chars().into_iter().enumerate() {
            matrix[(y, x)] = char;
        }
    }
    matrix
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn get_tuple(&self) -> (usize, usize) {
        (self.y, self.x)
    }
    fn up(&self) -> Option<Coord> {
        if self.y > 0 {
            Some(Coord {
                y: self.y - 1,
                x: self.x,
            })
        } else {
            None
        }
    }
    fn left(&self) -> Option<Coord> {
        if self.x > 0 {
            Some(Coord {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        }
    }
    fn right(&self, x_max: usize) -> Option<Coord> {
        if self.x + 1 < x_max {
            Some(Coord {
                x: self.x + 1,
                y: self.y,
            })
        } else {
            None
        }
    }
    fn down(&self, y_max: usize) -> Option<Coord> {
        if self.y + 1 < y_max {
            Some(Coord {
                x: self.x,
                y: self.y + 1,
            })
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = parse_input(input);
    let mut visited = nalgebra::DMatrix::from_element(matrix.nrows(), matrix.ncols(), false);
    let mut search_vec = VecDeque::from([Coord { y: 0, x: 0 }]);
    let mut sum_fence = 0;
    loop {
        let search_element = search_vec.pop_front();
        if search_element.is_none() {
            break;
        }
        let (current_fence_length, num_elements) = visit(
            search_element.unwrap(),
            &matrix,
            &mut visited,
            &mut search_vec,
        );
        sum_fence += current_fence_length * num_elements;
    }
    Some(sum_fence)
}

fn visit(
    unwrap: Coord,
    matrix: &nalgebra::DMatrix<char>,
    visited: &mut nalgebra::DMatrix<bool>,
    search_vec: &mut VecDeque<Coord>,
) -> (usize, usize) {
    let mut local_fence = 0;
    let mut num_elements = 1;
    if visited[unwrap.get_tuple()] {
        return (local_fence, 0);
    }
    *visited.get_mut(unwrap.get_tuple()).unwrap() = true;
    for option in [unwrap.up(), unwrap.left()] {
        match option {
            Some(coord) => {
                if matrix[unwrap.get_tuple()] == matrix[coord.get_tuple()] {
                    if !visited[coord.get_tuple()] {
                        let (add_local_fence, add_num_elements) =
                            visit(coord, matrix, visited, search_vec);
                        local_fence += add_local_fence;
                        num_elements += add_num_elements;
                    }
                } else {
                    local_fence += 1;
                }
            }
            None => {
                local_fence += 1;
            }
        }
    }
    for option in [unwrap.down(matrix.nrows()), unwrap.right(matrix.ncols())] {
        match option {
            Some(coord) => {
                if matrix[unwrap.get_tuple()] == matrix[coord.get_tuple()] {
                    if !visited[coord.get_tuple()] {
                        let (add_local_fence, add_num_elements) =
                            visit(coord, matrix, visited, search_vec);
                        local_fence += add_local_fence;
                        num_elements += add_num_elements;
                    }
                } else {
                    local_fence += 1;
                    if !visited[coord.get_tuple()] {
                        search_vec.push_back(coord);
                    }
                }
            }
            None => {
                local_fence += 1;
            }
        }
    }
    return (local_fence, num_elements);
}

fn is_end_edge(
    coord_beyond_edge: &Option<Coord>,
    coord_next_element_edge: &Option<Coord>,
    coord_beyond_next_element: &Option<Coord>,
    v: &char,
    matrix: &nalgebra::DMatrix<char>,
) -> bool {
    if (coord_beyond_edge.as_ref().is_none()
        || coord_beyond_edge
            .as_ref()
            .is_some_and(|cbe| matrix[cbe.get_tuple()] != *v))
        && (coord_next_element_edge.as_ref().is_none()
            || coord_next_element_edge
                .as_ref()
                .is_some_and(|cne| *v != matrix[cne.get_tuple()])
            || coord_next_element_edge.as_ref().is_some_and(|_| {
                coord_beyond_next_element
                    .as_ref()
                    .is_some_and(|cbne| matrix[cbne.get_tuple()] == *v)
            }))
    {
        return true;
    } else {
        return false;
    }
}

fn visit_two(
    unwrap: &Coord,
    matrix: &nalgebra::DMatrix<char>,
    visited: &mut nalgebra::DMatrix<bool>,
    search_vec: &mut VecDeque<Coord>,
) -> (usize, usize) {
    let mut local_fence = 0;
    let mut num_elements = 1;
    if visited[unwrap.get_tuple()] {
        return (local_fence, 0);
    }
    *visited.get_mut(unwrap.get_tuple()).unwrap() = true;
    let v = matrix[unwrap.get_tuple()];
    let up = unwrap.up();
    let down = unwrap.down(matrix.nrows());
    let left = unwrap.left();
    let right = unwrap.right(matrix.ncols());
    for (cbe, cne, cbne) in [
        (&up, &right, right.as_ref().map_or(None, |r| r.up())),
        (
            &right,
            &down,
            down.as_ref().map_or(None, |d| d.right(matrix.ncols())),
        ),
        (
            &down,
            &left,
            left.as_ref().map_or(None, |l| l.down(matrix.nrows())),
        ),
        (&left, &up, up.as_ref().map_or(None, |u| u.left())),
    ] {
        if is_end_edge(&cbe, cne, &cbne, &v, &matrix) {
            local_fence += 1;
        }
    }
    for dir in [&up, &left, &down, &right] {
        if dir
            .as_ref()
            .is_some_and(|d| matrix[d.get_tuple()] == matrix[unwrap.get_tuple()])
        {
            let (fence, elements) = visit_two(dir.as_ref().unwrap(), matrix, visited, search_vec);
            local_fence += fence;
            num_elements += elements;
        }
    }
    for dir in [down, right] {
        if dir
            .as_ref()
            .is_some_and(|d| matrix[d.get_tuple()] != matrix[unwrap.get_tuple()])
        {
            search_vec.push_back(dir.unwrap());
        }
    }
    return (local_fence, num_elements);
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = parse_input(input);
    let mut visited = nalgebra::DMatrix::from_element(matrix.nrows(), matrix.ncols(), false);
    let mut search_vec = VecDeque::from([Coord { y: 0, x: 0 }]);
    let mut sum_fence = 0;
    loop {
        let search_element = search_vec.pop_front();
        if search_element.is_none() {
            break;
        }
        let (current_fence_length, num_elements) = visit_two(
            &search_element.unwrap(),
            &matrix,
            &mut visited,
            &mut search_vec,
        );
        sum_fence += current_fence_length * num_elements;
    }
    Some(sum_fence)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
