advent_of_code::solution!(1);
pub fn part_one(input: &str) -> Option<u32> {
    let numbers: Vec<_> = input
        .lines()
        .map(|c| {
            c.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();
    let lists = advent_of_code::vectranspose(numbers);

    let mut vec_a: Vec<i32> = lists[0].clone();
    let mut vec_b = lists[1].clone();

    vec_a.sort();
    vec_b.sort();

    let diff: Vec<i32> = vec_a.into_iter().zip(vec_b).map(|(a, b)| (a - b)).collect();

    Some(
        diff.iter()
            .map(|&n| n.abs())
            .into_iter()
            .map(|n| n as u32)
            .sum(),
    )

    // Some(diff)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
