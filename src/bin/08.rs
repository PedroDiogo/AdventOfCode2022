use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - '0' as u8).collect_vec())
        .collect_vec();

    let (y_max, x_max) = (input.len(), input.first().unwrap().len());

    let mut visible = vec![vec![(true, true, true, true); x_max]; y_max];

    // Seen from the top
    for x in 0..x_max {
        let mut current_max = 0;
        for y in 0..y_max {
            if input[y][x] > current_max {
                current_max = input[y][x];
            // } else {
            } else if x > 0 && y > 0 && x + 1 < x_max && y + 1 < y_max {
                visible[y][x].0 = false;
            }
        }
    }

    // Seen from the right
    for y in 0..y_max {
        let mut current_max = 0;
        for x in (0..x_max).rev() {
            if input[y][x] > current_max {
                current_max = input[y][x];
            // } else {
            } else if x > 0 && y > 0 && x + 1 < x_max && y + 1 < y_max {
                visible[y][x].1 = false;
            }
        }
    }

    // Seen from the bottom
    for x in 0..x_max {
        let mut current_max = 0;
        for y in (0..y_max).rev() {
            if input[y][x] > current_max {
                current_max = input[y][x];
            // } else {
            } else if x > 0 && y > 0 && x + 1 < x_max && y + 1 < y_max {
                visible[y][x].2 = false;
            }
        }
    }

    // Seen from the left
    for y in 0..y_max {
        let mut current_max = 0;
        for x in 0..x_max {
            if input[y][x] > current_max {
                current_max = input[y][x];
            } else if x > 0 && y > 0 && x + 1 < x_max && y + 1 < y_max {
                visible[y][x].3 = false;
            }
        }
    }

    Some(
        visible
            .iter()
            .flatten()
            .filter(|v| v.0 || v.1 || v.2 || v.3)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - '0' as u8).collect_vec())
        .collect_vec();

    let (y_max, x_max) = (input.len(), input.first().unwrap().len());

    let mut visible = vec![vec![(0, 0, 0, 0); x_max]; y_max];

    for x in 0..x_max {
        for y in 0..y_max {
            // Looking left
            let mut found = false;
            for i in (0..x).rev() {
                if input[y][i] >= input[y][x] {
                    visible[y][x].0 = x - i;
                    found = true;
                    break;
                }
            }
            if !found {
                visible[y][x].0 = x;
            }
            // Looking right
            let mut found = false;
            for i in x + 1..x_max {
                if input[y][i] >= input[y][x] {
                    visible[y][x].1 = i - x;
                    found = true;
                    break;
                }
            }
            if !found {
                visible[y][x].1 = x_max - (x + 1);
            }
            // Looking down
            let mut found = false;
            for i in y + 1..y_max {
                if input[i][x] >= input[y][x] {
                    visible[y][x].2 = i - y;
                    found = true;
                    break;
                }
            }
            if !found {
                visible[y][x].2 = y_max - (y + 1);
            }
            // Looking up
            let mut found = false;
            for i in (0..y).rev() {
                if input[i][x] >= input[y][x] {
                    visible[y][x].3 = y - i;
                    found = true;
                    break;
                }
            }
            if !found {
                visible[y][x].3 = y;
            }
        }
    }

    visible
        .into_iter()
        .flatten()
        .map(|v| v.0 * v.1 * v.2 * v.3)
        .max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
