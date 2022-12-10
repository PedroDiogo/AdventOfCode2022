use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let mut cycle: usize = 0;
    let mut registers = vec![1];
    let mut result = 0;
    for command in input.lines() {
        let old_cycle = cycle;
        let old_registers = registers.clone();

        let parts = command.split_ascii_whitespace().collect_vec();
        cycle += match parts[..] {
            ["noop"] => 1,
            ["addx", a] => addx(a, &mut registers),
            _ => unimplemented!(),
        };

        if (cycle + 20) % 40 == 0 {
            result += cycle * old_registers[0] as usize;
        } else if ((cycle + 20) % 40) < ((old_cycle + 20) % 40) {
            result += (cycle - ((cycle + 20) % 40)) * old_registers[0] as usize;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cycle: usize = 0;
    let mut registers = vec![1];
    for command in input.lines() {
        let old_cycle = cycle;
        let old_registers = registers.clone();

        let parts = command.split_ascii_whitespace().collect_vec();
        cycle += match parts[..] {
            ["noop"] => 1,
            ["addx", a] => addx(a, &mut registers),
            _ => unimplemented!(),
        };

        for c in old_cycle..cycle {
            let position = c % 40;
            if (position as isize - old_registers[0]).abs() < 2 {
                print!("#")
            } else {
                print!(".");
            }
            if position == 39 {
                println!("");
            }
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn addx(value: &str, registers: &mut [isize]) -> usize {
    let value = value.parse::<isize>().unwrap();
    registers[0] += value;
    2
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
        assert_eq!(part_two(&input), None);
    }
}
