advent_of_code::solution!(9);

use rayon::prelude::*;

struct Block {
    val: isize,
    len: usize,
}

impl Block {
    fn new(val: isize, len: usize) -> Block {
        Block { val: val, len: len }
    }

    fn to_vec(&self) -> Vec<isize> {
        vec![self.val; self.len]
    }
}

fn parse_map(input: &str) -> Vec<Block> {
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
            Block::new(c, n.to_digit(10).unwrap() as usize)
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
    let mut disk_layout = disk_blocks.iter().flat_map(|b| b.to_vec()).collect();

    println!("{:?}", disk_layout);

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

        println!("{} > {}", empty_idx, block_idx);
        // println!("{:?}", disk_layout);
    }

    Some(checksum(&disk_layout))
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
