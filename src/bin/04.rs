use std::ops::RangeInclusive;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| line.split(&[',', '-'][..]).collect_vec())
            .map(|ranges| to_ranges(&ranges))
            .filter(|(range1, range2)| fully_contains(&range1, &range2))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    None
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

fn fully_contains(range1: &RangeInclusive<usize>, range2: &RangeInclusive<usize>) -> bool {
    let a = range1.clone().count();
    let b = range2.clone().count();
    let largest_range = if a > b { range1 } else { range2 };
    let smallest_range = if a <= b { range1 } else { range2 };

    largest_range.start() <= smallest_range.start() && largest_range.end() >= smallest_range.end()
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
        assert_eq!(part_two(&input), None);
    }
}
