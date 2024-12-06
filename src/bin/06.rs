advent_of_code::solution!(6);

use array2d::Array2D;

fn turn(guard: char) -> char {
    match guard {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => todo!(),
    }
}

fn parse_input(input: &str, direction: char) -> (Array2D<char>, (usize, usize)) {
    let lines: Vec<String> = input
        .lines()
        .into_iter()
        .map(|line| line.to_owned())
        .collect();

    let coords: (usize, usize) = *lines
        .iter()
        .enumerate()
        .filter_map(|(i, row)| match row.find(direction) {
            Some(n) => Some((i, n)),
            None => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    let maze = Array2D::from_rows(
        &lines
            .iter()
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>(),
    )
    .unwrap();

    (maze, coords)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut direction: char = '^';
    let (maze, coords) = parse_input(input, direction);

    println!("{:?}", coords);

    direction = turn(direction);

    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
