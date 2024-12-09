advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut checksum = 0;
    let mut x = 0;
    let mut currently_block = true;
    let mut current_idx = 0;
    let mut y = input.len() - 2;
    let mut y_value = usize::from_str_radix(&input[y..y+1], 10).unwrap();
    while x <= y {
        let current_block_length = usize::from_str_radix(&input[x..x+1], 10).unwrap();
        if currently_block {
            let mut end_index = current_block_length;
            if x == y {
                end_index = y_value;
            }
            for idx in 0..end_index {
                checksum += x / 2 * (idx + current_idx);
            }
        } else {
            for idx in 0..current_block_length {
                checksum += y / 2 * (idx + current_idx);
                y_value -= 1;
                if y_value == 0 {
                    y -= 2;
                    y_value = usize::from_str_radix(&input[y..y+1], 10).unwrap();
                }
            }
        }
        current_idx += current_block_length;
        currently_block = !currently_block;
        x += 1;
    }
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<usize> {
    return Some(0);
    let mut currently_block = true;
    let mut out_vec = vec![];
    let mut set_groups = vec![];
    for x in 0..input.len() - 1 {
        let mut current_block_length = usize::from_str_radix(&input[x..x+1], 10).unwrap();
        let mut y = input.len() - 2;
        if currently_block {
            currently_block = !currently_block;
            if set_groups.contains(&x) {
                out_vec.push((0, current_block_length));
                continue;
            }
            out_vec.push((x / 2, current_block_length));
            set_groups.push(x);
        } else {
            currently_block = !currently_block;
            while y > x && current_block_length > 0 {
                if set_groups.contains(&y) {
                    y -= 2;
                    continue;
                }
                let y_block_length: usize = usize::from_str_radix(&input[y..y+1], 10).unwrap();
                if y_block_length <= current_block_length {
                    current_block_length -= y_block_length;
                    out_vec.push((y / 2, y_block_length));
                    set_groups.push(y);
                }
                y -= 2;
            }
            if current_block_length > 0 {
                out_vec.push((0, current_block_length));
            }
        }
    }
    let mut checksum = 0;
    let mut idx = 0;
    for (id, num_elements) in out_vec {
        for local_idx in 0..num_elements {
            checksum += (idx + local_idx) * id;
        }
        idx += num_elements;
    }

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
