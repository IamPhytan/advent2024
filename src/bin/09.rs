advent_of_code::solution!(9);

use std::collections::HashMap;

use indicatif::ProgressBar;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Block {
    val: isize,
    len: usize,
    start: usize,
}

impl Block {
    fn new(val: isize, len: usize, start: usize) -> Block {
        Block {
            val: val,
            len: len,
            start: start,
        }
    }

    fn to_vec(&self) -> Vec<isize> {
        vec![self.val; self.len]
    }

    fn is_full(&self) -> bool {
        self.val != -1
    }
}

fn parse_map(input: &str) -> Vec<Block> {
    let mut i: usize = 0;
    input
        .chars()
        .enumerate()
        .map(|(idx, n)| {
            let c = match idx % 2 {
                // Even
                0 => (idx / 2) as isize,
                // Odd
                _ => -1_isize,
            };
            let start = i.clone();
            let len = n.to_digit(10).unwrap() as usize;
            i += len;
            Block::new(c, len, start)
        })
        .collect()
}

fn is_compacted(disk_layout: &Vec<isize>) -> bool {
    let blockpos: Vec<usize> = disk_layout
        .par_iter()
        .enumerate()
        .rev()
        .filter(|(_, &c)| c >= 0)
        .map(|(i, _)| i)
        .collect();

    let compacted = blockpos[0] < blockpos.len();
    // println!("{} {:?}", compacted, blockpos);
    compacted
}

fn checksum(disk_layout: &Vec<isize>) -> usize {
    disk_layout
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| if b < 0 { None } else { Some(i * b as usize) })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let disk_blocks = parse_map(input);
    let mut disk_layout = disk_blocks
        .iter()
        .flat_map(|b| b.to_vec())
        .collect::<Vec<isize>>();

    // println!("{:?}", disk_layout);

    let bar = ProgressBar::new(disk_layout.len() as u64);

    while !is_compacted(&disk_layout) {
        let empty_idx = disk_layout
            .iter()
            .enumerate()
            .skip_while(|(_, &c)| c >= 0)
            .take(1)
            .map(|(i, _)| i)
            .next()
            .unwrap();

        let block_idx = disk_layout
            .iter()
            .enumerate()
            .rev()
            .skip_while(|(_, &c)| c < 0)
            .take(1)
            .map(|(i, _)| i)
            .next()
            .unwrap();

        disk_layout[empty_idx] = *disk_layout.get(block_idx).unwrap();
        disk_layout[block_idx] = -1;

        bar.set_position(empty_idx as u64);

        // println!("{} > {}", empty_idx, block_idx);
        // println!("{:?}", disk_layout);
    }

    Some(checksum(&disk_layout))
}

pub fn part_two(input: &str) -> Option<u32> {
    let disk_blocks = parse_map(input);
    let disk_len: usize = disk_blocks.iter().map(|b| b.len).sum();
    let bar = ProgressBar::new(disk_len as u64);

    let mut start_map: HashMap<usize, Block> = HashMap::new();
    for block in disk_blocks.clone() {
        start_map.insert(block.start, block.clone());
    }

    let mut empties = disk_blocks
        .iter()
        .filter(|&b| !b.is_full())
        .sorted_by_key(|&b| b.start)
        .collect::<Vec<&Block>>();

    let mut availables = disk_blocks
        .iter()
        .filter(|&b| b.is_full())
        .sorted_by_key(|&b| b.start)
        .rev()
        .map(|&b| b)
        .collect::<Vec<Block>>();

    let mut res: Vec<Block> = vec![];
    let mut i: usize = 0;

    while !empties.is_empty() {
        if start_map.contains_key(&i) && start_map[&i].is_full() {
            let first_block = start_map[&i];
            i += first_block.len;
            res.push(first_block);
            continue;
        }

        let next_empty = empties[0];
        println!("{} {} {:?}", empties.len(), i, next_empty);

        while i != next_empty.start {
            let (i_available, next_available) = availables
                .iter()
                .enumerate()
                .skip_while(|(_, &b)| b.len > next_empty.len)
                .take(1)
                .next()
                .unwrap();

            i += next_available.len;
            res.push(next_available.clone());
            availables.remove(i_available);
        }

        assert!(i == next_empty.start, "Missing full block");

        empties = empties[1..].to_vec();

        // let first_block = start_map
        //     .iter()
        //     .filter(|(_, &v)| v.is_full())
        //     .min_by_key(|(&k, _)| k)
        //     .map(|(k, v)| v)
        //     .unwrap();
    }

    return Some(5);

    let mut i: usize = 0;

    let next_empty = disk_blocks
        .iter()
        .filter(|&b| (b.val < 0))
        .min_by_key(|&b| b.start)
        .unwrap();
    let last_full = disk_blocks
        .iter()
        .filter(|&b| b.val >= 0)
        .max_by_key(|&b| b.start + b.len)
        .unwrap();
    println!("{:?} {:?}", start_map[&0], res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
