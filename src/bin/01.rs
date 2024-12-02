advent_of_code::solution!(1);
use std::iter::zip;
use std::collections::HashMap;

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+)   (\d+)").unwrap();
    let capacity = input.len() / 14;
    let mut values_1: Vec<u32> = Vec::with_capacity(capacity);
    let mut values_2: Vec<u32> = Vec::with_capacity(capacity);

    for (_, [value_1, value_2]) in re.captures_iter(input).map(|c| c.extract()) {
        values_1.push(usize::from_str_radix(value_1, 10).unwrap().try_into().unwrap());
        values_2.push(usize::from_str_radix(value_2, 10).unwrap().try_into().unwrap());
    }
    values_1.sort();
    values_2.sort();

    let mut list_diff: u32 = u32::MIN;
    for (value_1, value_2) in zip(values_1, values_2) {
        if value_2 > value_1 {
            list_diff += value_2 - value_1;
        } else {
            list_diff += value_1 - value_2;
        }
    }
    Some(list_diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: HashMap<u32, u32> = HashMap::new();
    let re = Regex::new(r"(\d+)   (\d+)").unwrap();
    let capacity = input.len() / 14;
    let mut values_1: Vec<u32> = Vec::with_capacity(capacity);
    let mut values_2: Vec<u32> = Vec::with_capacity(capacity);

    for (_, [value_1, value_2]) in re.captures_iter(input).map(|c| c.extract()) {
        values_1.push(usize::from_str_radix(value_1, 10).unwrap().try_into().unwrap());
        values_2.push(usize::from_str_radix(value_2, 10).unwrap().try_into().unwrap());
    }
    for value_2 in values_2 {
        if map.contains_key(&value_2) {
            *map.get_mut(&value_2).unwrap() = map[&value_2] + 1;
        } else {
            map.insert(value_2, 1);
        }
    }
    let mut result: u32 = 0;
    for value_1 in values_1 {
        if map.contains_key(&value_1) {
            result += map[&value_1] * value_1;
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
