// Inspired by https://gist.github.com/icub3d/ad5ffceed8a5170725ea389340b4f6b7
advent_of_code::solution!(6);

use fxhash::FxHashSet;
use rayon::prelude::*;

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
            println!("{}", row.iter().collect::<String>());
        }
        println!("===========");
    }

    fn get(&self, point: &Point) -> Option<&char> {
        self.grid.get(point.y as usize, point.x as usize)
    }
}

fn find_loop(maze: &Maze, start: &Point, obstacle: &Point) -> Option<()> {
    // If we put an obstacle on the next position, we should return to the same position with same heading, or go off the map
    let mut map = maze.clone();
    if map.get(&obstacle) == Some(&'.') {
        let _ = map.grid.set(obstacle.y as usize, obstacle.x as usize, '#');
    } else {
        return None;
    }

    // Run the loop
    let mut current_pos = *start;
    let mut current_head = Direction::UP;
    let mut seen = FxHashSet::default();
    loop {
        // Try to move. If not possible, turn right
        let next_pos = current_pos.next(&current_head);
        match map.get(&next_pos) {
            Some(&'#') => {
                current_head = current_head.turn_guard();
            }
            None => return None,
            _ => {
                current_pos = next_pos;
            }
        }
        if seen.contains(&(current_pos, current_head)) {
            return Some(());
        }
        seen.insert((current_pos, current_head));
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
    let mut heading: Direction = Direction::UP;
    let (grid, mut current_pos) = parse_input(input);
    let maze = Maze::new(&grid);

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

    println!(
        "{} / {}",
        directions.values().map(|v| v.len()).sum::<usize>(),
        maze.grid.num_elements()
    );

    // maze.show_maze();

    Some(seen.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut heading: Direction = Direction::UP;
    let (grid, mut current_pos) = parse_input(input);
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

    let now = std::time::Instant::now();
    let obstacles = directions
        .par_iter()
        .flat_map(|(point, directions)| {
            directions
                .iter()
                .filter_map(|&direction| {
                    find_loop(&maze, &original_pos, &point.next(&direction))
                        .map(|_| point.next(&direction))
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<Point>>();

    println!("p2: {} ({:?})", obstacles.len(), now.elapsed());

    Some(obstacles.len() as u32)
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
        assert_eq!(result, Some(6));
    }
}
