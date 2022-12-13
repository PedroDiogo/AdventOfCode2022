use std::{cmp::Ordering, ops::Index, vec};

use itertools::Itertools;
use regex::Regex;

type Packet = Vec<Item>;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
    Number(usize),
    List(Packet),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Item::Number(n_self), Item::Number(n_other)) => n_self.cmp(n_other),
            (n_self @ Item::Number(_), l_other @ Item::List(_)) => {
                Item::List(vec![n_self.clone()]).cmp(l_other)
            }
            (l_self @ Item::List(_), n_other @ Item::Number(_)) => {
                l_self.cmp(&Item::List(vec![n_other.clone()]))
            }
            (Item::List(l_self), Item::List(l_other)) => {
                let mut order = Ordering::Equal;
                for i in 0..l_self.len().max(l_other.len()) {
                    order = match (l_self.get(i), l_other.get(i)) {
                        (Some(_), None) => Ordering::Less,
                        (None, Some(_)) => Ordering::Greater,
                        (Some(s), Some(o)) => s.cmp(o),
                        _ => unimplemented!(),
                    };

                    if order != Ordering::Equal {
                        break;
                    }
                }
                order
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    for block in input.split("\n\n") {
        let mut block = block.lines();
        let packet1 = block.next();
        let packet1 = parse_packet(packet1);
        let packet2 = block.next();
        let packet2 = parse_packet(packet2);
        println!("{:?} < {:?} = {}", packet1, packet2, packet1 < packet2);
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn parse_packet(input: Option<&str>) -> Vec<Item> {
    // let mut packet: Packet = vec![];
    // let mut queue = vec![];
    // if input.is_none() {
    //     return packet;
    // }
    // println!("Input: {}", input.unwrap());
    // let input = input
    //     .unwrap()
    //     .split_inclusive(&[',', '[', ']'])
    //     .flat_map(|s| s.split("]"))
    //     .filter(|s| *s != ",")
    //     .map(|s| s.replace(",", ""));
    // println!("{:?}", input.clone().collect_vec());

    // for s in input {
    //     match s.as_str() {
    //         "[" => queue.push(Item::List(vec![])),
    //         "" => {
    //             if let Some(queue_elem) = queue.pop() {
    //                 packet.push(queue_elem);
    //             }
    //         }
    //         x => {
    //             let x = x.parse::<usize>().unwrap();
    //             match queue.last_mut() {
    //                 Some(Item::List(list)) => list.push(Item::Number(x)),
    //                 _ => unimplemented!(),
    //             }
    //         }
    //     }
    // }
    // packet

    vec![]
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
