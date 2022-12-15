use std::collections::HashSet;

use itertools::Itertools;

type Position = (isize, isize);

#[derive(Debug)]
struct Square {
    up: Position,
    down: Position,
    left: Position,
    right: Position,
    center: Position,
    distance: isize,
}

fn part_one_with_row(input: &str, row: &isize) -> Option<usize> {
    let squares = input.lines().map(|line| {
        let numbers = advent_of_code::helpers::GetNumbers::<isize>::number_by_separators(
            line,
            &['=', ' ', ',', ':'],
        );
        let sensor = (numbers[0], numbers[1]);
        let beacon = (numbers[2], numbers[3]);
        let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

        Square {
            up: (sensor.0, sensor.1 - distance),
            down: (sensor.0, sensor.1 + distance),
            left: (sensor.0 - distance, sensor.1),
            right: (sensor.0 + distance, sensor.1),
            center: sensor,
            distance,
        }
    });

    let filtered_squares = squares.filter(|square| square.up.1 <= *row && square.down.1 >= *row);

    let mut visited = HashSet::new();
    for square in filtered_squares {
        let x_delta = (square.distance - (square.center.1 - *row).abs());
        println!(
            "Self: {:?} | distance: {:?} | x delta: {:?}",
            square.center, square.distance, x_delta
        );
        for x in (square.center.0 - x_delta)..(square.center.0 + x_delta) {
            visited.insert(x);
        }
    }
    Some(visited.len())
}

pub fn part_one(input: &str) -> Option<usize> {
    // 6990756 - too high
    part_one_with_row(input, &2000000)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
