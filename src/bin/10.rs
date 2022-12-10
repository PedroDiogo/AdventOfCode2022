use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let mut cycle: usize = 0;
    let mut x = 1;
    let mut result = 0;

    for command in input.lines() {
        let parts = command.split_ascii_whitespace().collect_vec();
        let (cycle_inc, new_x) = match parts[..] {
            ["noop"] => (1, x),
            ["addx", a] => addx(a, &x),
            _ => unimplemented!(),
        };
        let new_cycle = cycle + cycle_inc;

        if (new_cycle + 20) % 40 == 0 {
            result += new_cycle * x as usize;
        } else if ((new_cycle + 20) % 40) < ((cycle + 20) % 40) {
            result += (new_cycle - ((new_cycle + 20) % 40)) * x as usize;
        }

        cycle = new_cycle;
        x = new_x;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut cycle: usize = 0;
    let mut x = 1;
    let mut crt = vec!['.'; 40 * 6];

    for command in input.lines() {
        let parts = command.split_ascii_whitespace().collect_vec();
        let (cycle_inc, new_x) = match parts[..] {
            ["noop"] => (1, x),
            ["addx", a] => addx(a, &x),
            _ => unimplemented!(),
        };
        let new_cycle = cycle + cycle_inc;

        for c in cycle..new_cycle {
            let position = c % 40;
            if (position as isize - x).abs() < 2 {
                crt[c] = '#';
            }
        }

        cycle = new_cycle;
        x = new_x;
    }
    Some(
        crt.chunks(40)
            .map(|c| c.iter().collect::<String>())
            .join("\n"),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn addx(value: &str, x: &isize) -> (usize, isize) {
    let value = value.parse::<isize>().unwrap();
    (2, x + value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        let output = String::from(
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....",
        );
        assert_eq!(part_two(&input), Some(output));
    }
}
