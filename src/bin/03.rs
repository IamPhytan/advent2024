use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let products = re
        .captures_iter(input)
        .map(|c| c.extract::<2>())
        .map(|(_, v)| {
            v.iter()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|v| v[0] * v[1]);

    Some(products.sum())
}

fn is_disabled(position: usize, ranges: &Vec<Vec<&usize>>) -> bool {
    ranges.iter().any(|c| &position > c[0] && &position < c[1])
}

pub fn part_two(input: &str) -> Option<u32> {
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();

    let dont_pos: Vec<_> = re_dont.find_iter(input).map(|m| m.start()).collect();
    let do_pos: Vec<_> = re_do.find_iter(input).map(|m| m.start()).collect();
    let inlen = input.len();

    let disabled_ranges: Vec<_> = dont_pos
        .iter()
        .map(|st| {
            [
                st,
                do_pos
                    .iter()
                    .filter(|val| val >= &st)
                    .min()
                    .unwrap_or(&inlen),
            ]
            .to_vec()
        })
        .collect();

    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let products = re_mul
        .captures_iter(input)
        .filter(|c| !is_disabled(c.get(0).unwrap().start(), &disabled_ranges))
        .map(|c| c.extract::<2>())
        .map(|(_, v)| {
            v.iter()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|v| v[0] * v[1]);

    Some(products.sum())
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
