use std::collections::HashMap;

use advent_of_code::helpers::GetNumbers;
use itertools::Itertools;

type Position = (isize, isize, isize);

trait AddXYZ {
    fn add_x(&self, x: &isize) -> Position;
    fn add_y(&self, y: &isize) -> Position;
    fn add_z(&self, z: &isize) -> Position;
}

impl AddXYZ for Position {
    fn add_x(&self, x: &isize) -> Position {
        (self.0 + x, self.1, self.2)
    }

    fn add_y(&self, y: &isize) -> Position {
        (self.0, self.1 + y, self.2)
    }

    fn add_z(&self, z: &isize) -> Position {
        (self.0, self.1, self.2 + z)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let coordinates: Vec<Position> = input
        .lines()
        .filter_map(|line| {
            line.number_by_separators(&[','])
                .into_iter()
                .collect_tuple()
        })
        .collect_vec();

    let mut sides = HashMap::new();
    for coordinate in coordinates {
        // Front
        sides
            .entry((
                coordinate,
                coordinate.add_x(&1),
                coordinate.add_z(&1),
                coordinate.add_x(&1).add_z(&1),
            ))
            .and_modify(|f| *f += 1)
            .or_insert(1);
        // Back
        sides
            .entry((
                coordinate.add_y(&1),
                coordinate.add_y(&1).add_x(&1),
                coordinate.add_y(&1).add_z(&1),
                coordinate.add_y(&1).add_x(&1).add_z(&1),
            ))
            .and_modify(|f| *f += 1)
            .or_insert(1);
        // Bottom
        sides
            .entry((
                coordinate,
                coordinate.add_x(&1),
                coordinate.add_y(&1),
                coordinate.add_x(&1).add_y(&1),
            ))
            .and_modify(|f| *f += 1)
            .or_insert(1);
        // Top
        sides
            .entry((
                coordinate.add_z(&1),
                coordinate.add_z(&1).add_x(&1),
                coordinate.add_z(&1).add_y(&1),
                coordinate.add_z(&1).add_x(&1).add_y(&1),
            ))
            .and_modify(|f| *f += 1)
            .or_insert(1);
        // Left
        sides
            .entry((
                coordinate,
                coordinate.add_z(&1),
                coordinate.add_y(&1),
                coordinate.add_z(&1).add_y(&1),
            ))
            .and_modify(|f| *f += 1)
            .or_insert(1);
        // Right
        sides
            .entry((
                coordinate.add_x(&1),
                coordinate.add_x(&1).add_z(&1),
                coordinate.add_x(&1).add_y(&1),
                coordinate.add_x(&1).add_z(&1).add_y(&1),
            ))
            .and_modify(|f| *f += 1)
            .or_insert(1);
    }

    Some(sides.into_iter().filter(|(_, v)| v == &1).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
