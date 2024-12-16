// Partly inspired by https://github.com/jayo60013/aoc_2024/blob/main/day09/src/main.rs
advent_of_code::solution!(9);

// use indicatif::ProgressBar;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Block {
    id: Option<u32>,
    start: usize,
    end: usize,
}

impl Block {
    fn new(id: Option<u32>, start: usize, end: usize) -> Block {
        Block {
            id: id,
            start: start,
            end: end,
        }
    }

    fn as_vec(&self) -> Vec<Option<u32>> {
        vec![self.id; self.end + 1 - self.start]
    }

    fn is_file(&self) -> bool {
        self.id.is_some()
    }
}

fn checksum(disk_layout: &Vec<Option<u32>>) -> usize {
    disk_layout
        .iter()
        .enumerate()
        .filter(|(_, v)| v.is_some())
        .map(|(i, &id)| i * id.unwrap() as usize)
        .sum()
}

fn parse_map(input: &str) -> Vec<Block> {
    let mut ptr: usize = 0;
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(idx, c)| {
            let file_id = match idx % 2 {
                // Even
                0 => Some(idx as u32 / 2),
                // Odd
                _ => None,
            };
            let start = ptr.clone();
            let len = c.to_digit(10).unwrap() as usize;
            ptr += len;
            Block::new(file_id, start, start + len - 1)
        })
        .collect()
}

fn get_disk(disk_map: &Vec<Block>) -> Vec<Option<u32>> {
    disk_map.iter().map(|b| b.as_vec()).flatten().collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let disk_map = parse_map(input);
    let disk = get_disk(&disk_map);

    let mut compacted_disk = disk.clone();

    // let bar = ProgressBar::new(disk_map.len() as u64);

    while compacted_disk.contains(&None) {
        let last = compacted_disk.pop().unwrap();
        if last.is_none() {
            continue;
        }

        if let Some(first_none) = compacted_disk.iter_mut().find(|x| x.is_none()) {
            *first_none = last;
        }

        // Progress bar
        // bar.set_position(
        //     compacted_disk
        //         .iter()
        //         .filter_map(|&v| v)
        //         .collect::<Vec<u32>>()
        //         .len() as u64,
        // );
    }

    println!("{}", compacted_disk.len());

    Some(checksum(&compacted_disk))
}

pub fn part_two(input: &str) -> Option<usize> {
    let disk_map = parse_map(input);
    let disk = get_disk(&disk_map);

    let mut compacted_disk = disk.clone();

    // let bar = ProgressBar::new(
    //     disk_map
    //         .iter()
    //         .filter(|&b| b.is_file())
    //         .collect::<Vec<&Block>>()
    //         .len() as u64,
    // );

    for file in disk_map.iter().filter(|&b| b.is_file()).rev() {
        // bar.inc(1);

        let right_ptr = file.end;
        let winsize = file.as_vec().len();

        if let Some(start_idx) = compacted_disk
            .windows(winsize)
            .position(|win| win == vec![None; winsize])
        {
            if start_idx < right_ptr {
                for i in start_idx..(start_idx + winsize) {
                    compacted_disk[i] = file.id;
                }
                for i in file.start..=file.end {
                    compacted_disk[i] = None;
                }
            }
        }
    }

    Some(checksum(&compacted_disk))
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
        assert_eq!(result, Some(2858));
    }
}
