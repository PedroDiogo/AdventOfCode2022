use std::collections::HashMap;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| parse_line(line))
            .map(|(other_player, my_move)| {
                shape_score(&my_move) + game_score(&other_player, &my_move)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn parse_line(line: &str) -> (Move, Move) {
    let mapping = HashMap::from([
        ("A", Move::Rock),
        ("X", Move::Rock),
        ("B", Move::Paper),
        ("Y", Move::Paper),
        ("C", Move::Scissors),
        ("Z", Move::Scissors),
    ]);

    let moves = line.split_whitespace().collect_vec();
    (mapping[(moves[0])], mapping[moves[1]])
}

fn shape_score(my_move: &Move) -> usize {
    match my_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn game_score(other_move: &Move, my_move: &Move) -> usize {
    if other_move == my_move {
        return 3;
    }

    if (other_move == &Move::Rock && my_move == &Move::Scissors)
        || (other_move == &Move::Paper && my_move == &Move::Rock)
        || (other_move == &Move::Scissors && my_move == &Move::Paper)
    {
        return 0;
    }

    6
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
