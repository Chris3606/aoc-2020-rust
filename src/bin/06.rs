use std::str::FromStr;

use advent_of_code::helpers;
use itertools::Itertools;

struct Group {
    yes_answers: Vec<char>,
    num_people: usize,
}

impl FromStr for Group {
    type Err = helpers::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut yes_answers = Vec::new();
        let mut num_people = 0;
        for line in s.lines() {
            yes_answers.extend(line.chars());
            num_people += 1;
        }

        Ok(Self {
            yes_answers,
            num_people,
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|gp_data| gp_data.parse::<Group>().unwrap())
            .map(|gp| gp.yes_answers.iter().unique().count())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|gp_data| gp_data.parse::<Group>().unwrap())
            .map(|gp| {
                gp.yes_answers
                    .iter()
                    .counts()
                    .iter()
                    .filter(|&(_, yes_num)| *yes_num == gp.num_people)
                    .count()
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(6));
    }
}
