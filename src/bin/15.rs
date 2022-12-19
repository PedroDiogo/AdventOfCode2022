use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use advent_of_code::helpers::{Overlaps, Union};
use itertools::Itertools;

type Position = (isize, isize);

#[derive(Debug)]
struct Square {
    up: Position,
    down: Position,
    center: Position,
    distance: isize,
}

fn part_one_with_row(input: &str, row: &isize) -> Option<usize> {
    let mut idx = 0;
    let squares = input.lines().map(|line| {
        let numbers = advent_of_code::helpers::GetNumbers::<isize>::number_by_separators(
            line,
            &['=', ' ', ',', ':'],
        );
        let sensor = (numbers[0], numbers[1]);
        let beacon = (numbers[2], numbers[3]);
        let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

        idx += 1;
        Square {
            up: (sensor.0, sensor.1 - distance),
            down: (sensor.0, sensor.1 + distance),
            center: sensor,
            distance,
        }
    });

    let filtered_squares = squares.filter(|square| square.up.1 <= *row && square.down.1 >= *row);

    let mut visited = HashSet::new();
    for square in filtered_squares {
        let x_delta = square.distance - (square.center.1 - *row).abs();
        for x in (square.center.0 - x_delta)..(square.center.0 + x_delta) {
            visited.insert(x);
        }
    }
    Some(visited.len())
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_with_row(input, &2000000)
}

fn part_two_with_limit(input: &str, limit: &isize) -> Option<usize> {
    let mut idx = 0;
    let squares = input
        .lines()
        .map(|line| {
            let numbers = advent_of_code::helpers::GetNumbers::<isize>::number_by_separators(
                line,
                &['=', ' ', ',', ':'],
            );
            let sensor = (numbers[0], numbers[1]);
            let beacon = (numbers[2], numbers[3]);
            let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

            idx += 1;
            Square {
                up: (sensor.0, sensor.1 - distance),
                down: (sensor.0, sensor.1 + distance),
                center: sensor,
                distance,
            }
        })
        .collect_vec();

    let mut a = HashMap::<isize, Vec<RangeInclusive<isize>>>::new();
    for square in squares {
        let mut i = 0;
        for row in square.up.1..=square.down.1 {
            a.entry(row)
                .and_modify(|r| {
                    let range = square.up.0 - i..=square.up.0 + i;
                    if range.overlaps(&(0..=*limit)) {
                        r.push((square.up.0 - i).max(0)..=(square.up.0 + i).min(*limit));
                    }
                })
                .or_insert_with(|| vec![square.up.0 - i..=square.up.0 + i]);
            if row < square.center.1 {
                i += 1;
            } else {
                i -= 1;
            }
        }
    }

    let a: HashMap<&isize, Vec<RangeInclusive<isize>>> = a
        .iter()
        .map(|r| {
            let a =
                r.1.iter()
                    .cloned()
                    .sorted_by_key(|ra| *ra.start())
                    .collect_vec();
            (r.0, a)
        })
        .collect();

    for (row, ranges) in a.into_iter() {
        if ranges.is_empty() {
            continue;
        }
        let mut a = vec![ranges.first().unwrap().clone()];
        for range in ranges.into_iter().skip(1) {
            let b = a.pop().unwrap();
            if let Some(u) = b.union(&range) {
                a.push(u);
            } else {
                a.push(b);
                a.push(range);
            }
        }

        for window in a.windows(2) {
            if (window[1].start() - window[0].end()) > 1 {
                let col = window[0].end() + 1;
                if row > &0 && row <= limit && col > 0 && col <= *limit {
                    return Some((col * 4000000 + row) as usize);
                }
            }
        }
    }
    None
}
pub fn part_two(input: &str) -> Option<usize> {
    part_two_with_limit(input, &4000000)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one_with_row(&input, &10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two_with_limit(&input, &20), Some(56000011));
    }
}
