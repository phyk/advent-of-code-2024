advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut sum = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        let a_usize = u32::from_str_radix(a, 10).unwrap();
        let b_usize = u32::from_str_radix(b, 10).unwrap();
        sum += a_usize * b_usize;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul|do|don't)\(([0-9,]*)\)").unwrap();
    println!("{}",re);
    let mut sum = 0;
    let mut enabled = true;
    for (_, [expr, value]) in re.captures_iter(input).map(|c| c.extract()) {
        match expr {
            "do" => enabled = true,
            "don't" => enabled = false,
            "mul" => {
                if enabled {
                    let split: Vec<&str> = value.split(",").collect();
                    let a = split[0];
                    let b = split[1];
                    let a_usize = u32::from_str_radix(a, 10).unwrap();
                    let b_usize = u32::from_str_radix(b, 10).unwrap();
                    sum += a_usize * b_usize;
                }
            }
            _ => ()
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
