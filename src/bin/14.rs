use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Position = (usize, usize);

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = parse_map(input);
    let min_height = map.iter().max_by(|a, b| (a.1).cmp(&b.1)).unwrap().1;
    let initial_size = map.len();

    loop {
        let mut sand = (500, 0);
        let mut falling_into_void = false;
        loop {
            if sand.1 > min_height {
                falling_into_void = true;
                break;
            }
            if !map.contains(&(sand.0, sand.1 + 1)) {
                sand = (sand.0, sand.1 + 1);
            } else if !map.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !map.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                map.insert(sand);
                break;
            }
        }

        if falling_into_void {
            break;
        }
    }

    Some(map.len() - initial_size)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map = parse_map(input);
    let min_height = map.iter().max_by(|a, b| (a.1).cmp(&b.1)).unwrap().1;

    for x in 0..1500 {
        map.insert((x, min_height + 2));
    }
    let initial_size = map.len();
    loop {
        let mut sand = (500, 0);
        let mut blocked = true;
        loop {
            if !map.contains(&(sand.0, sand.1 + 1)) {
                sand = (sand.0, sand.1 + 1);
            } else if !map.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !map.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                map.insert(sand);
                break;
            }
            blocked = false;
        }

        if blocked {
            break;
        }
    }

    Some(map.len() - initial_size)
}

fn parse_map(input: &str) -> HashSet<Position> {
    let mut map: HashSet<Position> = HashSet::new();

    for path in input.lines() {
        let path = path
            .split(" -> ")
            .map(|c| {
                c.split(',')
                    .map(|f| f.parse::<usize>().unwrap())
                    .collect_tuple::<Position>()
                    .unwrap()
            })
            .tuple_windows();
        for (start, end) in path {
            let x_min = start.0.min(end.0);
            let x_max = start.0.max(end.0);
            let y_min = start.1.min(end.1);
            let y_max = start.1.max(end.1);

            for x in x_min..=x_max {
                for y in y_min..=y_max {
                    map.insert((x, y));
                }
            }
        }
    }
    map
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
