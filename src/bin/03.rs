use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|rucksack| {
            vec![
                rucksack.get(0..rucksack.len() / 2).unwrap(),
                rucksack.get(rucksack.len() / 2..rucksack.len()).unwrap(),
            ]
        })
        .map(|rucksack| find_duplicate_item(rucksack))
        .map(|duplicate_item| priority(&duplicate_item.unwrap()))
        .sum()
}

pub fn part_two(input: &str) -> Option<usize> {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| find_duplicate_item(group.collect_vec()))
        .map(|duplicate_item| priority(&duplicate_item.unwrap()))
        .sum()
}

fn find_duplicate_item(input: Vec<&str>) -> Option<char> {
    let mut duplicate_items = HashSet::<char>::new();

    for rucksack in input {
        let seen_items = rucksack
            .chars()
            .fold(HashSet::<char>::new(), |mut seen_items, item| {
                seen_items.insert(item);
                seen_items
            });
        duplicate_items = if duplicate_items.is_empty() {
            seen_items
        } else {
            duplicate_items
                .intersection(&seen_items)
                .map(|c| *c)
                .collect()
        }
    }
    duplicate_items.iter().next().copied()
}

fn priority(input: &char) -> Option<usize> {
    if *input >= 'A' && *input <= 'Z' {
        Some(*input as usize - 'A' as usize + 27)
    } else if *input >= 'a' && *input <= 'z' {
        Some(*input as usize - 'a' as usize + 1)
    } else {
        None
    }
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
