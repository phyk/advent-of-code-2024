advent_of_code::solution!(1);
use std::iter::zip;


pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let capacity = input.len() / 14;
    let mut values_1: Vec<u32> = Vec::with_capacity(capacity);
    let mut values_2: Vec<u32> = Vec::with_capacity(capacity);

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let mut values = line.split("   ");
        values_1.push(u32::from_str_radix(values.next()?, 10).unwrap());
        values_2.push(u32::from_str_radix(values.next()?, 10).unwrap());
    }
    values_1.sort();
    values_2.sort();

    let mut list_diff: u32 = u32::MIN;
    for (value_1, value_2) in zip(values_1, values_2) {
        list_diff += value_1.abs_diff(value_2)
    }
    Some(list_diff)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.split("\n");
    let mut num_elements: [usize;100000] = [0;100000];
    let capacity = input.len() / 14;
    let mut values_1: Vec<usize> = Vec::with_capacity(capacity);

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let mut values = line.split("   ");
        values_1.push(
            usize::from_str_radix(values.next().unwrap(), 10)
                .unwrap()
        );
        num_elements[usize::from_str_radix(values.next().unwrap(), 10)
        .unwrap()] += 1;
    }
    let mut result: usize = 0;
    for value in values_1 {
        result += &num_elements[value] * value;
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
