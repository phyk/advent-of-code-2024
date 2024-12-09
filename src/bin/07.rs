advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    let lines = input.split("\n");
    let mut equations = vec![];
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let mut first_split = line.split(": ");
        let result = usize::from_str_radix(first_split.next().unwrap(), 10).unwrap();
        let mut numbers = vec![];
        for number in first_split.next().unwrap().split(" ") {
            numbers.push(usize::from_str_radix(number, 10).unwrap());
        }
        equations.push((result, numbers));
    }
    equations
}

fn recursive_apply(current_result: usize, current_index: usize, all_numbers: &Vec<usize>) -> bool {
    if current_index == 0 {
        if current_result == all_numbers[current_index] {
            return true;
        } else {
            return false;
        }
    }
    if current_result % all_numbers[current_index] == 0
        && recursive_apply(
            current_result / all_numbers[current_index],
            current_index - 1,
            all_numbers,
        )
    {
        return true;
    } else if 0 + all_numbers[current_index] <= current_result {
        return recursive_apply(
            current_result - all_numbers[current_index],
            current_index - 1,
            all_numbers,
        );
    } else {
        return false;
    }
}

fn find_pow10(number: usize) -> usize {
    let mut value = 10;
    while number / value > 0 {
        value = value * 10;
    }
    value
}

fn recursive_apply_two(
    current_result: usize,
    current_index: usize,
    all_numbers: &Vec<usize>,
) -> bool {
    if current_index == 0 {
        if current_result == all_numbers[current_index] {
            return true;
        } else {
            return false;
        }
    }
    let n_digits = find_pow10(all_numbers[current_index]);
    if current_result % all_numbers[current_index] == 0
        && recursive_apply_two(
            current_result / all_numbers[current_index],
            current_index - 1,
            all_numbers,
        )
    {
        return true;
    } else if current_result % n_digits == all_numbers[current_index]
        && recursive_apply_two(current_result / n_digits, current_index - 1, all_numbers)
    {
        return true;
    } else if 0 + all_numbers[current_index] <= current_result {
        return recursive_apply_two(
            current_result - all_numbers[current_index],
            current_index - 1,
            all_numbers,
        );
    } else {
        return false;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed_input = parse_input(input);
    let mut sum_valid = 0;
    for equation in parsed_input {
        if recursive_apply(equation.0, equation.1.len() - 1, &equation.1) {
            sum_valid += equation.0;
        }
    }
    Some(sum_valid)
}

pub fn part_two(input: &str) -> Option<usize> {
    let parsed_input = parse_input(input);
    let mut sum_valid = 0;
    for equation in parsed_input {
        if recursive_apply_two(equation.0, equation.1.len() - 1, &equation.1) {
            sum_valid += equation.0;
        }
    }
    Some(sum_valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
