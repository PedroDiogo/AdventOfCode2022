use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    // let input = String::from(input);
    input
        .lines()
        .map(|rucksack| find_duplicate_item(rucksack))
        .map(|duplicate_item| priority(&duplicate_item.unwrap()))
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn find_duplicate_item(input: &str) -> Option<char> {
    let halves = (
        input.get(0..input.len() / 2).unwrap(),
        input.get(input.len() / 2..input.len()).unwrap(),
    );

    let seen_items = halves
        .0
        .chars()
        .fold(HashSet::<char>::new(), |mut seen_items, item| {
            seen_items.insert(item);
            seen_items
        });

    halves.1.chars().find(|item| seen_items.contains(item))
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
        assert_eq!(part_two(&input), None);
    }
}
