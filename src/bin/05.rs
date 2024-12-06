advent_of_code::solution!(5);
use std::cmp::Ordering;

fn get_middle_page(pages: &Vec<i32>) -> u32 {
    return pages.get((pages.len() - 1) / 2).unwrap().to_owned() as u32;
}

fn is_valid_update(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    let applied_rules: Vec<_> = rules
        .iter()
        .filter(|&rule| rule.iter().all(|r| update.contains(&r)))
        .collect();

    for rule in applied_rules {
        let pos = update.iter().position(|item| item == &rule[0]).unwrap();
        if !update[pos..].contains(&rule[1]) {
            return false;
        }
    }
    true
}

fn rule_order(a: i32, b: i32, rules: &Vec<Vec<i32>>) -> Ordering {
    for rule in rules.clone() {
        let rule_tup = (rule[0], rule[1]);
        if (a, b) == rule_tup {
            return Ordering::Greater;
        } else if (b, a) == rule_tup {
            return Ordering::Less;
        } else {
            continue;
        }
    }
    return Ordering::Equal;
}

fn sort_update(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut sorted_upd = update.clone();
    sorted_upd.sort_by(|&a, &b| rule_order(a, b, rules));
    sorted_upd
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();

    let rules: Vec<Vec<i32>> = rules_str
        .lines()
        .map(|s| s.to_string())
        .into_iter()
        .map(|l| l.split("|").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    let updates: Vec<Vec<i32>> = updates_str
        .lines()
        .map(|s| s.to_string())
        .into_iter()
        .map(|l| l.split(",").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    let correct_updates: Vec<_> = updates
        .iter()
        .filter(|&u| is_valid_update(u, &rules))
        .collect();

    Some(correct_updates.iter().map(|u| get_middle_page(u)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();

    let rules: Vec<Vec<i32>> = rules_str
        .lines()
        .map(|s| s.to_string())
        .into_iter()
        .map(|l| l.split("|").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    let updates: Vec<Vec<i32>> = updates_str
        .lines()
        .map(|s| s.to_string())
        .into_iter()
        .map(|l| l.split(",").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    let sorted_updates: Vec<_> = updates
        .iter()
        .filter(|&u| !is_valid_update(u, &rules))
        .map(|u| sort_update(u, &rules))
        .collect();
    Some(sorted_updates.iter().map(|u| get_middle_page(u)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
