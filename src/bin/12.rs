use std::collections::{BinaryHeap, HashMap, HashSet};

type Position = (usize, usize);

#[derive(PartialEq, Eq, Debug)]
struct AStar {
    node: Position,
    f: usize,
    g: usize,
    // h: usize,
}

impl Ord for AStar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for AStar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const VALID_POSITION_DELTAS: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

pub fn part_one(input: &str) -> Option<usize> {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start: Position = (0, 0);
    let mut end: Position = (0, 0);
    let (m_max, n_max) = (map.len(), map[0].len());
    for m in 0..m_max {
        for n in 0..n_max {
            match map[m][n] {
                'S' => start = (m, n),
                'E' => end = (m, n),
                _ => (),
            }
        }
    }
    map[start.0][start.1] = 'a';
    map[end.0][end.1] = 'z';

    println!("{:?} -> {:?}", start, end);

    let mut open: BinaryHeap<AStar> = BinaryHeap::from([AStar {
        node: start,
        f: 0,
        g: 0,
    }]);
    let mut open_map: HashMap<Position, usize> = HashMap::from([(start, 0)]);
    let mut closed: HashSet<Position> = HashSet::new();
    let mut processed = 0;

    while !open.is_empty() {
        processed += 1;
        let q = open.pop().unwrap();
        open_map.remove(&q.node);
        closed.insert(q.node);

        if q.node == end {
            return Some(q.f);
        }

        // if processed % 1000 == 0 {
        //     println!("Processed: {:?}. Q: {:?}", processed, q);
        //     for m in 0..m_max {
        //         for n in 0..n_max {
        //             let a = match closed.get(&(m, n)) {
        //                 Some(_True) => '#',
        //                 _ => '.',
        //             };
        //             print!("{}", a);
        //         }
        //         println!("");
        //     }
        // }

        for delta in VALID_POSITION_DELTAS {
            let sucessor_pos = (q.node.0 as isize + delta.0, q.node.1 as isize + delta.1);
            if sucessor_pos.0 < 0
                || sucessor_pos.1 < 0
                || sucessor_pos.0 >= m_max as isize
                || sucessor_pos.1 >= n_max as isize
            {
                continue;
            }
            let sucessor_pos = (sucessor_pos.0 as usize, sucessor_pos.1 as usize);
            if closed.contains(&sucessor_pos) {
                continue;
            }
            if (map[sucessor_pos.0][sucessor_pos.1] as i8 - map[q.node.0][q.node.1] as i8) > 1 {
                continue;
            }

            let g = q.g + 1;
            let h: usize = ((sucessor_pos.0 as isize - end.0 as isize).abs()
                + (sucessor_pos.1 as isize - end.1 as isize).abs())
                as usize;
            // let h = ((sucessor_pos.0 as isize - end.0 as isize).pow(2)
            //     + (sucessor_pos.1 as isize - end.1 as isize).pow(2)) as usize;
            let f = g + h;

            if let Some(open_node) = open_map.get(&sucessor_pos) {
                if open_node < &g {
                    continue;
                }
            }

            open.push(AStar {
                node: sucessor_pos,
                f,
                g,
            });
            open_map.insert(sucessor_pos, g);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
