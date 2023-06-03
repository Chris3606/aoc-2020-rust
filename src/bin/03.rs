use advent_of_code::helpers;

fn trees_on_slope(grid: &helpers::Grid<bool>, slope: helpers::Vector2i) -> u32 {
    let mut cur_pos = helpers::Vector2i::new(0, 0);

    let mut trees = 0;
    while (cur_pos.y as usize) < grid.height() {
        if grid[cur_pos] == false {
            trees += 1;
        }

        cur_pos.x = (cur_pos.x + slope.x) % grid.width() as i64;
        cur_pos.y += slope.y;
    }

    trees
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = helpers::grid_from_floor_wall_grid(input);
    let trees = trees_on_slope(&grid, helpers::Vector2i::new(3, 1));

    Some(trees)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = helpers::grid_from_floor_wall_grid(input);

    let mut product = 1;
    product *= trees_on_slope(&grid, helpers::Vector2i::new(1, 1));
    product *= trees_on_slope(&grid, helpers::Vector2i::new(3, 1));
    product *= trees_on_slope(&grid, helpers::Vector2i::new(5, 1));
    product *= trees_on_slope(&grid, helpers::Vector2i::new(7, 1));
    product *= trees_on_slope(&grid, helpers::Vector2i::new(1, 2));

    Some(product)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(336));
    }
}
