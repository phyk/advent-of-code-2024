use std::{collections::HashMap, usize};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut checksum = 0;
    let mut x = 0;
    let mut currently_block = true;
    let mut current_idx = 0;
    let mut y = input.len() - 2;
    let mut y_value = usize::from_str_radix(&input[y..y + 1], 10).unwrap();
    while x <= y {
        let current_block_length = usize::from_str_radix(&input[x..x + 1], 10).unwrap();
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
                    y_value = usize::from_str_radix(&input[y..y + 1], 10).unwrap();
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
    let mut checksum = 0;
    let mut index = 0;
    let mut set_groups = vec![false; input.len()];
    let blocks: Vec<(usize, usize)> = input
        .chars()
        .zip(0..input.len())
        .filter(|(c, _)| *c != '\n')
        .map(|(c, idx)| (usize::from_str_radix(&format!("{c}"), 10).unwrap(), idx))
        .collect();
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (block_length, idx) in &blocks {
        if idx % 2 == 0 {
            if map.contains_key(&block_length) {
                map.get_mut(&block_length)?.push(idx / 2);
            } else {
                map.insert(*block_length, vec![idx / 2]);
            }
        }
    }
    for (mut current_block_length, idx) in blocks {
        if idx % 2 == 0 {
            if set_groups[idx] {
                index += current_block_length;
                continue;
            }
            let checkadd: usize = (index..index + current_block_length)
                .map(|idx_| idx_ * (idx / 2))
                .sum();
            checksum += checkadd;
            index += current_block_length;
            set_groups[idx] = true;
        } else {
            while current_block_length > 0 {
                let block = (1..current_block_length + 1)
                    .map(|v| (v, *map[&v].last().unwrap_or(&0)))
                    .max_by(|(_, idx), (_, idx_2)| idx.cmp(idx_2));
                if block.is_some() {
                    let (selected_len, value) = block.unwrap();
                    if value != 0 {
                        map.get_mut(&selected_len)?.pop();
                        set_groups[value * 2] = true;
                        let checkadd: usize =
                            (index..index + selected_len).map(|idx| idx * (value)).sum();
                        checksum += checkadd;
                        index += selected_len;
                        current_block_length -= selected_len;
                    } else {
                        index += current_block_length;
                        break;
                    }
                }
            }
        }
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
