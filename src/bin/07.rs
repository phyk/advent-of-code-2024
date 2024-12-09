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

fn recursive_apply(
    final_result: usize,
    current_result: usize,
    current_index: usize,
    all_numbers: &Vec<usize>,
) -> bool {
    if current_index == all_numbers.len() {
        if current_result == final_result {
            return true;
        } else {
            return false;
        }
    }
    if current_result > final_result {
        return false;
    }
    return recursive_apply(
        final_result,
        current_result + all_numbers[current_index],
        current_index + 1,
        all_numbers,
    ) || recursive_apply(
        final_result,
        current_result * all_numbers[current_index],
        current_index + 1,
        all_numbers,
    );
}

fn recursive_apply_two(
    final_result: usize,
    current_result: usize,
    current_index: usize,
    all_numbers: &Vec<usize>,
) -> bool {
    if current_index == all_numbers.len() {
        if current_result == final_result {
            return true;
        } else {
            return false;
        }
    }
    if current_result > final_result {
        return false;
    }
    let current_result_str: String = current_result.to_string();
    let number = all_numbers[current_index].to_string();
    let result = usize::from_str_radix(&(current_result_str + &number), 10).unwrap();
    return recursive_apply_two(
        final_result,
        current_result + all_numbers[current_index],
        current_index + 1,
        all_numbers,
    ) || recursive_apply_two(
        final_result,
        current_result * all_numbers[current_index],
        current_index + 1,
        all_numbers,
    ) || recursive_apply_two(final_result, result, current_index + 1, all_numbers);
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed_input = parse_input(input);
    let mut sum_valid = 0;
    for equation in parsed_input {
        if recursive_apply(equation.0, equation.1[0], 1, &equation.1) {
            sum_valid += equation.0;
        }
    }
    Some(sum_valid)
}

pub fn part_two(input: &str) -> Option<usize> {
    let parsed_input = parse_input(input);
    let mut sum_valid = 0;
    for equation in parsed_input {
        if recursive_apply_two(equation.0, equation.1[0], 1, &equation.1) {
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
