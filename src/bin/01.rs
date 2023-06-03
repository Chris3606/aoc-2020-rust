pub fn part_one(input: &str) -> Option<u32> {
    let expenses = input
        .lines()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    for i in 0..expenses.len() {
        for j in i..expenses.len() {
            if expenses[i] + expenses[j] == 2020 {
                return Some(expenses[i] * expenses[j]);
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let expenses = input
        .lines()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    for i in 0..expenses.len() {
        for j in 0..expenses.len() {
            for k in 0..expenses.len() {
                if expenses[i] + expenses[j] + expenses[k] == 2020 {
                    return Some(expenses[i] * expenses[j] * expenses[k]);
                }
            }
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(514579));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(241861950));
    }
}
