advent_of_code::solution!(4);

fn count_occurences(grid: &Vec<String>) -> u32 {
    grid.iter()
        .map(|s| {
            let lr = s.rmatches("XMAS").collect::<Vec<&str>>().len();
            let rl = s.rmatches("SAMX").collect::<Vec<&str>>().len();
            lr + rl
        } as u32)
        .sum()
}

fn to_string_grid(grid: &Vec<Vec<char>>) -> Vec<String> {
    grid.iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect()
}

fn to_diagonal_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = grid.len();
    let mut diag: Vec<Vec<char>> = vec![];
    for ridx in 0..(2 * n) {
        let mut row: Vec<char> = vec![];
        for x in 0..n {
            if x > ridx {
                row.push('.');
                continue;
            }

            let y = ridx - x;
            if (x >= n) || (y >= n) {
                continue;
            } else {
                row.push(grid[x][ridx - x]);
            }
        }
        diag.push(row);
    }
    diag
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|r| r.to_string().chars().collect())
        .collect();

    let horiz_sgrid = to_string_grid(&grid);
    let horiz: u32 = count_occurences(&horiz_sgrid);

    let vertic_grid = advent_of_code::vectranspose(grid.clone());
    let vertic_sgrid: Vec<String> = to_string_grid(&vertic_grid);
    let vertic: u32 = count_occurences(&vertic_sgrid);

    let diag_tl_sgrid = to_string_grid(&to_diagonal_grid(&grid));
    let diag_tl = count_occurences(&diag_tl_sgrid);
    let diag_bl_sgrid = to_string_grid(&to_diagonal_grid(
        &vertic_grid
            .iter()
            .map(|r| r.clone().into_iter().rev().collect::<Vec<char>>())
            .collect(),
    ));
    let diag_bl = count_occurences(&diag_bl_sgrid);

    println!("{} {}", grid.len(), grid[0].len());
    Some(horiz + vertic + diag_tl + diag_bl)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let target = ('M' as u32) + ('S' as u32);

    let mut cnt: u32 = 0;

    let n = grid.len();

    for r in 1..(n - 1) {
        for c in 1..(n - 1) {
            if grid[r][c] != 'A' {
                continue;
            }

            let tlbr: u32 = vec![grid[r - 1][c - 1], grid[r + 1][c + 1]]
                .iter()
                .map(|&n| n as u32)
                .sum();
            let trbl: u32 = vec![grid[r - 1][c + 1], grid[r + 1][c - 1]]
                .iter()
                .map(|&n| n as u32)
                .sum();

            if tlbr == target && trbl == target {
                cnt += 1;
            }
        }
    }

    Some(cnt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
