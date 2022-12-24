use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Value {
    Number(isize),
    Operation(String, char, String),
}

impl Value {
    fn from_str(input: &str) -> Option<Value> {
        let input = input.trim();
        if let Some(number) = input.parse::<isize>().ok() {
            return Some(Value::Number(number));
        } else {
            let mut input = input.split_whitespace();
            return Some(Value::Operation(
                input.next()?.to_string(),
                input.next()?.bytes().next()? as char,
                input.next()?.to_string(),
            ));
        }
    }

    fn calculate(&self, a: isize, b: isize) -> Option<isize> {
        match self {
            Value::Operation(_, '+', _) => Some(a + b),
            Value::Operation(_, '-', _) => Some(a - b),
            Value::Operation(_, '*', _) => Some(a * b),
            Value::Operation(_, '/', _) => Some(a / b),
            _ => None,
        }
    }

    fn number(&self) -> Option<isize> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }
    fn is_operation(&self) -> bool {
        match self {
            Value::Operation(_, _, _) => true,
            _ => false,
        }
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut a: HashMap<&str, Value> = input
        .lines()
        .map(|line| {
            let mut split = line.split(':');
            let input = split.next().unwrap();
            let value = split.next().unwrap();
            (input, Value::from_str(value).unwrap())
        })
        .collect();

    let mut operations = a.iter().filter(|(_, v)| v.is_operation()).count();

    while operations > 0 {
        let b = a.clone();
        let b = b.iter().filter(|(_, v)| v.is_operation());

        for (key, op) in b {
            if let Value::Operation(input_1, _, input_2) = op {
                let input_1_number = a.get(input_1.as_str()).unwrap().number();
                let input_2_number = a.get(input_2.as_str()).unwrap().number();

                if input_1_number.is_some() && input_2_number.is_some() {
                    let result = op
                        .calculate(input_1_number.unwrap(), input_2_number.unwrap())
                        .unwrap();
                    operations -= 1;
                    a.insert(*key, Value::Number(result));
                }
            }
        }
    }

    a.get("root")?.number()
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }
}
