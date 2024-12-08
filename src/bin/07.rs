advent_of_code::solution!(7);

#[derive(Debug, Clone)]
struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let (res, nums) = line.split_once(':').unwrap();
            Equation {
                result: res.parse::<u64>().unwrap(),
                numbers: nums
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            }
        })
        .collect::<Vec<Equation>>()
}

fn is_solvable(eq: &Equation, has_concat: bool) -> bool {
    let result = eq.result;
    let numbers = eq.numbers.clone();

    eval(result, numbers[0], &numbers[1..], has_concat)
}

fn eval(result: u64, accum: u64, vals: &[u64], has_concat: bool) -> bool {
    if vals.is_empty() {
        return accum == result;
    }

    eval(result, accum + vals[0], &vals[1..], has_concat)
        || eval(result, accum * vals[0], &vals[1..], has_concat)
        || (has_concat && eval(result, concat(accum, vals[0]), &vals[1..], true))
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10_u64.pow(b.ilog(10) + 1) + b
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);

    let result: u64 = equations
        .iter()
        .filter_map(|equation| {
            if is_solvable(&equation, false) {
                Some(equation.result)
            } else {
                None
            }
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);

    let result: u64 = equations
        .iter()
        .filter_map(|equation| {
            if is_solvable(&equation, true) {
                Some(equation.result)
            } else {
                None
            }
        })
        .sum();

    Some(result)
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
