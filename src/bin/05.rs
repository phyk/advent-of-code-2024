use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

struct InputParsed {
    rules: HashMap<usize, Vec<usize>>,
    samples: Vec<Vec<usize>>,
}

fn get_middle_number(sample: Vec<usize>) -> usize {
    let length = sample.len();
    let middle_index = (length - 1) / 2;
    *sample.get(middle_index).unwrap()
}

fn parse_input(input: &str) -> InputParsed {
    let lines = input.split("\n");
    let mut rules = HashMap::new();
    let mut samples = vec![];
    for line in lines {
        if line.contains("|") {
            let items: Vec<&str> = line.split("|").collect();
            let rule_index = usize::from_str_radix(items[1], 10).unwrap();
            if rules.contains_key(&rule_index) {
                let rules_for_index: &mut Vec<usize> = rules.get_mut(&rule_index).unwrap();
                rules_for_index.push(usize::from_str_radix(items[0], 10).unwrap());
            } else {
                rules.insert(
                    rule_index,
                    vec![usize::from_str_radix(items[0], 10).unwrap()],
                );
            }
        } else if line.contains(",") {
            let mut sample = vec![];
            for number in line.split(",") {
                sample.push(usize::from_str_radix(number, 10).unwrap());
            }
            samples.push(sample);
        }
    }
    InputParsed { rules, samples }
}

fn is_valid(sample: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> bool {
    let mut forbidden_items = vec![];
    for number in sample {
        if forbidden_items.contains(number) {
            return false;
        } else {
            if rules.contains_key(number) {
                for item in rules.get(number).unwrap() {
                    forbidden_items.push(*item);
                }
            }
        }
    }
    true
}

fn sort_values_rules(a: &usize, b: &usize, rules_a: &Vec<usize>, rules_b: &Vec<usize>) -> Ordering {
    if rules_a.contains(b) {
        Ordering::Greater
    } else if rules_b.contains(a) {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn reorder_sample(sample: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut sample_copy = sample.clone();
    let default = vec![];
    sample_copy
        .sort_by(|a, b| sort_values_rules(a, b, rules.get(a).unwrap_or_else(|| &default), rules.get(b).unwrap_or_else(|| &default)));
    sample_copy
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed_input = parse_input(input);
    let mut result: usize = 0;
    for sample in parsed_input.samples {
        if is_valid(&sample, &parsed_input.rules) {
            result += get_middle_number(sample);
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let parsed_input = parse_input(input);
    let mut result: usize = 0;
    for sample in parsed_input.samples {
        if !is_valid(&sample, &parsed_input.rules) {
            let reordered_sample = reorder_sample(&sample, &parsed_input.rules);
            result += get_middle_number(reordered_sample);
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
