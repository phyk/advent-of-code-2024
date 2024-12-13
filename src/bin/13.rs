advent_of_code::solution!(13);

fn parse_xy(input: &str) -> i64 {
    i64::from_str_radix(&input[2..], 10).unwrap()
}

fn parse_button(input: Option<&str>) -> Option<(i64, i64)> {
    if input.is_none() {
        return None;
    }
    let left_right: Vec<&str> = input.unwrap().split(": ").collect();
    let items: Vec<&str> = left_right[1].split(", ").collect();
    Some((parse_xy(items[0]), parse_xy(items[1])))
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut lines = input.lines();
    let mut sum_possible = 0;
    loop {
        let a = parse_button(lines.next());
        let b = parse_button(lines.next());
        let prize = parse_button(lines.next());
        lines.next();
        if a.is_none() {
            break;
        }
        sum_possible += solve_machine(a.unwrap(), b.unwrap(), prize.unwrap(), 0);
    }
    Some(sum_possible)
}

fn solve_machine(a: (i64, i64), b: (i64, i64), prize: (i64, i64), offset: i64) -> i64 {
    let prize = (prize.0 + offset, prize.1 + offset);
    let det = a.0 * b.1 - a.1 * b.0;
    let a_ = (prize.0 * b.1 - prize.1 * b.0) / det;
    let b_ = (a.0 * prize.1 - a.1 * prize.0) / det;
    if (a.0 * a_ + b.0 * b_, a.1 * a_ + b.1 * b_) == (prize.0, prize.1) {
        a_ * 3 + b_
    } else {
        0
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut lines = input.lines();
    let mut sum_possible = 0;
    loop {
        let a = parse_button(lines.next());
        let b = parse_button(lines.next());
        let prize = parse_button(lines.next());
        lines.next();
        if a.is_none() {
            break;
        }
        sum_possible += solve_machine(a.unwrap(), b.unwrap(), prize.unwrap(), 10000000000000);
    }
    Some(sum_possible)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
