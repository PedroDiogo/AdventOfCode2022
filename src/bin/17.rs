use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Minus,
    Plus,
    L,
    I,
    Block,
}

type Position = (usize, usize);

#[derive(Debug, Clone, Copy)]
struct RockPosition {
    rock: Rock,
    position: Position,
}

impl RockPosition {
    fn initial_position(rock: &Rock, highest_rock: &Option<isize>) -> Self {
        let highest_rock = highest_rock.unwrap_or(-1);
        Self {
            rock: *rock,
            position: match rock {
                Rock::Minus => (2, (highest_rock + 4) as usize),
                Rock::Plus => (3, (highest_rock + 5) as usize),
                Rock::L => (4, (highest_rock + 4) as usize),
                Rock::I => (2, (highest_rock + 4) as usize),
                Rock::Block => (2, (highest_rock + 4) as usize),
            },
        }
    }
    fn move_down(&self, map: &HashSet<Position>) -> Option<Self> {
        let can_move = !self.positions().iter().any(|position| position.1 == 0);

        if !can_move {
            return None;
        }

        let new_position = Self {
            rock: self.rock,
            position: (self.position.0, self.position.1 - 1),
        };

        let new_position_blocked = new_position
            .positions()
            .iter()
            .any(|position| map.contains(position));

        match new_position_blocked {
            true => None,
            false => Some(new_position),
        }
    }

    fn move_side(&self, jet: &char, map: &HashSet<Position>) -> Self {
        let can_move = match jet {
            '<' => !self.positions().iter().any(|position| position.0 == 0),
            '>' => !self.positions().iter().any(|position| position.0 == 6),
            _ => unimplemented!(),
        };

        let delta = match jet {
            '<' => -1,
            '>' => 1,
            _ => unimplemented!(),
        };

        let new_position = match can_move {
            true => Self {
                rock: self.rock,
                position: ((self.position.0 as isize + delta) as usize, self.position.1),
            },
            false => *self,
        };

        let new_position_blocked = new_position
            .positions()
            .iter()
            .any(|position| map.contains(position) || position.1 == 0);

        match new_position_blocked {
            true => *self,
            false => new_position,
        }
    }

    fn positions(&self) -> Vec<Position> {
        match self.rock {
            Rock::Minus => Vec::from([
                self.position,
                (self.position.0 + 1, self.position.1),
                (self.position.0 + 2, self.position.1),
                (self.position.0 + 3, self.position.1),
            ]),
            Rock::Plus => Vec::from([
                self.position,
                (self.position.0 - 1, self.position.1),
                (self.position.0 + 1, self.position.1),
                (self.position.0, self.position.1 - 1),
                (self.position.0, self.position.1 + 1),
            ]),
            Rock::L => Vec::from([
                self.position,
                (self.position.0, self.position.1 + 1),
                (self.position.0, self.position.1 + 2),
                (self.position.0 - 1, self.position.1),
                (self.position.0 - 2, self.position.1),
            ]),
            Rock::I => Vec::from([
                self.position,
                (self.position.0, self.position.1 + 1),
                (self.position.0, self.position.1 + 2),
                (self.position.0, self.position.1 + 3),
            ]),
            Rock::Block => Vec::from([
                self.position,
                (self.position.0 + 1, self.position.1),
                (self.position.0, self.position.1 + 1),
                (self.position.0 + 1, self.position.1 + 1),
            ]),
        }
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut jet_pattern = VecDeque::from_iter(input.chars());
    let mut rock_pattern = VecDeque::from([Rock::Minus, Rock::Plus, Rock::L, Rock::I, Rock::Block]);

    let mut highest_rock: Option<isize> = None;

    let mut map = HashSet::new();
    for _ in 0..2022 {
        let rock = rock_pattern.pop_front().unwrap();
        rock_pattern.push_back(rock);
        let mut position = RockPosition::initial_position(&rock, &highest_rock);
        loop {
            let jet = jet_pattern.pop_front().unwrap();
            jet_pattern.push_back(jet);
            let side_position = position.move_side(&jet, &map);
            // println!("Side Position after {}: {:?}", jet, side_position);

            if let Some(down_position) = side_position.move_down(&map) {
                position = down_position
            } else {
                position = side_position;
                break;
            }
        }
        // println!("Rock: {:?}", position);
        for p in position.positions() {
            highest_rock = Some(match highest_rock {
                None => p.1 as isize,
                Some(h) => h.max(p.1 as isize),
            });
            map.insert(p);
        }
    }
    Some(highest_rock.unwrap() + 1)
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut jet_pattern = VecDeque::from_iter(input.chars());
    let mut rock_pattern = VecDeque::from([Rock::Minus, Rock::Plus, Rock::L, Rock::I, Rock::Block]);

    let mut highest_rock: Option<isize> = None;

    let mut map = HashSet::new();
    let mut visited: HashMap<(Rock, String, String), (Option<isize>, isize)> = HashMap::new();
    let mut i = 0;
    let mut found_cycle = false;
    while i < 1000000000000 {
        let rock = rock_pattern.pop_front().unwrap();
        rock_pattern.push_back(rock);
        let current_jet_pattern = jet_pattern.iter().join("");

        // TODO: Maintain a list of highest positions for each col, instead of computing this
        let positions = match highest_rock {
            Some(_) => (0..7)
                .map(|x| {
                    ((highest_rock.unwrap() - 50)..=highest_rock.unwrap())
                        .rev()
                        .map(|y| match map.get(&(x as usize, y as usize)) {
                            Some(_) => '#',
                            None => '.',
                        })
                        .join("")
                })
                .join("\n"),
            None => String::new(),
        };

        if let Some((high, idx)) =
            visited.get(&(rock, current_jet_pattern.clone(), positions.clone()))
        {
            if !found_cycle {
                let cycle_duration = i - idx;
                let cycle_height_increase = highest_rock.unwrap() - high.unwrap();

                let remaining_rocks = 1000000000000 - i;
                let cycles_remaining = remaining_rocks / cycle_duration;

                let rock_increment = cycles_remaining * cycle_height_increase;

                i += cycle_duration * cycles_remaining;

                found_cycle = true;

                for x in 0..7 {
                    for y in ((highest_rock.unwrap() - 50)..=highest_rock.unwrap()).rev() {
                        if map.contains(&(x as usize, y as usize)) {
                            map.insert((x as usize, (y + rock_increment) as usize));
                            break;
                        }
                    }
                }
                highest_rock = Some(highest_rock.unwrap() + rock_increment);
            }
        }
        if i > 1000 && !found_cycle {
            visited.insert((rock, current_jet_pattern, positions), (highest_rock, i));
        }
        let mut position = RockPosition::initial_position(&rock, &highest_rock);
        loop {
            let jet = jet_pattern.pop_front().unwrap();
            jet_pattern.push_back(jet);
            let side_position = position.move_side(&jet, &map);

            if let Some(down_position) = side_position.move_down(&map) {
                position = down_position
            } else {
                position = side_position;
                break;
            }
        }
        for p in position.positions() {
            highest_rock = Some(match highest_rock {
                None => p.1 as isize,
                Some(h) => h.max(p.1 as isize),
            });
            map.insert(p);
        }
        i += 1;
    }
    Some(highest_rock.unwrap() + 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
