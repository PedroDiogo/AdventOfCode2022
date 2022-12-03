use advent_of_code::helpers::First;
use itertools::Itertools;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|rucksack| {
            vec![
                rucksack.get(0..rucksack.len() / 2).unwrap(),
                rucksack.get(rucksack.len() / 2..rucksack.len()).unwrap(),
            ]
        })
        .filter_map(|rucksack| find_duplicate_items(rucksack))
        .map(|duplicate_item| priority(duplicate_item.first()))
        .sum()
}

pub fn part_two(input: &str) -> Option<usize> {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| group.collect_vec())
        .filter_map(|group| find_duplicate_items(group))
        .map(|duplicate_item| priority(duplicate_item.first()))
        .sum()
}

fn find_duplicate_items(input: Vec<&str>) -> Option<HashSet<char>> {
    input
        .into_iter()
        .map(|rucksack| -> HashSet<char> { HashSet::from_iter(rucksack.chars()) })
        .reduce(|duplicates, seen_types| duplicates.intersection(&seen_types).map(|c| *c).collect())
}

fn priority(input: Option<&char>) -> Option<usize> {
    if let Some(input) = input {
        if *input >= 'A' && *input <= 'Z' {
            return Some(*input as usize - 'A' as usize + 27);
        } else if *input >= 'a' && *input <= 'z' {
            return Some(*input as usize - 'a' as usize + 1);
        }
    }
    None
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
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
