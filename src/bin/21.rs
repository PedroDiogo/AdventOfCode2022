use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Value {
    Number(isize),
    Operation(String, char, String),
    Human,
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

    fn dfs(&self, map: &HashMap<&str, Self>) -> String {
        match self {
            Value::Number(n) => format!("{}", n),
            Value::Human => format!("Human"),
            Value::Operation(i_1, c, i_2) => format!(
                "({}) {} ({})",
                map.get(i_1.as_str()).unwrap().dfs(map),
                c,
                map.get(i_2.as_str()).unwrap().dfs(map)
            ),
        }
    }

    fn calculate_opposite(a: &isize, c: &char, b: &isize) -> Option<isize> {
        match c {
            '+' => Some(a - b),
            '-' => Some(a + b),
            '*' => Some(a / b),
            '/' => Some(a * b),
            _ => None,
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

pub fn part_two(input: &str) -> Option<isize> {
    let mut a: HashMap<&str, Value> = input
        .lines()
        .map(|line| {
            let mut split = line.split(':');
            let input = split.next().unwrap();
            let value = split.next().unwrap();
            match input {
                "humn" => (input, Value::Human),
                _ => (input, Value::from_str(value).unwrap()),
            }
        })
        .collect();

    let mut operations = a.iter().filter(|(_, v)| v.is_operation()).count();
    let mut previous_operations = operations;

    while operations > 0 {
        let b = a.clone();
        let b = b.iter().filter(|(_, v)| v.is_operation());

        for (key, op) in b {
            if let Value::Operation(input_1, c, input_2) = op {
                if c == &'=' {
                    continue;
                }
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
        if previous_operations == operations {
            break;
        }
        previous_operations = operations;
    }

    let b = a.clone();
    loop {
        if let Some(Value::Operation(left, _, right)) = b.get("root") {
            match (a.get(left.as_str()), a.get(right.as_str())) {
                (Some(Value::Number(n)), Some(Value::Operation(l, c, r))) => {
                    // update left
                    if let Some(l_number) = b.get(l.as_str()).unwrap().number() {
                        let to_save =
                            Value::Number(Value::calculate_opposite(n, c, &l_number).unwrap());
                        a.insert(right.as_str(), b.get(r.as_str()).unwrap().clone());
                        a.insert(left.as_str(), to_save);
                    } else if let Some(r_number) = b.get(r.as_str()).unwrap().number() {
                        let to_save =
                            Value::Number(Value::calculate_opposite(n, c, &r_number).unwrap());
                        a.insert(right.as_str(), b.get(l.as_str()).unwrap().clone());
                        a.insert(left.as_str(), to_save);
                    } else {
                        break;
                    };
                }
                (Some(Value::Operation(l, c, r)), Some(Value::Number(n))) => {
                    // update right
                    if let Some(l_number) = b.get(l.as_str()).unwrap().number() {
                        let to_save = if c == &'-' {
                            Value::Number(-(n - l_number))
                        } else {
                            Value::Number(Value::calculate_opposite(n, c, &l_number).unwrap())
                        };

                        a.insert(left.as_str(), b.get(r.as_str()).unwrap().clone());
                        a.insert(right.as_str(), to_save);
                    } else if let Some(r_number) = b.get(r.as_str()).unwrap().number() {
                        let to_save =
                            Value::Number(Value::calculate_opposite(n, c, &r_number).unwrap());
                        a.insert(left.as_str(), b.get(l.as_str()).unwrap().clone());
                        a.insert(right.as_str(), to_save);
                    } else {
                        break;
                    };
                }
                _ => break,
            }
        } else {
            unimplemented!()
        }
    }

    if let Some(Value::Operation(left, _, right)) = a.get("root") {
        match (a.get(left.as_str()), a.get(right.as_str())) {
            (Some(Value::Number(x)), _) => return Some(*x),
            (_, Some(Value::Number(x))) => return Some(*x),
            _ => todo!(),
        }
    };
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
        assert_eq!(part_two(&input), Some(301));
    }
}
