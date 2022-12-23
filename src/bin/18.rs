use std::collections::{HashMap, HashSet};

use advent_of_code::helpers::GetNumbers;
use itertools::Itertools;

type Position = (isize, isize, isize);

trait AddXYZ {
    fn add_x(&self, x: &isize) -> Position;
    fn add_y(&self, y: &isize) -> Position;
    fn add_z(&self, z: &isize) -> Position;
}

impl AddXYZ for Position {
    fn add_x(&self, x: &isize) -> Position {
        (self.0 + x, self.1, self.2)
    }

    fn add_y(&self, y: &isize) -> Position {
        (self.0, self.1 + y, self.2)
    }

    fn add_z(&self, z: &isize) -> Position {
        (self.0, self.1, self.2 + z)
    }
}

trait Neighbours {
    fn neighbours(&self) -> Vec<Position>;
}

impl Neighbours for Position {
    fn neighbours(&self) -> Vec<Self> {
        let deltas = vec![
            (1, 0, 0),
            (0, 1, 0),
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ];
        deltas
            .into_iter()
            .map(|delta| (delta.0 + self.0, delta.1 + self.1, delta.2 + self.2))
            .collect_vec()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let coordinates: Vec<Position> = input
        .lines()
        .filter_map(|line| {
            line.number_by_separators(&[','])
                .into_iter()
                .collect_tuple()
        })
        .collect_vec();

    let mut sides = HashMap::new();
    for coordinate in coordinates {
        // Front
        sides
            .entry((
                coordinate,
                coordinate.add_x(&1),
                coordinate.add_z(&1),
                coordinate.add_x(&1).add_z(&1),
            ))
            .and_modify(|f| *f += 1)
            .or_insert(1);
        // Back
        sides
            .entry((
                coordinate.add_y(&1),
                coordinate.add_y(&1).add_x(&1),
                coordinate.add_y(&1).add_z(&1),
                coordinate.add_y(&1).add_x(&1).add_z(&1),
            ))
            .and_modify(|f| *f += 1)
            .or_insert(1);
        // Bottom
        sides
            .entry((
                coordinate,
                coordinate.add_x(&1),
                coordinate.add_y(&1),
                coordinate.add_x(&1).add_y(&1),
            ))
            .and_modify(|f| *f += 1)
            .or_insert(1);
        // Top
        sides
            .entry((
                coordinate.add_z(&1),
                coordinate.add_z(&1).add_x(&1),
                coordinate.add_z(&1).add_y(&1),
                coordinate.add_z(&1).add_x(&1).add_y(&1),
            ))
            .and_modify(|f| *f += 1)
            .or_insert(1);
        // Left
        sides
            .entry((
                coordinate,
                coordinate.add_z(&1),
                coordinate.add_y(&1),
                coordinate.add_z(&1).add_y(&1),
            ))
            .and_modify(|f| *f += 1)
            .or_insert(1);
        // Right
        sides
            .entry((
                coordinate.add_x(&1),
                coordinate.add_x(&1).add_z(&1),
                coordinate.add_x(&1).add_y(&1),
                coordinate.add_x(&1).add_z(&1).add_y(&1),
            ))
            .and_modify(|f| *f += 1)
            .or_insert(1);
    }

    // println!("{:?}", sides);
    Some(sides.into_iter().filter(|(_, v)| v == &1).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let coordinates: Vec<Position> = input
        .lines()
        .filter_map(|line| {
            line.number_by_separators(&[','])
                .into_iter()
                .collect_tuple()
        })
        .collect_vec();
    let (min_x, max_x) = match coordinates.iter().minmax_by_key(|c| c.0) {
        itertools::MinMaxResult::MinMax(min, max) => (min.0, max.0),
        _ => unimplemented!(),
    };
    let (min_y, max_y) = match coordinates.iter().minmax_by_key(|c| c.1) {
        itertools::MinMaxResult::MinMax(min, max) => (min.1, max.1),
        _ => unimplemented!(),
    };
    let (min_z, max_z) = match coordinates.iter().minmax_by_key(|c| c.2) {
        itertools::MinMaxResult::MinMax(min, max) => (min.2, max.2),
        _ => unimplemented!(),
    };

    let (min_x, max_x) = (min_x - 1, max_x + 1);
    let (min_y, max_y) = (min_y - 1, max_y + 1);
    let (min_z, max_z) = (min_z - 1, max_z + 1);

    let mut map = HashMap::new();
    map.insert((min_x, min_y, min_z), '-');
    for coordinate in coordinates.clone() {
        map.insert(coordinate, '#');
    }
    for z in min_z..=max_z {
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c = map.get(&(x, y, z));
                if c.is_some() {
                    continue;
                }
                let any_outside = (x, y, z)
                    .neighbours()
                    .iter()
                    .filter_map(|m| map.get(m))
                    .any(|m| m == &'-');
                if any_outside {
                    map.insert((x, y, z), '-');
                }
            }
        }
    }

    for z in (min_z..=max_z).rev() {
        for y in (min_y..=max_y).rev() {
            for x in (min_x..=max_x).rev() {
                let c = map.get(&(x, y, z));
                if c.is_some() {
                    continue;
                }
                let any_outside = (x, y, z)
                    .neighbours()
                    .iter()
                    .filter_map(|m| map.get(m))
                    .any(|m| m == &'-');
                if any_outside {
                    map.insert((x, y, z), '-');
                }
            }
        }
    }

    for z in min_z..=max_z {
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!("{}", map.get(&(x, y, z)).unwrap_or(&'.'))
            }
            println!("");
        }
        println!("///");
    }
    println!("x: {}->{}", min_x, max_x);
    println!("y: {}->{}", min_y, max_y);
    println!("z: {}->{}", min_z, max_z);

    let mut a = 0;
    for coordinate in &coordinates {
        a += coordinate
            .neighbours()
            .iter()
            .filter_map(|n| map.get(n))
            .filter(|c| *c == &'-')
            .count();
    }

    Some(a)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
