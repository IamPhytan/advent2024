advent_of_code::solution!(5);

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

    let total = updates
        .iter()
        .filter_map(|update| {
            let n_elem = update.len();
            if !rules.iter().all(|r| {
                let (a, b) = (r[0], r[1]);
                let pos_a = update.iter().position(|&x| x == a);
                let pos_b = update.iter().position(|&x| x == b);
                matches!((pos_a, pos_b), (Some(i), Some(j)) if i<j)
            }) {
                let mut graph = vec![Vec::new(); n_elem];
                for rule in &rules {
                    if let (Some(i), Some(j)) = (
                        update.iter().position(|&x| x == rule[0]),
                        update.iter().position(|&x| x == rule[1]),
                    ) {
                        graph[i].push(j);
                    }
                }

                let mut seen = vec![false; n_elem];
                let mut order = Vec::with_capacity(n_elem);

                fn visit(
                    n: usize,
                    seen: &mut [bool],
                    order: &mut Vec<usize>,
                    graph: &[Vec<usize>],
                ) {
                    if seen[n] {
                        return;
                    }
                    seen[n] = true;
                    for &next in &graph[n] {
                        visit(next, seen, order, graph);
                    }
                    order.push(n);
                }

                for i in 0..n_elem {
                    if !seen[i] {
                        visit(i, &mut seen, &mut order, &graph);
                    }
                }

                order.reverse();
                Some(update[order[order.len() / 2]] as u32)
            } else {
                None
            }
        })
        .sum();

    Some(total)
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
