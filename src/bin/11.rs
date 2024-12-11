use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse_input(input: &str) -> HashMap<u128, usize> {
    let mut outmap: HashMap<u128, usize> = HashMap::new();
    for item in input[..input.len() - 1].split(" ") {
        let parsed = u128::from_str_radix(item, 10).unwrap();
        if outmap.contains_key(&parsed) {
            *outmap.get_mut(&parsed).unwrap() += 1;
        } else {
            outmap.insert(parsed, 1_usize);
        }
    }
    outmap
}

fn get_n_digits(number: &u128) -> u32 {
    let mut n_digits = 1;
    while number / 10_u128.pow(n_digits) > 0 {
        n_digits += 1;
    }
    n_digits
}

fn update_hashmap(item: u128, value: usize, map: &mut HashMap<u128, usize>) {
    if map.contains_key(&item) {
        *map.get_mut(&item).unwrap() += value;
    } else {
        map.insert(item, value);
    }
}

fn blink(current_map: &HashMap<u128, usize>) -> HashMap<u128, usize> {
    let mut new_map = HashMap::new();
    for (number, num_number) in current_map {
        if *number == 0_u128 {
            update_hashmap(1, *num_number, &mut new_map);
        } else {
            let n_digits = get_n_digits(&number);
            if n_digits % 2 == 0 {
                update_hashmap(
                    number / 10_u128.pow(n_digits / 2),
                    *num_number,
                    &mut new_map,
                );
                update_hashmap(
                    number % 10_u128.pow(n_digits / 2),
                    *num_number,
                    &mut new_map,
                );
            } else {
                update_hashmap(number * 2024, *num_number, &mut new_map);
            }
        }
    }
    new_map
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = parse_input(input);
    for _ in 0..25 {
        map = blink(&map);
    }
    let mut sum_entries = 0;
    for (_, num_elements) in map {
        sum_entries += num_elements;
    }
    Some(sum_entries)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map = parse_input(input);
    for _ in 0..75 {
        map = blink(&map);
    }
    let mut sum_entries = 0;
    for (_, num_elements) in map {
        sum_entries += num_elements;
    }
    Some(sum_entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
