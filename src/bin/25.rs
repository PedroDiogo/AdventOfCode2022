fn parse_snafu(input: &str) -> isize {
    let result = input
        .chars()
        .rev()
        .enumerate()
        .fold(0, |accum, (idx, char)| {
            accum
                + (5_isize.pow(idx as u32)
                    * match char {
                        '-' => -1,
                        '=' => -2,
                        x => (x as u8 - '0' as u8) as isize,
                    })
        });
    result
}

fn to_snafu(input: &isize) -> String {
    let mut input = input.clone();
    let mut result = String::new();
    let mut carry_over = 0;
    while input > 0 {
        let digit = (input + carry_over) % 5;
        let (digit, new_carry_over) = match digit {
            0 => ('0', carry_over),
            3 => ('=', 1),
            4 => ('-', 1),
            x => (('0' as u8 + x as u8) as char, 0),
        };
        carry_over = new_carry_over;
        result.insert(0, digit);
        input /= 5;
    }
    if carry_over > 0 {
        result.insert(0, '1');
    }
    result
}
pub fn part_one(input: &str) -> Option<String> {
    let sum: isize = input.lines().map(|line| parse_snafu(line)).sum();
    let result = to_snafu(&sum);
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some(String::from("2=-1=0")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
