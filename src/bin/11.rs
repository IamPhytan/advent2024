use std::collections::HashMap;

advent_of_code::solution!(11);

fn blink(stones: HashMap<u128, u128>) -> HashMap<u128, u128> {
    let mut blinked: HashMap<u128, u128> = HashMap::new();
    for (stone, cnt) in stones {
        if stone == 0 {
            *blinked.entry(1).or_insert(0) += 1;
        } else {
            let n_digits = stone.checked_ilog10().unwrap_or(0) + 1;
            if n_digits % 2 == 0 {
                // Split number
                let div = n_digits / 2;
                let b = stone % 10_u128.pow(div);
                let a = stone / 10_u128.pow(div);
                // Insert both numbrs in blinked
                *blinked.entry(a).or_insert(0) += 1;
                *blinked.entry(b).or_insert(0) += 1;
            } else {
                *blinked.entry(stone * 2024).or_insert(0) += 1;
            }
        }
    }
    return blinked;
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = parse_input(input);
    let mut stone_cnt: HashMap<u128, u128> = stones.iter().fold(HashMap::new(), |mut map, &s| {
        *map.entry(s.into()).or_insert(0) += 1;
        map
    });
    // let mut stone_cnt: HashMap<u64, u128> = HashMap::new();
    // for stone in stones {
    //     match stone_cnt.get(&stone) {
    //         Some(count) => {
    //             stone_cnt.insert(stone, count + 1);
    //         }
    //         None => {
    //             stone_cnt.insert(stone, 1);
    //         }
    //     }
    //     println!("{}", stone);
    // }

    let n_steps = 25;
    for i in 0..(n_steps + 1) {
        stone_cnt = blink(stone_cnt);
        println!("{} {:?}", i, stone_cnt.values().sum::<u128>());
    }
    Some(stone_cnt.values().sum::<u128>() as u64)
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
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
