use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    unique_sequence(input, &4)
}

pub fn part_two(input: &str) -> Option<usize> {
    unique_sequence(&input, &14)
}

fn unique_sequence(input: &str, length: &usize) -> Option<usize> {
    let mut start = 0;
    let mut seen_chars = HashSet::<char>::new();

    let input = input.as_bytes();

    for end in 0..input.len() {
        let current_char = input[end] as char;
        if !seen_chars.contains(&current_char) {
            seen_chars.insert(current_char);
        } else {
            while input[start] as char != current_char && start < end {
                seen_chars.remove(&(input[start] as char));
                start += 1;
            }
            start += 1;
        }

        if end - start == length - 1 {
            return Some(end + 1);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let inputs = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for input in inputs {
            assert_eq!(part_one(&input.0), Some(input.1));
        }
    }

    #[test]
    fn test_part_two() {
        let inputs = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for input in inputs {
            assert_eq!(part_two(&input.0), Some(input.1));
        }

        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
