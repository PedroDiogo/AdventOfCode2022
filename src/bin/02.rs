use std::str::FromStr;

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| parse_line(line, |second_col_str| Move::from_str(second_col_str).ok()))
            .map(|(other_player, my_move)| {
                my_move as usize + game_result(&other_player, &my_move) as usize
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| parse_line(line, |second_col_str| Result::from_str(second_col_str).ok()))
            .map(|(other_player, result)| {
                find_my_move(&other_player, &result) as usize + result as usize
            })
            .sum(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, EnumString, EnumIter)]
enum Move {
    #[strum(serialize = "A", serialize = "X")]
    Rock = 1,
    #[strum(serialize = "B", serialize = "Y")]
    Paper = 2,
    #[strum(serialize = "C", serialize = "Z")]
    Scissors = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumString)]
enum Result {
    #[strum(serialize = "X")]
    Lose = 0,
    #[strum(serialize = "Y")]
    Draw = 3,
    #[strum(serialize = "Z")]
    Win = 6,
}

fn parse_line<F: Fn(&str) -> Option<Sc>, Sc>(line: &str, second_col_mapping_fn: F) -> (Move, Sc) {
    let moves = line.split_whitespace().collect_vec();
    (
        Move::from_str(moves[0]).unwrap(),
        second_col_mapping_fn(moves[1]).unwrap(),
    )
}

fn game_result(other_move: &Move, my_move: &Move) -> Result {
    if other_move == my_move {
        return Result::Draw;
    }

    if (*my_move as usize) % 3 + 1 == *other_move as usize {
        return Result::Lose;
    }

    Result::Win
}

fn find_my_move(other_move: &Move, result: &Result) -> Move {
    Move::iter()
        .find(|my_move| &game_result(other_move, my_move) == result)
        .unwrap()
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
        assert_eq!(part_two(&input), Some(12));
    }
}
