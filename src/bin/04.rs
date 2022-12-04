use std::ops::RangeInclusive;

use advent_of_code::helpers::{FullyContains, Overlaps};
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| line.split(&[',', '-'][..]).collect_vec())
            .map(|ranges| to_ranges(&ranges))
            .filter(|(range1, range2)| range1.fully_contains(&range2))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| line.split(&[',', '-'][..]).collect_vec())
            .map(|ranges| to_ranges(&ranges))
            .filter(|(range1, range2)| range1.overlaps(&range2))
            .count(),
    )
}

fn to_ranges(ranges: &Vec<&str>) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let ranges: Vec<usize> = ranges
        .iter()
        .map(|range| range.parse().unwrap())
        .collect_vec();

    (
        RangeInclusive::new(ranges[0], ranges[1]),
        RangeInclusive::new(ranges[2], ranges[3]),
    )
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
        assert_eq!(part_two(&input), Some(4));
    }
}
