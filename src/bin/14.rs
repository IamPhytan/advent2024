use regex::Regex;

advent_of_code::solution!(14);

use advent_of_code::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn new(position: Point, velocity: Point) -> Robot {
        Robot { position, velocity }
    }

    fn next(&self) -> Robot {
        Robot::new(self.position + self.velocity, self.velocity)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MapSize {
    width: usize,
    height: usize,
}

#[cfg(test)]
impl MapSize {
    fn new() -> MapSize {
        MapSize {
            width: 11,
            height: 7,
        }
    }
}

#[cfg(not(test))]
impl MapSize {
    fn new() -> MapSize {
        MapSize {
            width: 101,
            height: 103,
        }
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"[a-z]=(-?\d+),(-?\d+)").unwrap();
    input
        .lines()
        .map(|s| {
            re.captures_iter(s)
                .map(|c| c.extract::<2>())
                .map(|(_, v)| {
                    v.iter()
                        .map(|n| n.parse::<isize>().unwrap())
                        .collect::<Vec<isize>>()
                })
                .map(|v| Point::new(v[0], v[1]))
                .collect::<Vec<Point>>()
        })
        .map(|pts| Robot::new(pts[0], pts[1]))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let map_size = MapSize::new();
    let robots = parse_input(input);
    println!("{:?}", robots);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let map_size = MapSize::new();
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
