use std::collections::HashSet;

type Position = (isize, isize);
pub fn part_one(input: &str) -> Option<usize> {
    let mut visited = HashSet::<Position>::from([(0, 0)]);
    let mut head_position: Position = (0, 0);
    let mut tail_position: Position = (0, 0);

    for motion in input.lines() {
        let mut motion = motion.split_whitespace();
        let direction = motion.next();
        let times = motion.next()?.parse::<usize>().unwrap();

        for _ in 0..times {
            match direction {
                Some("U") => head_position.1 += 1,
                Some("D") => head_position.1 -= 1,
                Some("R") => head_position.0 += 1,
                Some("L") => head_position.0 -= 1,
                _ => println!("Nothing..."),
            }

            let delta = (
                (head_position.0 - tail_position.0),
                (head_position.1 - tail_position.1),
            );
            let not_touching = delta.0.abs() > 1 || delta.1.abs() > 1;

            if not_touching {
                tail_position.0 += 1 * delta.0.signum();
                tail_position.1 += 1 * delta.1.signum();
                visited.insert(tail_position);
            }
        }
    }
    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
