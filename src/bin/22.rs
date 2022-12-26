use std::collections::HashMap;

use itertools::Itertools;

type Position = (isize, isize);

#[derive(Debug)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn turn(&self, c: &char) -> Self {
        match (self, c) {
            (Facing::Down, 'L') => Facing::Right,
            (Facing::Down, 'R') => Facing::Left,
            (Facing::Right, 'L') => Facing::Up,
            (Facing::Right, 'R') => Facing::Down,
            (Facing::Up, 'L') => Facing::Left,
            (Facing::Up, 'R') => Facing::Right,
            (Facing::Left, 'L') => Facing::Down,
            (Facing::Left, 'R') => Facing::Up,
            _ => unimplemented!(),
        }
    }

    fn to_number(&self) -> isize {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }
}

fn next_position(
    current_position: &Position,
    facing: &Facing,
    map: &HashMap<Position, char>,
) -> Option<Position> {
    let delta: (isize, isize) = match facing {
        Facing::Right => (1, 0),
        Facing::Down => (0, 1),
        Facing::Left => (-1, 0),
        Facing::Up => (0, -1),
    };

    let mut new_position = (current_position.0 + delta.0, current_position.1 + delta.1);

    loop {
        match map.get(&new_position) {
            Some('.') => return Some(new_position),
            Some('#') => return Some(*current_position),
            None => new_position = find_opposite(current_position, facing, map),
            _ => unimplemented!(),
        }
    }
}

fn find_opposite(
    current_position: &Position,
    facing: &Facing,
    map: &HashMap<Position, char>,
) -> Position {
    let max = (
        map.keys().max_by_key(|p| p.0).unwrap().0,
        map.keys().max_by_key(|p| p.1).unwrap().1,
    );
    let delta: (isize, isize) = match facing {
        Facing::Right => (1, 0),
        Facing::Down => (0, 1),
        Facing::Left => (-1, 0),
        Facing::Up => (0, -1),
    };
    let mut position = match facing {
        Facing::Right => (0, current_position.1),
        Facing::Down => (current_position.0, 0),
        Facing::Left => (max.0, current_position.1),
        Facing::Up => (current_position.0, max.1),
    };

    loop {
        if let Some(_) = map.get(&position) {
            return position;
        } else {
            position = (position.0 + delta.0, position.1 + delta.1);
        }
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let (whole_map, directions) = match input.split("\n\n").collect_tuple() {
        Some((map, directions)) => (map, directions),
        _ => unimplemented!(),
    };

    let directions = directions
        .split_inclusive(['L', 'R'])
        .flat_map(|s| {
            let s = s.to_string();
            match s.parse::<usize>() {
                Result::Ok(_) => vec![s],
                _ => vec![s[0..s.len() - 1].to_string(), s[s.len() - 1..].to_string()],
            }
        })
        .collect_vec();

    println!("{:?}", directions);
    let mut start_position = (isize::MAX, isize::MAX);
    let mut map = HashMap::new();
    for (y, line) in whole_map.lines().enumerate() {
        for (x, c) in line.char_indices() {
            let (x, y) = (x as isize, y as isize);
            if c == '.' || c == '#' {
                map.insert((x, y), c);
                if x <= start_position.0 && y <= start_position.1 {
                    start_position = (x, y);
                }
            }
        }
    }
    println!("Start: {:?}", start_position);
    println!("map: {:?}", map);

    let mut pos = start_position;
    let mut facing = Facing::Right;

    for direction in directions {
        if direction == "L" || direction == "R" {
            facing = facing.turn(&direction.chars().next().unwrap());
        } else {
            for _ in 0..direction.parse().unwrap() {
                if let Some(new_pos) = next_position(&pos, &facing, &map) {
                    pos = new_pos;
                } else {
                    break;
                }
            }
        }
    }
    println!("Position: {:?}", pos);
    Some((pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + facing.to_number())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
