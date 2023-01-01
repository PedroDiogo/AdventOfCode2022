use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    hash::Hash,
};

use itertools::Itertools;

type Position = (isize, isize);

trait OOBAdd {
    fn oob_add(&self, other: &Self, box_size: &(usize, usize)) -> Self;
}

impl OOBAdd for Position {
    fn oob_add(&self, other: &Self, box_size: &(usize, usize)) -> Self {
        let box_size = ((*box_size).0 as isize, (*box_size).1 as isize);
        match (self.0 + other.0, self.1 + other.1) {
            (x, y) if x == 0 => (box_size.0 - 2, y),
            (x, y) if x == (box_size.0 - 1) => (1, y),
            (x, y) if y == 0 => (x, box_size.1 - 2),
            (x, y) if y == (box_size.1 - 1) => (x, 1),
            (x, y) => (x, y),
        }
    }
}

trait Neighbours {
    fn valid_neighbours(
        &self,
        blizzards_coords: &HashSet<Position>,
        box_size: &(usize, usize),
        entrance: &Position,
        exit: &Position,
    ) -> Vec<Position>;
    fn neighbours(&self) -> Vec<Position>;
}

impl Neighbours for Position {
    fn neighbours(&self) -> Vec<Position> {
        let deltas = vec![(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];
        deltas
            .into_iter()
            .map(|delta| (delta.0 + self.0, delta.1 + self.1))
            .collect()
    }

    fn valid_neighbours(
        &self,
        blizzards_coords: &HashSet<Position>,
        box_size: &(usize, usize),
        entrance: &Position,
        exit: &Position,
    ) -> Vec<Position> {
        self.neighbours()
            .into_iter()
            .filter(|n| {
                (n == entrance)
                    || (n == exit)
                    || (!blizzards_coords.contains(n)
                        && n.0 > 0
                        && n.1 > 0
                        && n.0 < ((*box_size).0 as isize) - 1
                        && n.1 < ((*box_size).1 as isize) - 1)
            })
            .collect()
    }
}

fn next_position(blizzard: &char, position: &Position, box_size: &(usize, usize)) -> Position {
    match blizzard {
        '<' => position.oob_add(&(-1, 0), box_size),
        '>' => position.oob_add(&(1, 0), box_size),
        '^' => position.oob_add(&(0, -1), box_size),
        'v' => position.oob_add(&(0, 1), box_size),
        _ => unimplemented!(),
    }
}

fn next_blizzards(
    blizzards: &HashSet<(Position, char)>,
    box_size: &(usize, usize),
) -> HashSet<(Position, char)> {
    blizzards
        .iter()
        .map(|(coords, c)| (next_position(c, coords, box_size), *c))
        .collect()
}

fn blizzards_coords(blizzards: &HashSet<(Position, char)>) -> HashSet<Position> {
    blizzards.iter().map(|(coords, _)| *coords).collect()
}

fn print_blizzards(blizzards: &HashSet<(Position, char)>, box_size: &(usize, usize)) {
    println!("{:?}", blizzards);
    let mut a: HashMap<Position, Vec<char>> = HashMap::new();
    for blizzard in blizzards {
        a.entry(blizzard.0).or_insert(Vec::new()).push(blizzard.1);
    }
    for y in 0..box_size.1 {
        for x in 0..box_size.0 {
            if y == 0 || y == box_size.1 - 1 || x == 0 || x == box_size.0 - 1 {
                print!("#");
            } else if let Some(chars) = a.get(&(x as isize, y as isize)) {
                if chars.len() > 1 {
                    print!("{}", chars.len());
                } else {
                    print!("{}", chars.first().unwrap());
                }
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let blizzards: HashSet<(Position, char)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter(|(_x, c)| c != &' ' && c != &'.' && c != &'#')
                .map(|(x, c)| ((x as isize, y as isize), c))
                .collect::<Vec<(Position, char)>>()
        })
        .collect();
    // println!("{:?}", blizzards);
    let mut entrance: Position = (0, 0);
    let mut exit: Position = (0, 0);
    let box_size = (
        input.lines().next().unwrap().trim().len(),
        input.lines().map(|_| 1).sum(),
    );

    for (y, line) in input.lines().enumerate() {
        if y == 0 {
            entrance = (
                line.char_indices().find(|(_, c)| c == &'.').unwrap().0 as isize,
                y as isize,
            );
        }
        if y + 1 == box_size.1 {
            exit = (
                line.char_indices().find(|(_, c)| c == &'.').unwrap().0 as isize,
                y as isize,
            );
        }
    }
    // println!(
    //     "box: {:?}, entrance: {:?}, exit: {:?}",
    //     box_size, entrance, exit
    // );

    let mut blizzards_by_minute: HashMap<usize, HashSet<(Position, char)>> = HashMap::new();
    blizzards_by_minute.insert(0, blizzards.clone());
    blizzards_by_minute.insert(1, next_blizzards(&blizzards, &box_size));

    let mut blizzards_coords_by_minute: HashMap<usize, HashSet<Position>> = HashMap::new();
    blizzards_coords_by_minute.insert(0, blizzards_coords(&blizzards));
    blizzards_coords_by_minute.insert(1, blizzards_coords(blizzards_by_minute.get(&1).unwrap()));

    let mut queue: VecDeque<(usize, Position)> = VecDeque::new();
    let queue_minute = 1;
    let queue_blizzards = blizzards_by_minute.get(&1).unwrap();
    let queue_blizzards_coords = blizzards_coords(&queue_blizzards);
    let valid_neighbours = entrance
        .valid_neighbours(&queue_blizzards_coords, &box_size, &entrance, &exit)
        .iter()
        .map(|n| (queue_minute, *n))
        .collect_vec();

    // println!(
    //     "Current Position: {:?} | Minute: {:?} | Next Positions: {:?}",
    //     entrance,
    //     0,
    //     valid_neighbours.iter().map(|n| n.1).collect_vec()
    // );
    // print_blizzards(&blizzards, &box_size);

    for n in valid_neighbours {
        queue.push_back(n);
    }
    // println!("Q: {:?}", queue);

    let mut visited: HashSet<(usize, Position)> = HashSet::new();
    loop {
        let a = queue.pop_front().unwrap();
        if visited.contains(&a) {
            // println!("Visited");
        } else {
            // println!("Not Visited");
            visited.insert(a);
            let queue_minute = a.0 + 1;
            // println!("Minute: {}", queue_minute);
            let last_blizzards = &blizzards_by_minute[&a.0].clone();
            let queue_blizzards = blizzards_by_minute
                .entry(queue_minute)
                .or_insert(next_blizzards(last_blizzards, &box_size));
            let queue_blizzards_coords = blizzards_coords_by_minute
                .entry(queue_minute)
                .or_insert(blizzards_coords(&queue_blizzards));

            let valid_neighbours =
                a.1.valid_neighbours(&queue_blizzards_coords, &box_size, &entrance, &exit)
                    .iter()
                    .map(|n| (queue_minute, *n))
                    .collect_vec();
            // println!(
            //     "Current Position: {:?} | Minute: {:?} | Next Positions: {:?}",
            //     a.1,
            //     a.0,
            //     valid_neighbours.iter().map(|n| n.1).collect_vec()
            // );
            // print_blizzards(&queue_blizzards, &box_size);
            for n in valid_neighbours {
                if n.1 == exit {
                    return Some(n.0);
                }
                if !visited.contains(&n) {
                    queue.push_back(n);
                }
            }
        }
        // break None;
        // println!("Q: {:?}", queue);
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), None);
    }
}
