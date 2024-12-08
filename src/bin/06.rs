// Inspired by https://gist.github.com/icub3d/ad5ffceed8a5170725ea389340b4f6b7
advent_of_code::solution!(6);

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use array2d::Array2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn turn_guard(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
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

    fn next(&self, direction: &Direction) -> Point {
        match direction {
            Direction::UP => Point::new(self.x, self.y - 1),
            Direction::DOWN => Point::new(self.x, self.y + 1),
            Direction::LEFT => Point::new(self.x - 1, self.y),
            Direction::RIGHT => Point::new(self.x + 1, self.y),
        }
    }

    fn is_out(&self, maze: &Array2D<char>) -> bool {
        (self.x < 0 || self.x >= (maze.num_columns() as isize))
            || (self.y < 0 || self.y >= (maze.num_rows() as isize))
    }
}

#[derive(Debug, Clone)]
struct Maze {
    grid: Array2D<char>,
}

impl Maze {
    fn new(grid: &Array2D<char>) -> Maze {
        Maze { grid: grid.clone() }
    }

    fn show_maze(&self) {
        for row in self.grid.as_rows() {
            println!("{:?}", row);
        }
        println!("===========");
    }

    fn get(&self, point: &Point) -> Option<&char> {
        self.grid.get(point.y as usize, point.x as usize)
    }
}

fn parse_input(input: &str) -> (Array2D<char>, Point) {
    let lines: Vec<String> = input
        .lines()
        .into_iter()
        .map(|line| line.to_owned())
        .collect();

    let mut maze = Array2D::from_rows(
        &lines
            .iter()
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>(),
    )
    .unwrap();

    let coords = *lines
        .iter()
        .enumerate()
        .filter_map(|(i, row)| match row.find('^') {
            Some(n) => Some(Point {
                x: n as isize,
                y: i as isize,
            }),
            None => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    let _ = maze.set(coords.y as usize, coords.x as usize, '.');

    (maze, coords)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, mut current_pos) = parse_input(input);
    let mut heading: Direction = Direction::UP;

    let maze = Maze::new(&grid);
    let original_pos = current_pos;

    let mut directions: HashMap<Point, Vec<Direction>> = HashMap::new();

    let mut seen = HashSet::new();
    seen.insert(current_pos);
    loop {
        directions.entry(current_pos).or_default().push(heading);

        // Move if we can
        let next_pos = current_pos.next(&heading);
        match maze.get(&next_pos) {
            Some(&'#') => {
                heading = heading.turn_guard();
            }
            None => break,
            _ => {
                current_pos = next_pos;
                seen.insert(current_pos);
            }
        }
    }

    Some(seen.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut guard: Direction = Direction::UP;
    let (mut maze, mut coords) = parse_input(input);

    // Not enough time for this
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
