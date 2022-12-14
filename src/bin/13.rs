use std::{cmp::Ordering, vec};

use itertools::Itertools;
use regex::Regex;
use serde_json::Value;

type Packet = Vec<Item>;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
    Number(usize),
    List(Packet),
}

    fn cmp_value(left: &Value, right: &Value) -> std::cmp::Ordering {
        match (left, right) {
            (Value::Number(n_self), Value::Number(n_other)) => n_self.as_u64().cmp(&n_other.as_u64()),
            (n_self @ Value::Number(_), l_other @ Value::Array(_)) => {
                cmp_value(&Value::Array(vec![n_self.clone()]), l_other)
            }

            (l_self @ Value::Array(_), n_other @ Value::Number(_)) => {
                cmp_value(l_self, &Value::Array(vec![n_other.clone()]))
            }
            (Value::Array(l_self), Value::Array(l_other)) => {
                let mut order = Ordering::Equal;
                for i in 0..l_self.len().max(l_other.len()) {
                    order = match (l_self.get(i), l_other.get(i)) {
                        (Some(_), None) =>  Ordering::Greater,
                        (None, Some(_)) =>  Ordering::Less,
                        (Some(s), Some(o)) => cmp_value(s, o),
                        _ => unimplemented!(),
                    };

                    if order != Ordering::Equal {
                        break;
                    }
                }
                order
            }
         _ => unimplemented!()
            }
    }

pub fn part_one(input: &str) -> Option<usize> {
    let mut idx = 1;
    let mut sum = 0;
    for block in input.split("\n\n") {
        let mut block = block.lines();
        let packet1 = block.next();
        let packet1 = parse_packet(packet1);
        let packet2 = block.next();
        let packet2 = parse_packet(packet2);
        if cmp_value(&packet1, &packet2) == Ordering::Less {
            sum += idx;
        }
        idx += 1;
    }
    Some(sum) 
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn parse_packet(input: Option<&str>) -> Value {
    serde_json::from_str(input.unwrap()).unwrap()
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
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
