advent_of_code::solution!(6);

use std::{fmt, ops};
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
struct Coordinate {
    row: usize,
    col: usize,
}

#[derive(Debug, Copy, Clone)]
struct ICoordinate {
    row: isize,
    col: isize,
}

impl ops::Add<ICoordinate> for Coordinate {
    type Output = ICoordinate;

    fn add(self, _rhs: ICoordinate) -> Self::Output {
        ICoordinate {
            row: self.row as isize + _rhs.row,
            col: self.col as isize + _rhs.col,
        }
    }
}

impl From<ICoordinate> for Coordinate {
    fn from(o: ICoordinate) -> Coordinate {
        Coordinate {
            row: o.row as usize,
            col: o.col as usize,
        }
    }
}

impl From<Direction> for ICoordinate {
    fn from(guard: Direction) -> ICoordinate {
        match guard {
            Direction::UP => ICoordinate { row: -1, col: 0 },
            Direction::DOWN => ICoordinate { row: 1, col: 0 },
            Direction::RIGHT => ICoordinate { row: 0, col: 1 },
            Direction::LEFT => ICoordinate { row: 0, col: -1 },
        }
    }
}

fn parse_input(input: &str, direction: &Direction) -> (Array2D<char>, Coordinate) {
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

    let coords: Coordinate = Coordinate {
        row: start.0,
        col: start.1,
    };

    (maze, coords)
}

fn is_next_out(coords: Coordinate, next_step: ICoordinate, maze: &Array2D<char>) -> bool {
    let next_col = coords.col as isize + next_step.col;
    let next_row = coords.row as isize + next_step.row;

    let out_tl = next_col < 0 || next_row < 0;
    let out_br =
        { next_col >= (maze.num_columns() as isize) || next_row >= (maze.num_rows() as isize) };

    out_tl || out_br
}

fn is_next_free(coords: Coordinate, next_step: ICoordinate, maze: &Array2D<char>) -> bool {
    if is_next_out(coords, next_step, maze) {
        // Out of the maze
        return false;
    } else {
        // Not an ostacle
        let next_coord = Coordinate::from(coords + next_step);
        return maze.get(next_coord.row, next_coord.col).unwrap() != &'#';
    }
}

fn move_guard(
    mut maze: Array2D<char>,
    mut coords: Coordinate,
    guard: &Direction,
) -> (Array2D<char>, Coordinate) {
    let next_step = ICoordinate::from(guard.clone());

    while is_next_free(coords, next_step, &maze) {
        maze.set(coords.row, coords.col, '*').unwrap();
        coords = Coordinate::from(coords + next_step);
        maze.set(coords.row, coords.col, guard.as_char()).unwrap();
    }

    (maze, coords)
}

fn turn_guard(guard: Direction, maze: &Array2D<char>, coords: &Coordinate) -> Direction {
    assert!(
        guard.as_char() == *maze.get(coords.row, coords.col).unwrap(),
        "Guard is not in the right direction, expected {}",
        guard.as_char()
    );

    match guard {
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
    }
}

fn show_maze(grid: &Array2D<char>) {
    for row in grid.as_rows() {
        println!("{:?}", row);
    }
    println!("===========");
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut guard: Direction = Direction::UP;
    let (mut maze, mut coords) = parse_input(input, &guard);

    while is_next_free(coords, ICoordinate::from(guard.clone()), &maze) {
        // Move
        (maze, coords) = move_guard(maze, coords, &guard);
        if is_next_out(coords, ICoordinate::from(guard.clone()), &maze) {
            break;
        }
        // Turn
        guard = turn_guard(guard, &maze, &coords);
        maze.set(coords.row, coords.col, guard.as_char()).unwrap();
    }

    Some(
        maze.as_rows()
            .iter()
            .map(|row| row.iter().filter(|&x| x == &'*').count() as u32)
            .sum::<u32>()
            + 1,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut guard: Direction = Direction::UP;
    let (mut maze, mut coords) = parse_input(input, &guard);

    let mut turns: i16 = 0;

    while is_next_free(coords, ICoordinate::from(guard.clone()), &maze) {
        // Move
        (maze, coords) = move_guard(maze, coords, &guard);
        if is_next_out(coords, ICoordinate::from(guard.clone()), &maze) {
            break;
        }
        // Turn
        guard = turn_guard(guard, &maze, &coords);
        turns += 1;

        maze.set(coords.row, coords.col, guard.as_char()).unwrap();
    }

    // Not enough time for this
    None
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
