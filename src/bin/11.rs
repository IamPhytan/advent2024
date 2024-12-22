// Inspired by https://github.com/bozdoz/advent-of-code-2024/blob/main/day-11/src/main.rs
use std::{collections::HashMap, hash::Hash};

advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn blink(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut blinked: HashMap<usize, usize> = HashMap::new();

    for (stone, cnt) in stones {
        if stone == 0 {
            *blinked.entry(1).or_insert(0) += cnt;
        } else {
            let n_digits = stone.ilog10() as usize + 1;
            if n_digits % 2 == 0 {
                // Split number
                let div = n_digits / 2;
                let b = stone % 10_usize.pow(div as u32);
                let a = stone / 10_usize.pow(div as u32);
                // Insert both numbrs in blinked
                *blinked.entry(a).or_insert(0) += cnt;
                *blinked.entry(b).or_insert(0) += cnt;
            } else {
                *blinked.entry(stone * 2024).or_insert(0) += cnt;
            }
        }
    }
    return blinked;
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = parse_input(input);
    let mut stone_cnt: HashMap<usize, usize> = stones.iter().fold(HashMap::new(), |mut map, &s| {
        *map.entry(s.into()).or_insert(0) += 1;
        map
    });

    let n_steps = 25;
    for _ in (0..).take(n_steps) {
        // println!("{} {:?}", i, stone_cnt);
        stone_cnt = blink(stone_cnt);
    }
    Some(stone_cnt.values().sum::<usize>() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = parse_input(input);
    let mut stone_cnt: HashMap<usize, usize> = stones.iter().fold(HashMap::new(), |mut map, &s| {
        *map.entry(s.into()).or_insert(0) += 1;
        map
    });

    let n_steps = 75;
    for _ in (0..).take(n_steps) {
        // println!("{} {:?}", i, stone_cnt);
        stone_cnt = blink(stone_cnt);
    }
    Some(stone_cnt.values().sum::<usize>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
