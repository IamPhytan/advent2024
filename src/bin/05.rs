advent_of_code::solution!(5);

fn get_middle_page(pages: &Vec<i32>) -> i32 {
    return pages.get((pages.len() - 1) / 2).unwrap().to_owned();
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines: Vec<_> = input.lines().map(|s| s.to_string()).collect();
    let limit = lines.iter().position(|l| l == "").unwrap();
    let rules: Vec<Vec<i32>> = lines[..limit]
        .iter()
        .map(|l| l.split("|").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    let updates: Vec<Vec<i32>> = lines[limit + 1..]
        .iter()
        .map(|l| l.split(",").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    None
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
