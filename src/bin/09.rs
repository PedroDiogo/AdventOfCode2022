use std::collections::{HashSet, VecDeque};

type Position = (isize, isize);
pub fn part_one(input: &str) -> Option<usize> {
    get_tail_visited_positions(input, &2)
}

pub fn part_two(input: &str) -> Option<usize> {
    get_tail_visited_positions(input, &10)
}

fn get_tail_visited_positions(input: &str, rope_length: &usize) -> Option<usize> {
    let mut visited = HashSet::<Position>::from([(0, 0)]);
    let mut rope = VecDeque::<Position>::new();

    for _ in 0..*rope_length {
        rope.push_front((0, 0))
    }

    for motion in input.lines() {
        let mut motion = motion.split_whitespace();
        let direction = motion.next();
        let times = motion.next()?.parse::<usize>().unwrap();

        for _ in 0..times {
            let mut head_position = rope.pop_front().unwrap();
            match direction {
                Some("U") => head_position.1 += 1,
                Some("D") => head_position.1 -= 1,
                Some("R") => head_position.0 += 1,
                Some("L") => head_position.0 -= 1,
                _ => unreachable!("Invalid direction"),
            }
            rope.push_back(head_position);

            for _ in 0..*rope_length - 1 {
                let mut tail_position = rope.pop_front().unwrap();
                let head_position = rope.back().unwrap();

                let delta = (
                    (head_position.0 - tail_position.0),
                    (head_position.1 - tail_position.1),
                );
                let not_touching = delta.0.abs() > 1 || delta.1.abs() > 1;

                if not_touching {
                    tail_position.0 += 1 * delta.0.signum();
                    tail_position.1 += 1 * delta.1.signum();
                }
                rope.push_back(tail_position);
            }
            visited.insert(*rope.back().unwrap());
        }
    }
    Some(visited.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }

    #[test]
    fn test_part_two_example_two() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part_two(&input), Some(36));
    }
}
