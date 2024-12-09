use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::{Add, Mul, Sub},
};

use array2d::Array2D;
use itertools::Itertools;

advent_of_code::solution!(8);

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

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self {
        Self {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

struct Map {
    grid: Array2D<char>,
    n_rows: usize,
    n_cols: usize,
}

impl Map {
    fn new(grid: &Array2D<char>) -> Map {
        Map {
            grid: grid.clone(),
            n_cols: grid.num_columns(),
            n_rows: grid.num_rows(),
        }
    }

    fn show(&self, grid: &Array2D<char>) {
        for row in grid.as_rows() {
            println!("{}", row.iter().collect::<String>());
        }
        println!();
    }

    fn show_map(&self, map: Option<&Array2D<char>>) {
        match map {
            Some(other_map) => self.show(&other_map),
            None => self.show(&self.grid),
        }
    }

    fn decorate(
        &self,
        antennas: &HashMap<char, Vec<Point>>,
        antinodes: Option<&Vec<Point>>,
    ) -> (Array2D<char>, Option<Array2D<char>>) {
        let initial = self.grid.clone();

        let mut antenna_grid = initial.clone();
        for (freq, positions) in antennas.into_iter() {
            for pos in positions.iter() {
                antenna_grid
                    .set(pos.y as usize, pos.x as usize, *freq)
                    .unwrap();
            }
        }

        let antinodes_grid = match antinodes {
            None => None,
            Some(node_list) => {
                let mut antinode_grid = initial.clone();
                for pt in node_list {
                    antinode_grid
                        .set(pt.y as usize, pt.x as usize, '#')
                        .unwrap();
                }
                Some(antinode_grid)
            }
        };

        (antenna_grid, antinodes_grid)
    }
}

fn parse_input(input: &str) -> (Map, HashMap<char, Vec<Point>>) {
    let lines: Vec<String> = input
        .lines()
        .into_iter()
        .map(|line| line.to_owned())
        .collect();

    let mut map = Array2D::from_rows(
        &lines
            .iter()
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>(),
    )
    .unwrap();

    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    for y in 0_usize..map.num_rows() {
        for x in 0_usize..map.num_columns() {
            let c = *map.get(y, x).unwrap();
            if c == '.' {
                continue;
            }
            let pos = Point {
                x: x as isize,
                y: y as isize,
            };
            antennas.entry(c).or_insert(vec![]).push(pos);
            let _ = map.set(y, x, '.');
        }
    }

    (Map::new(&map), antennas)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, antennas) = parse_input(input);

    let (height, width) = (map.n_rows, map.n_cols);
    // let frequencies: Vec<char> = antennas.iter().map(|(&k, _)| k).collect();

    let all_antinodes: Vec<Point> = antennas
        .clone()
        .into_iter()
        .map(|(_, freq_ant)| {
            freq_ant
                .to_owned()
                .iter()
                .combinations(2)
                .into_iter()
                .map(|ant_pair| {
                    let [&a, &b]: [&Point; 2] = ant_pair.try_into().unwrap();
                    // let antpair_grid = map.decorate(&antennas, Some(&vec![a, b])).1.unwrap();
                    // map.show_map(Some(&antpair_grid));
                    let offset = b - a;
                    vec![a - offset, b + offset]
                })
                .flatten()
                .collect::<Vec<Point>>()
        })
        .flatten()
        .collect::<Vec<Point>>();

    let antinodes = all_antinodes
        .into_iter()
        .filter(|&pt| pt.is_in_map(height, width))
        .collect::<Vec<Point>>();

    let (antenna_map, opt_antinode_map) = map.decorate(&antennas, Some(&antinodes));
    let antinode_map = opt_antinode_map.unwrap();

    // map.show_map(None);
    map.show_map(Some(&antenna_map));
    map.show_map(Some(&antinode_map));

    // Remove duplicates
    let unique = antinodes
        .clone()
        .into_iter()
        .collect::<HashSet<Point>>()
        .into_iter()
        .collect::<Vec<Point>>();

    println!("{} {}", antinodes.len(), unique.len());

    Some(unique.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, antennas) = parse_input(input);

    let (height, width) = (map.n_rows, map.n_cols);
    // let frequencies: Vec<char> = antennas.iter().map(|(&k, _)| k).collect();

    let all_antinodes: Vec<Point> = antennas
        .clone()
        .into_iter()
        .map(|(_, freq_ant)| {
            freq_ant
                .to_owned()
                .iter()
                .combinations(2)
                .into_iter()
                .map(|ant_pair| {
                    let [&a, &b]: [&Point; 2] = ant_pair.try_into().unwrap();
                    // let antpair_grid = map.decorate(&antennas, Some(&vec![a, b])).1.unwrap();
                    // map.show_map(Some(&antpair_grid));
                    let offset = b - a;
                    let max_offset = [offset.x, offset.y]
                        .iter()
                        .map(|dim| dim.abs())
                        .max()
                        .unwrap();
                    let n = 2 * {
                        if max_offset == offset.x.abs() {
                            (width as isize) / max_offset
                        } else {
                            (height as isize) / max_offset
                        }
                    };
                    (0..n)
                        .into_iter()
                        .map(|i| {
                            let d = offset * i;
                            vec![a - d, b + d]
                        })
                        .flatten()
                        .collect::<Vec<Point>>()
                })
                .flatten()
                .collect::<Vec<Point>>()
        })
        .flatten()
        .collect::<Vec<Point>>();

    let antinodes = all_antinodes
        .into_iter()
        .filter(|&pt| pt.is_in_map(height, width))
        .collect::<Vec<Point>>();

    let (antenna_map, opt_antinode_map) = map.decorate(&antennas, Some(&antinodes));
    let antinode_map = opt_antinode_map.unwrap();

    // map.show_map(None);
    map.show_map(Some(&antenna_map));
    map.show_map(Some(&antinode_map));

    // Remove duplicates
    let unique = antinodes
        .clone()
        .into_iter()
        .collect::<HashSet<Point>>()
        .into_iter()
        .collect::<Vec<Point>>();

    println!("{} {}", antinodes.len(), unique.len());

    Some(unique.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
