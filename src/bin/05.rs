use std::collections::HashSet;

use advent_of_code::helpers;
use itertools::Itertools;

#[derive(Debug)]
enum PartitionSequence {
    Upper,
    Lower,
}

fn partition(min: i64, max: i64, seq: impl Iterator<Item = PartitionSequence>) -> i64 {
    let mut low = min;
    let mut high = max;

    for s in seq {
        match s {
            PartitionSequence::Lower => high = (high + low) / 2,
            PartitionSequence::Upper => low = (high + low) / 2,
        }
    }

    high
}

pub fn parse_seat(s: &str) -> helpers::Vector2i {
    if s.len() != 10 {
        panic!("Invalid input");
    }

    let row_data = &s[0..7];
    let col_data = &s[7..];

    let row_part = row_data.chars().filter_map(|i| match i {
        'F' => Some(PartitionSequence::Lower),
        'B' => Some(PartitionSequence::Upper),
        _ => None,
    });
    let row = partition(0, 127, row_part);

    let col_part = col_data.chars().filter_map(|i| match i {
        'R' => Some(PartitionSequence::Upper),
        'L' => Some(PartitionSequence::Lower),
        _ => None,
    });
    let col = partition(0, 7, col_part);

    helpers::Vector2i::new(col, row)
}

pub fn part_one(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|b| parse_seat(b))
        .map(|pos| pos.to_index(8))
        .max()
}

pub fn part_two(input: &str) -> Option<usize> {
    let seats = input.lines().map(|b| parse_seat(b)).collect::<HashSet<_>>();

    let seat = (0..=127)
        .cartesian_product(1..=6)
        .map(|(r, c)| helpers::Vector2i::new(c, r))
        .filter(|pos| !seats.contains(pos))
        .find(|pos| {
            seats.contains(&helpers::Vector2i::new(pos.x - 1, pos.y))
                && seats.contains(&helpers::Vector2i::new(pos.x + 1, pos.y))
        });

    match seat {
        None => None,
        Some(s) => Some(s.to_index(8)),
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(820));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
