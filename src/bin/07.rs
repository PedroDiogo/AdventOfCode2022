use std::vec;

#[derive(Debug)]
struct Directory<T>
where
    T: PartialEq,
{
    idx: usize,
    size: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Directory<T>
where
    T: PartialEq,
{
    fn new(idx: usize, size: T, parent: Option<usize>) -> Directory<T> {
        Directory {
            idx: idx,
            size: size,
            parent: parent,
            children: vec![],
        }
    }
}

type Filesystem = Vec<Directory<usize>>;
const ROOT_DIRECTORY: usize = 0;

pub fn part_one(input: &str) -> Option<usize> {
    let filesystem = create_filesystem_from_input(input);
    Some(
        filesystem
            .iter()
            .map(|directory| calculate_directory_total_size(&filesystem, directory.idx))
            .filter(|total_size| *total_size <= 100000)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let filesystem = create_filesystem_from_input(input);
    let root_directory_size = calculate_directory_total_size(&filesystem, 0);
    let need_size = root_directory_size - (70000000 - 30000000);

    filesystem
        .iter()
        .map(|directory| calculate_directory_total_size(&filesystem, directory.idx))
        .filter(|directory_total_size| *directory_total_size >= need_size)
        .min()
}

fn create_filesystem_from_input(input: &str) -> Filesystem {
    let mut filesystem = vec![Directory::new(ROOT_DIRECTORY, 0, None)];
    let mut current_directory = ROOT_DIRECTORY;

    let input = input.split("$").map(|l| l.trim()).filter(|i| !i.is_empty());

    for command_output in input {
        let mut command_output = command_output.lines();
        let command = command_output.next().unwrap();

        if command == "ls" {
            for ls_line in command_output {
                if let Some(file_size) = get_file_size_from_line(ls_line) {
                    filesystem[current_directory].size += file_size;
                }
            }
        } else {
            match command.split_whitespace().last() {
                Some("..") => current_directory = filesystem[current_directory].parent.unwrap(),
                Some("/") => current_directory = ROOT_DIRECTORY,
                _ => {
                    let new_directory =
                        Directory::new(filesystem.len(), 0, Some(current_directory));
                    let new_idx = new_directory.idx;
                    filesystem[current_directory].children.push(new_idx);
                    filesystem.push(new_directory);
                    current_directory = new_idx;
                }
            }
        }
    }
    filesystem
}

fn get_file_size_from_line(line: &str) -> Option<usize> {
    line.split_whitespace().next()?.parse::<usize>().ok()
}

fn calculate_directory_total_size(filesystem: &[Directory<usize>], directory: usize) -> usize {
    return filesystem[directory].size
        + filesystem[directory]
            .children
            .iter()
            .map(|child| calculate_directory_total_size(filesystem, *child))
            .sum::<usize>();
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
