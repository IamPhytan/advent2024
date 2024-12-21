// Inspired by https://github.com/martin-conur/rust-advent-of-code-2024/blob/main/src/bin/day10.rs
advent_of_code::solution!(10);

use std::collections::{HashSet, VecDeque};

use array2d::Array2D;

#[derive(Debug, Clone, Copy)]
struct Direction {
    dx: isize,
    dy: isize,
}

impl Direction {
    fn new(dx: isize, dy: isize) -> Direction {
        Direction { dx, dy }
    }
}

struct Map {
    grid: Array2D<u8>,
}

impl Map {
    fn width(&self) -> usize {
        self.grid.num_columns()
    }

    fn height(&self) -> usize {
        self.grid.num_rows()
    }

    fn get(&self, row: usize, col: usize) -> u8 {
        self.grid[(row, col)]
    }

    fn is_inside(&self, row: isize, col: isize) -> bool {
        if row < 0 || col < 0 {
            return false;
        }
        (row as usize) < self.height() && (col as usize) < self.width()
    }
}

fn bfs(
    map: &Map,
    start_row: usize,
    start_col: usize,
    directions: [Direction; 4],
    revisit: bool,
) -> u32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut score = 0;

    queue.push_back((start_row, start_col, 0));
    visited.insert((start_row, start_col));

    while let Some((current_row, current_col, current_height)) = queue.pop_front() {
        for direction in directions {
            let neighbor_row = current_row as isize + direction.dy;
            let neighbor_col = current_col as isize + direction.dx;

            if !map.is_inside(neighbor_row, neighbor_col) {
                continue;
            }

            let n_row = neighbor_row as usize;
            let n_col = neighbor_col as usize;

            if map.get(n_row, n_col) == current_height + 1
                && (revisit || !visited.contains(&(n_row, n_col)))
            {
                visited.insert((n_row, n_col));
                queue.push_front((n_row, n_col, current_height + 1));

                if map.get(n_row, n_col) == 9 {
                    score += 1;
                }
            }
        }
    }
    score
}

fn parse_input(input: &str) -> Map {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect();
    Map {
        grid: Array2D::from_rows(&map).unwrap(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);

    let mut total = 0;
    let directions = [
        Direction::new(0, 1),
        Direction::new(1, 0),
        Direction::new(0, -1),
        Direction::new(-1, 0),
    ];

    for row in 0..map.height() {
        for col in 0..map.width() {
            if map.get(row, col) == 0 {
                total += bfs(&map, row, col, directions, false);
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);

    let mut total = 0;
    let directions = [
        Direction::new(0, 1),
        Direction::new(1, 0),
        Direction::new(0, -1),
        Direction::new(-1, 0),
    ];

    for row in 0..map.height() {
        for col in 0..map.width() {
            if map.get(row, col) == 0 {
                total += bfs(&map, row, col, directions, true);
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
