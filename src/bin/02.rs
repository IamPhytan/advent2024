advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let report_diff: Vec<_> = input
        .lines()
        .map(|report| {
            report
                .split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .windows(2)
                .map(|s| s[1] - s[0])
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();

    let same_sign: Vec<bool> = report_diff
        .iter()
        .map(|report| {
            report
                .iter()
                .map(|x| x.signum())
                .all(|x| x != 0 && x == report[0].signum())
        })
        .collect();

    let safe_diff: Vec<bool> = report_diff
        .iter()
        .map(|report| report.iter().map(|x| x.abs()).all(|x| (1..4).contains(&x)))
        .collect();

    Some(
        same_sign
            .iter()
            .zip(safe_diff)
            .filter(|(&x, y)| x && *y)
            .count() as u32,
        // .collect::<bool>()
    )
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
