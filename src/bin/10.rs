advent_of_code::solution!(10);

use array2d::Array2D;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let arr = input
        .lines()
        .map(|line| line.chars().map(|i| i.to_digit(10).unwrap()).collect())
        .collect();
    arr
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn is_in_map(&self, height: usize, width: usize) -> bool {
        let in_x = self.x >= 0 && self.x < (width as isize);
        let in_y = self.y >= 0 && self.y < (height as isize);
        in_x && in_y
    }
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
