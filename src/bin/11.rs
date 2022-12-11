use std::collections::VecDeque;

use advent_of_code::helpers::GetNumbers;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    inspections: usize,
    operation: Operation,
    test: Test,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Multiply(usize),
    Add(usize),
    Square,
}

impl Operation {
    fn result(&self, old: &usize) -> usize {
        match self {
            Operation::Add(x) => old + x,
            Operation::Multiply(x) => old * x,
            Operation::Square => old * old,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Test {
    divisible_by: usize,
    return_if_true: usize,
    return_if_false: usize,
}

impl Test {
    fn result(&self, input: &usize) -> usize {
        match input % self.divisible_by {
            0 => self.return_if_true,
            _ => self.return_if_false,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = parse_input(input);

    for _round in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            let mut monkey = monkeys[monkey_idx].clone();
            while !monkey.items.is_empty() {
                monkey.inspections += 1;
                let item = monkey.items.pop_front().unwrap();
                let item = monkey.operation.result(&item) / 3;
                monkeys[monkey.test.result(&item)].items.push_back(item);
            }
            monkeys[monkey_idx] = monkey;
        }
    }

    Some(
        monkeys
            .iter()
            .sorted_by(|a, b| Ord::cmp(&b.inspections, &a.inspections))
            .take(2)
            .fold(1, |mul, item| mul * item.inspections),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys = parse_input(input);

    // LCM * HCF = Product of all numbers
    // inputs are are co-prime so HCF is 1
    let lcm: usize = monkeys.iter().map(|m| m.test.divisible_by).product();

    for _round in 0..10000 {
        for monkey_idx in 0..monkeys.len() {
            let mut monkey = monkeys[monkey_idx].clone();
            while !monkey.items.is_empty() {
                monkey.inspections += 1;
                let item = monkey.items.pop_front().unwrap();
                let item = (monkey.operation.result(&item)) % lcm;
                monkeys[monkey.test.result(&item)].items.push_back(item);
            }
            monkeys[monkey_idx] = monkey;
        }
    }

    Some(
        monkeys
            .iter()
            .sorted_by(|a, b| Ord::cmp(&b.inspections, &a.inspections))
            .take(2)
            .fold(1, |mul, item| mul * item.inspections),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let monkeys = input.split("\n\n");

    monkeys
        .map(|monkey_lines| {
            let monkey_lines = monkey_lines.lines().collect_vec();
            let items = VecDeque::from(monkey_lines[1].replace(",", "").numbers());

            let operation = match monkey_lines[2]
                .split(" = ")
                .skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .collect_vec()[..]
            {
                ["old", "*", "old"] => Operation::Square,
                ["old", "+", x] => Operation::Add(x.parse().unwrap()),
                ["old", "*", x] => Operation::Multiply(x.parse().unwrap()),
                _ => unimplemented!(),
            };

            let test = Test {
                divisible_by: *monkey_lines[3].numbers().first().unwrap(),
                return_if_true: *monkey_lines[4].numbers().first().unwrap(),
                return_if_false: *monkey_lines[5].numbers().first().unwrap(),
            };

            Monkey {
                items,
                inspections: 0,
                operation,
                test,
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
