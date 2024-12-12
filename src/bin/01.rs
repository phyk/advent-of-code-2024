advent_of_code::solution!(1);

use rayon::{
    iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
    str::ParallelString,
};

pub fn part_one_parallel(input: &str) -> Option<u32> {
    let (mut values_1, mut values_2): (Vec<u32>, Vec<u32>) = input
        .par_lines()
        .map(|line: &str| -> (u32, u32) {
            let mut values = line.split("   ");
            (
                u32::from_str_radix(values.next().unwrap(), 10).unwrap(),
                u32::from_str_radix(values.next().unwrap(), 10).unwrap(),
            )
        })
        .unzip();
    values_1.par_sort();
    values_2.par_sort();

    let list_diff: u32 = values_1
        .into_par_iter()
        .zip(values_2)
        .map(|(a, b)| a.abs_diff(b))
        .sum();
    Some(list_diff)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut values_1, mut values_2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line: &str| -> (u32, u32) {
            let mut values = line.split("   ");
            (
                u32::from_str_radix(values.next().unwrap(), 10).unwrap(),
                u32::from_str_radix(values.next().unwrap(), 10).unwrap(),
            )
        })
        .unzip();
    values_1.sort();
    values_2.sort();

    let list_diff: u32 = values_1
        .into_iter()
        .zip(values_2)
        .map(|(a, b)| a.abs_diff(b))
        .sum();
    Some(list_diff)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.split("\n");
    let mut num_elements: [usize; 100000] = [0; 100000];
    let capacity = input.len() / 14;
    let mut values_1: Vec<usize> = Vec::with_capacity(capacity);

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let mut values = line.split("   ");
        values_1.push(usize::from_str_radix(values.next().unwrap(), 10).unwrap());
        num_elements[usize::from_str_radix(values.next().unwrap(), 10).unwrap()] += 1;
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
