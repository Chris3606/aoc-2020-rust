use std::{collections::HashMap, str::FromStr};

use advent_of_code::helpers;

type Passport = HashMap<String, String>;

// fn parse_passport(s: &str) -> Result<HashMap<String, String>, helpers::ParseError> {
//     let mut fields_it = s.split(&[' ', '\n', ':']);

//     let kv_it = fields_it.split
// }

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn verify_passport(passport: &Passport) -> bool {
    REQUIRED_FIELDS.iter().all(|&f| passport.contains_key(f))
}

fn verify_is_num(data: &str, min: u32, max: u32) -> bool {
    match data.parse::<u32>() {
        Ok(v) => v >= min && v <= max,
        Err(_) => false,
    }
}

fn verify_hgt(data: &str) -> bool {
    if data.len() < 3 {
        return false;
    }

    let (min, max) = match &data[data.len() - 2..] {
        "cm" => (150, 193),
        "in" => (59, 76),
        _ => return false,
    };

    verify_is_num(&data[0..data.len() - 2], min, max)
}

fn verify_hcl(data: &str) -> bool {
    if data.len() != 7 {
        return false;
    }

    if &data[0..1] != "#" {
        return false;
    }

    (&data[1..]).chars().all(|c| c.is_ascii_hexdigit())
}

fn verify_pid(data: &str) -> bool {
    if data.len() != 9 {
        return false;
    }
    data.chars().all(|c| c.is_ascii_digit())
}

fn verify_passport2(passport: &Passport) -> bool {
    if !REQUIRED_FIELDS.iter().all(|&f| passport.contains_key(f)) {
        return false;
    }

    for (k, v) in passport {
        let field_valid = match k.as_str() {
            "byr" => verify_is_num(v, 1920, 2002),
            "iyr" => verify_is_num(v, 2010, 2020),
            "eyr" => verify_is_num(v, 2020, 2030),
            "hgt" => verify_hgt(v),
            "hcl" => verify_hcl(v),
            "ecl" => match v.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            "pid" => verify_pid(v),
            _ => true,
        };
        if !field_valid {
            return false;
        }
    }

    true
}

fn parse_input(input: &str) -> Vec<Passport> {
    let passports_it = input.split("\n\n");

    let mut passports = Vec::new();

    for passport_data in passports_it {
        let field_it = passport_data.split_whitespace();

        let mut passport = Passport::new();
        for field_data in field_it {
            let mut it = field_data.split(':');
            let key = it.next().unwrap();
            let value = it.next().unwrap();

            passport.insert(key.to_string(), value.to_string());
        }

        passports.push(passport);
    }

    passports
}

pub fn part_one(input: &str) -> Option<usize> {
    let passports = parse_input(input);

    Some(passports.iter().filter(|&p| verify_passport(p)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let passports = parse_input(input);

    Some(passports.iter().filter(|&p| verify_passport2(p)).count())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(2));
    }
}
