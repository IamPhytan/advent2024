advent_of_code::solution!(6);

use std::fmt;
use strum::FromRepr;

use array2d::Array2D;

#[derive(FromRepr, Debug, Clone)]
#[repr(i32)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn as_char(&self) -> char {
        match self {
            Direction::UP => '^',
            Direction::DOWN => 'v',
            Direction::LEFT => '<',
            Direction::RIGHT => '>',
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            direction => write!(f, "{}", direction.as_char()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Coordinates {
    row: usize,
    col: usize,
}

fn parse_input(input: &str, direction: &Direction) -> (Array2D<char>, Coordinates) {
    let lines: Vec<String> = input
        .lines()
        .into_iter()
        .map(|line| line.to_owned())
        .collect();

    let maze = Array2D::from_rows(
        &lines
            .iter()
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>(),
    )
    .unwrap();

    let start: (usize, usize) = *lines
        .iter()
        .enumerate()
        .filter_map(|(i, row)| match row.find(direction.as_char()) {
            Some(n) => Some((i, n)),
            None => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    let coords: Coordinates = Coordinates {
        row: start.0,
        col: start.1,
    };

    (maze, coords)
}

fn move_guard(
    maze: Array2D<char>,
    coords: Coordinates,
    guard: &Direction,
) -> (Array2D<char>, Coordinates) {
    todo!("Implement moving forward");
    (maze, coords)
}

fn turn_guard(mut guard: Direction, maze: &Array2D<char>, coords: &Coordinates) -> Direction {
    assert!(
        guard.as_char() == *maze.get(coords.row, coords.col).unwrap(),
        "Guard is not in the right direction, expected {}",
        guard.as_char()
    );

    guard = match guard {
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
    };

    guard
}

fn is_border(coords: Coordinates, height: &usize, width: &usize) -> bool {
    let col = coords.col == 0 || coords.col == (width - 1);
    let row = coords.row == 0 || coords.row == (height - 1);
    col && row
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut guard: Direction = Direction::UP;
    let (mut maze, mut coords) = parse_input(input, &guard);

    let W = maze.num_columns();
    let H = maze.num_rows();

    while !is_border(coords, &H, &W) {
        // Move
        (maze, coords) = move_guard(maze, coords, &guard);
        // Turn
        guard = turn_guard(guard, &maze, &coords);
        maze.set(coords.row, coords.col, guard.as_char()).unwrap();
    }

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
