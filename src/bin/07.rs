use itertools::Itertools;

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.split("$").map(|l| l.trim()).filter(|i| !i.is_empty());
    let mut filesystem = vec![Node {
        idx: 0,
        val: 0,
        parent: None,
        children: vec![],
    }];
    let mut current_directory = 0;

    for command_output in input {
        let mut command_output = command_output.lines();
        let command = command_output.next().unwrap();

        if command == "ls" {
            for ls_line in command_output {
                if let Some(size) = ls_line
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .ok()
                {
                    filesystem[current_directory].val += size;
                }
            }
        } else {
            match command.split_whitespace().last() {
                Some("..") => current_directory = filesystem[current_directory].parent.unwrap(),
                Some("/") => current_directory = 0,
                _ => {
                    let new_directory = Node {
                        idx: filesystem.len(),
                        val: 0,
                        parent: Some(current_directory),
                        children: vec![],
                    };
                    let new_idx = new_directory.idx;
                    filesystem[current_directory].children.push(new_idx);
                    filesystem.push(new_directory);
                    current_directory = new_idx;
                }
            }
        }
    }
    Some(
        filesystem
            .iter()
            .map(|directory| calculate_total_size(&filesystem, directory.idx))
            .filter(|total_size| *total_size <= 100000)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.split("$").map(|l| l.trim()).filter(|i| !i.is_empty());
    let mut filesystem = vec![Node {
        idx: 0,
        val: 0,
        parent: None,
        children: vec![],
    }];
    let mut current_directory = 0;

    for command_output in input {
        let mut command_output = command_output.lines();
        let command = command_output.next().unwrap();

        if command == "ls" {
            for ls_line in command_output {
                if let Some(size) = ls_line
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .ok()
                {
                    filesystem[current_directory].val += size;
                }
            }
        } else {
            match command.split_whitespace().last() {
                Some("..") => current_directory = filesystem[current_directory].parent.unwrap(),
                Some("/") => current_directory = 0,
                _ => {
                    let new_directory = Node {
                        idx: filesystem.len(),
                        val: 0,
                        parent: Some(current_directory),
                        children: vec![],
                    };
                    let new_idx = new_directory.idx;
                    filesystem[current_directory].children.push(new_idx);
                    filesystem.push(new_directory);
                    current_directory = new_idx;
                }
            }
        }
    }

    let total_sizes = filesystem
        .iter()
        .map(|directory| calculate_total_size(&filesystem, directory.idx))
        .collect_vec();

    let root_file_size = total_sizes.iter().max().unwrap();
    let unused = 70000000 - root_file_size;
    let need = 30000000 - unused;

    total_sizes
        .into_iter()
        .filter(|total_size| *total_size >= need)
        .min()
}

fn calculate_total_size(filesystem: &[Node<usize>], node: usize) -> usize {
    let a: usize = filesystem[node]
        .children
        .iter()
        .map(|child| calculate_total_size(filesystem, *child))
        .sum();
    return filesystem[node].val + a;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
