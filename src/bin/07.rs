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

fn is_solvable(eq: &Equation) -> bool {
    let result = eq.result;
    let numbers = eq.numbers.clone();
    // Sum
    if numbers.iter().sum::<u64>() == result {
        return true;
    }
    // Product
    if numbers.iter().copied().reduce(|a, b| a * b).unwrap() == result {
        println!("Winner winner, chicken dinner");
        return true;
    }

    todo!("Other methods");

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);

    let result: u64 = equations
        .iter()
        .filter_map(|equation| {
            if is_solvable(&equation) {
                Some(equation.result)
            } else {
                None
            }
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
