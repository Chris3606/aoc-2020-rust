use std::str::FromStr;

use advent_of_code::helpers;

trait PasswordPolicy {
    fn meets(&self, password: &str) -> bool;
}

struct Part1Policy {
    occurences: helpers::NumRange,
    letter: char,
}

impl PasswordPolicy for Part1Policy {
    fn meets(&self, password: &str) -> bool {
        let count = password.chars().filter(|&s| s == self.letter).count() as u32;

        count >= self.occurences.min && count <= self.occurences.max
    }
}

impl FromStr for Part1Policy {
    type Err = helpers::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(' ');

        let occurences = it
            .next()
            .ok_or(helpers::ParseError::InvalidInput)?
            .parse::<helpers::NumRange>()
            .map_err(|_| helpers::ParseError::InvalidInput)?;

        let letter = it
            .next()
            .ok_or(helpers::ParseError::InvalidInput)?
            .chars()
            .next()
            .ok_or(helpers::ParseError::InvalidInput)?;

        Ok(Self { occurences, letter })
    }
}

struct Part2Policy {
    positions: (usize, usize),
    letter: char,
}

impl PasswordPolicy for Part2Policy {
    fn meets(&self, password: &str) -> bool {
        let chars = password.chars().collect::<Vec<_>>();

        let mut count = if chars[self.positions.0] == self.letter {
            1
        } else {
            0
        };
        count += if chars[self.positions.1] == self.letter {
            1
        } else {
            0
        };

        count == 1
    }
}

impl FromStr for Part2Policy {
    type Err = helpers::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(' ');

        let occurences = it
            .next()
            .ok_or(helpers::ParseError::InvalidInput)?
            .parse::<helpers::NumRange>()
            .map_err(|_| helpers::ParseError::InvalidInput)?;

        let letter = it
            .next()
            .ok_or(helpers::ParseError::InvalidInput)?
            .chars()
            .next()
            .ok_or(helpers::ParseError::InvalidInput)?;

        Ok(Self {
            positions: ((occurences.min - 1) as usize, (occurences.max - 1) as usize),
            letter,
        })
    }
}

struct PasswordDBEntry<T: PasswordPolicy + FromStr> {
    policy: T,
    password: String,
}

impl<T: PasswordPolicy + FromStr> FromStr for PasswordDBEntry<T> {
    type Err = helpers::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(": ");
        let policy = it
            .next()
            .ok_or(helpers::ParseError::InvalidInput)?
            .parse::<T>()
            .map_err(|_| helpers::ParseError::InvalidInput)?;

        let password = it
            .next()
            .ok_or(helpers::ParseError::InvalidInput)?
            .to_string();

        Ok(PasswordDBEntry { policy, password })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<PasswordDBEntry<Part1Policy>>().unwrap())
            .filter(|entry| entry.policy.meets(&entry.password))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<PasswordDBEntry<Part2Policy>>().unwrap())
            .filter(|entry| entry.policy.meets(&entry.password))
            .count(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(1));
    }
}
