use array2d::Array2D;
use regex::Regex;

advent_of_code::solution!(14);

use advent_of_code::Point;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Map {
    grid: Array2D<u32>,
}

impl Map {
    fn new() -> Map {
        Map {
            #[cfg(test)]
            grid: Array2D::filled_with(0, 7, 11),
            #[cfg(not(test))]
            grid: Array2D::filled_with(0, 103, 101),
        }
    }

    fn to_pt(&self) -> Point {
        Point {
            x: self.grid.num_columns() as isize,
            y: self.grid.num_rows() as isize,
        }
    }

    fn show_map(&self) {
        for row in self.grid.as_rows() {
            println!("{}", row.iter().map(|c| c.to_string()).collect::<String>());
        }
    }
    fn increment(&mut self, x: usize, y: usize) {
        let elem = self.grid.get(y, x).unwrap();
        self.grid.set(y, x, elem + 1).unwrap();
    }

    fn as_quadrants(&self) -> Vec<_> {
        let rows = self.grid.as_rows();
        let h_half = self.grid.num_rows() / 2;
        let w_half = self.grid.num_columns() / 2;
        todo!("Need implementation");
        vec![0; 5]
    }

    fn in_map(&self, pt: Point) -> bool {
        (pt.x as usize) < self.grid.num_columns() && (pt.y as usize) < self.grid.num_rows()
    }

    fn decrement(&mut self, x: usize, y: usize) {
        let elem = self.grid.get(y, x).unwrap();
        self.grid.set(y, x, elem - 1).unwrap();
    }

    fn add_robot(&mut self, r: Robot) {
        self.increment(r.position.x as usize, r.position.y as usize);
    }
    fn rem_robot(&mut self, r: Robot) {
        self.decrement(r.position.x as usize, r.position.y as usize);
    }
}

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
    let mut map = Map::new();
    let map_pt = map.to_pt();
    let robots = parse_input(input);

    let n_seconds: isize = 100;
    let post_pos = robots
        .iter()
        .map(|r| Robot::new(r.position + r.velocity * n_seconds, r.velocity))
        .map(|r| r.position % map_pt)
        .collect::<Vec<Point>>();

    for pos in post_pos {
        map.increment(pos.x as usize, pos.y as usize);
    }

    map.show_map();
    Some(2)
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
