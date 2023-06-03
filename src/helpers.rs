/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */
use std::str::FromStr;

#[derive(Debug)]
pub enum ParseError {
    InvalidInput,
}

/// Represents a number range with a min and a max value.
pub struct NumRange {
    pub min: u32,
    pub max: u32,
}

impl FromStr for NumRange {
    type Err = ParseError;

    /// Parses a number range in the format {min}-{max}
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num_it = s.split('-');
        let min = num_it
            .next()
            .ok_or(ParseError::InvalidInput)?
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidInput)?;
        let max = num_it
            .next()
            .ok_or(ParseError::InvalidInput)?
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidInput)?;

        Ok(NumRange { min, max })
    }
}
