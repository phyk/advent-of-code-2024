advent_of_code::solution!(2);
use std::usize;

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"([0-9 ]+)\n").unwrap();
    let re_elements = Regex::new(r"(\d+)\b").unwrap();
    let mut num_safe_results = 0;
    for (_, [value_line]) in re.captures_iter(input).map(|c| c.extract()) {
        let mut last_value: Option<usize> = None;
        let mut direction: Option<bool> = None;
        let mut _condition = true;

        for (_, [value]) in re_elements.captures_iter(value_line).map(|c|c.extract()) {
            let current_value = usize::from_str_radix(value, 10).unwrap();
            if last_value.is_some() {
                if direction.is_some() {
                    // True designates up
                    if direction.unwrap() & (current_value < last_value.unwrap()) {
                        _condition = false;
                        break;
                    } else if !direction.unwrap() & (current_value > last_value.unwrap()) {
                        _condition = false;
                        break;
                    }
                } else {
                    direction = Some(last_value.unwrap() < current_value);
                }
                if current_value == last_value.unwrap() {
                    _condition = false;
                    break;
                }
                if direction.unwrap() {
                    if current_value > last_value.unwrap() + 3 {
                        _condition = false;
                        break;
                    }
                } else {
                    if current_value + 3 < last_value.unwrap() {
                        _condition = false;
                        break;
                    }
                }
            }
            last_value = Some(current_value);
        }
        if _condition {
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
        let mut last_value: Option<usize> = None;
        let mut direction: Option<bool> = None;
        let mut _skipped = false;
        let mut _condition = true;

        for (_, [value]) in re_elements.captures_iter(value_line).map(|c|c.extract()) {
            let current_value = usize::from_str_radix(value, 10).unwrap();
            if last_value.is_some() {
                if current_value == last_value.unwrap() {
                    if _skipped {
                        _condition = false;
                        break;
                    } else {
                        _skipped = true;
                        continue;
                    }
                }
                
                if direction.is_some() {
                    // True designates up
                    if direction.unwrap() & (current_value < last_value.unwrap()) {
                        if _skipped {
                            _condition = false;
                            break;
                        } else {
                            _skipped = true;
                            continue;
                        }
                    } else if !direction.unwrap() & (current_value > last_value.unwrap()) {
                        if _skipped {
                            _condition = false;
                            break;
                        } else {
                            _skipped = true;
                            continue;
                        }
                    }
                } else {
                    direction = Some(last_value.unwrap() < current_value);
                }
                if direction.unwrap() {
                    if current_value > last_value.unwrap() + 3 {
                        if _skipped {
                            _condition = false;
                            break;
                        } else {
                            _skipped = true;
                            continue;
                        }
                    }
                } else {
                    if current_value + 3 < last_value.unwrap() {
                        if _skipped {
                            _condition = false;
                            break;
                        } else {
                            _skipped = true;
                            continue;
                        }
                    }
                }
            }
            last_value = Some(current_value);
        }
        if _condition {
            num_safe_results += 1;
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }
}
