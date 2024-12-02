use std::fmt::Debug;

advent_of_code::solution!(2);

fn is_safe_report(report: &Vec<i32>) -> bool {
    let report_diff = report.windows(2).map(|s| s[1] - s[0]).collect::<Vec<i32>>();

    let same_sign = report_diff
        .iter()
        .map(|x| x.signum())
        .all(|x| x != 0 && x == report_diff[0].signum());

    let safe_diff = report_diff
        .iter()
        .map(|x| x.abs())
        .all(|x| (1..4).contains(&x));

    safe_diff && same_sign
}

fn combinations<T: Clone + Debug>(vec: &[T]) -> Vec<Vec<T>> {
    let mut combinations = vec![vec.to_vec()];
    for i in 0..vec.len() {
        let mut combination = vec.to_vec();
        combination.remove(i);
        combinations.push(combination);
    }
    combinations
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports: Vec<_> = input
        .lines()
        .map(|report| {
            report
                .split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();

    Some(reports.iter().filter(|&x| is_safe_report(x)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports: Vec<_> = input
        .lines()
        .map(|report| {
            report
                .split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();

    let valid_reports: Vec<_> = reports
        .iter()
        .filter(|rep| combinations(rep).iter().any(|comb| is_safe_report(comb)))
        .collect();

    Some(valid_reports.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
