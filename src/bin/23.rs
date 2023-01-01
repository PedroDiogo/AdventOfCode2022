use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Position = (isize, isize);

trait Neighbours {
    fn neighbours(&self, deltas: &[Position]) -> Vec<Position>;
    fn all_neighbours(&self) -> Vec<Position>;
    fn north_neighbours(&self) -> Vec<Position>;
    fn south_neighbours(&self) -> Vec<Position>;
    fn west_neighbours(&self) -> Vec<Position>;
    fn east_neighbours(&self) -> Vec<Position>;
}

impl Neighbours for Position {
    fn neighbours(&self, deltas: &[Position]) -> Vec<Self> {
        deltas
            .into_iter()
            .map(|delta| (delta.0 + self.0, delta.1 + self.1))
            .collect()
    }

    fn north_neighbours(&self) -> Vec<Self> {
        let deltas = vec![(-1, -1), (0, -1), (1, -1)];
        self.neighbours(&deltas)
    }

    fn south_neighbours(&self) -> Vec<Self> {
        let deltas = vec![(-1, 1), (0, 1), (1, 1)];
        self.neighbours(&deltas)
    }

    fn west_neighbours(&self) -> Vec<Self> {
        let deltas = vec![(-1, -1), (-1, 0), (-1, 1)];
        self.neighbours(&deltas)
    }

    fn east_neighbours(&self) -> Vec<Self> {
        let deltas = vec![(1, -1), (1, 0), (1, 1)];
        self.neighbours(&deltas)
    }

    fn all_neighbours(&self) -> Vec<Position> {
        let deltas = vec![
            (1, -1),
            (1, 0),
            (1, 1),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];
        self.neighbours(&deltas)
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut elves: HashSet<Position> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter(|(_x, c)| c == &'#')
                .map(|(x, _c)| (x as isize, y as isize))
                .collect::<Vec<Position>>()
        })
        .collect();
    println!("Elves: {:?}", elves);

    type NeightboursFn = for<'r> fn(&'r Position) -> Vec<Position>;
    let mut moves: [(NeightboursFn, Position); 4] = [
        (Neighbours::north_neighbours, (0, -1)),
        (Neighbours::south_neighbours, (0, 1)),
        (Neighbours::west_neighbours, (-1, 0)),
        (Neighbours::east_neighbours, (1, 0)),
    ];

    for _round in 0..10 {
        let starting_map = elves.clone();
        // let mut new_map;
        let mut new_moves: HashMap<Position, Vec<Position>> = HashMap::new();
        for elf in &starting_map {
            let has_neighbours = elf
                .all_neighbours()
                .iter()
                .any(|n| starting_map.contains(n));

            if has_neighbours {
                let a = moves
                    .iter()
                    .find(|m| m.0(&elf).iter().all(|n| !starting_map.contains(n)));
                if let Some((_, delta)) = a {
                    let dst = (elf.0 + delta.0, elf.1 + delta.1);

                    new_moves.entry(dst).or_insert(Vec::new()).push(*elf);
                    println!("Elf {:?} moves to {:?}", elf, dst);
                } else {
                    println!("Elf {:?} does not move", elf);
                    new_moves.entry(*elf).or_insert(Vec::new()).push(*elf);
                }
            } else {
                println!("Elf {:?} does not move. No neighbours.", elf);
                new_moves.entry(*elf).or_insert(Vec::new()).push(*elf);
            }
        }
        println!("{:?}", new_moves);
        let mut new_map: HashSet<Position> = HashSet::new();
        for (dst, possible_elves) in new_moves.iter() {
            if possible_elves.len() == 1 {
                new_map.insert(*dst);
            } else {
                println!("More than one");
                for elf in possible_elves {
                    new_map.insert(*elf);
                }
            }
        }
        elves = new_map;
        moves = [moves[1], moves[2], moves[3], moves[0]];
        println!("{:?}", elves);
    }

    let ((min_x, _), (max_x, _)) = elves.iter().minmax_by_key(|e| e.0).into_option().unwrap();
    let ((_, min_y), (_, max_y)) = elves.iter().minmax_by_key(|e| e.1).into_option().unwrap();

    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            match elves.contains(&(x, y)) {
                true => print!("#"),
                false => print!("."),
            };
        }
        println!("");
    }
    Some((max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as isize)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
