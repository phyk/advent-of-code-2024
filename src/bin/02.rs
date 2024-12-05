advent_of_code::solution!(2);
use std::usize;

use regex::Regex;

fn has_jump(value: u32, comparison: u32) -> bool {
    ((value + 3) < comparison) | (value > (comparison + 3))
}

fn wrong_direction(value: u32, comparison: u32, direction: Option<bool>) -> bool {
    if direction.is_some_and(|f| f != (comparison < value)) {
        true
    } else {
        false
    }
}

fn is_safe(vec_: &Vec<u32>) -> (bool, Option<usize>) {
    let mut direction: Option<bool> = None;
    let mut index = 1;
    while index < vec_.len() {
        let last_value = vec_[index - 1];
        let current_value = vec_[index];
        if has_jump(current_value, last_value)
            | (current_value == last_value)
            | wrong_direction(current_value, last_value, direction)
        {
            return (false, Some(index));
        }
        if direction.is_none() {
            direction = Some(last_value < current_value);
        }
        index += 1;
    }
    return (true, None);
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"([0-9 ]+)\n").unwrap();
    let re_elements = Regex::new(r"(\d+)\b").unwrap();
    let mut num_safe_results = 0;
    for (_, [value_line]) in re.captures_iter(input).map(|c| c.extract()) {
        let mut values = vec![];

        for (_, [value]) in re_elements.captures_iter(value_line).map(|c| c.extract()) {
            let current_value = usize::from_str_radix(value, 10).unwrap();
            values.push(current_value.try_into().unwrap());
        }
        let (safety, _) = is_safe(&values);
        if safety {
            num_safe_results += 1;
        }
    }
    Some(num_safe_results)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"([0-9 ]+)\n").unwrap();
    let re_elements = Regex::new(r"(\d+)\b").unwrap();
    let mut num_safe_results = 0;
    for (_, [value_line]) in re.captures_iter(input).map(|c| c.extract()) {
        let mut values: Vec<u32> = vec![];

        for (_, [value]) in re_elements.captures_iter(value_line).map(|c| c.extract()) {
            let current_value = usize::from_str_radix(value, 10).unwrap();
            values.push(current_value.try_into().unwrap());
        }
        let (safety, index) = is_safe(&values);
        if safety {
            num_safe_results += 1;
        } else {
            let mut values_without_prev = values.clone();
            let mut values_without_curr = values.clone();
            let mut values_without_firs = values.clone();
            values_without_prev.remove(index.unwrap() - 1);
            values_without_curr.remove(index.unwrap());
            values_without_firs.remove(0);
            let (safety_without_prev, _) = is_safe(&values_without_prev);
            let (safety_without_curr, _) = is_safe(&values_without_curr);
            let (safety_without_firs, _) = is_safe(&values_without_firs);
            if safety_without_prev | safety_without_curr | safety_without_firs {
                num_safe_results += 1;
            }
        }
    }
    Some(num_safe_results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4 + 8));
    }
}
