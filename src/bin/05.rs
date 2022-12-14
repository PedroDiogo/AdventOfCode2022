use itertools::Itertools;

type Stack = Vec<char>;

#[derive(Debug)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}
pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    for m in moves {
        let (bottom, top) = stacks[m.from].split_at(stacks[m.from].len() - m.quantity);
        let bottom = bottom.to_vec();
        let mut top = top.to_vec();
        top.reverse();
        stacks[m.from] = bottom;
        stacks[m.to].append(&mut top);
    }
    Some(stacks.iter().map(|stack| stack.last().unwrap()).join(""))
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    for m in moves {
        let (bottom, top) = stacks[m.from].split_at(stacks[m.from].len() - m.quantity);
        let bottom = bottom.to_vec();
        let mut top = top.to_vec();
        stacks[m.from] = bottom;
        stacks[m.to].append(&mut top);
    }
    Some(stacks.iter().map(|stack| stack.last().unwrap()).join(""))
}

fn parse_input(input: &str) -> (Vec<Stack>, Vec<Move>) {
    let (starting_stacks, moves) = input.split("\n\n").collect_tuple().unwrap();
    let num_of_columns = starting_stacks
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .count();

    let stacks = starting_stacks.lines().dropping_back(1).fold(
        vec![Stack::new(); num_of_columns],
        |mut current_stacks, line| {
            let line = line.as_bytes();
            for idx in 0..num_of_columns {
                let character_at_idx = line[idx * 4 + 1] as char;
                if character_at_idx != ' ' {
                    current_stacks[idx].insert(0, character_at_idx);
                }
            }
            current_stacks
        },
    );

    let moves = moves
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect_vec();
            Move {
                quantity: split[1].parse().unwrap(),
                from: split[3].parse::<usize>().unwrap() - 1,
                to: split[5].parse::<usize>().unwrap() - 1,
            }
        })
        .collect_vec();
    (stacks, moves)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
