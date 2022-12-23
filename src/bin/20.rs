use std::ops::Rem;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<isize> {
    let initial_list = input
        .lines()
        .enumerate()
        .map(|(idx, num)| (idx, num.parse::<isize>().unwrap()))
        .collect_vec();

    let mut updated_list = initial_list.clone();

    for (initial_idx, num) in &initial_list {
        let ul_clone = updated_list.clone();
        let ul_idx = ul_clone
            .iter()
            .find_position(|(ul_idx, _)| ul_idx == initial_idx)
            .unwrap();

        // &updated_list.get(ul_idx.0).unwrap();
        if *num >= 0 {
            // println!("Num: {} | UL_Idx: {:?}", num, ul_idx);
            updated_list.remove(ul_idx.0);
            updated_list.insert((ul_idx.0 + *num as usize) % (ul_clone.len() - 1), *ul_idx.1);
        } else {
            let idx =
                ((ul_idx.0 as isize + num).rem_euclid((ul_clone.len() - 1) as isize)) as usize;
            // println!("Num: {} | UL_Idx: {:?} | Idx: {}", num, ul_idx, idx);
            updated_list.remove(ul_idx.0);
            updated_list.insert(idx, *ul_idx.1);
        }

        // println!("{:?}", updated_list);
    }

    let zero_pos = updated_list
        .iter()
        .find_position(|(_, elem)| *elem == 0)
        .unwrap();
    // println!(
    //     "{:?} -> {:?} -> {:?} -> {:?}",
    //     zero_pos,
    //     (zero_pos.0 + 1000) % &initial_list.len(),
    //     (zero_pos.0 + 2000) % initial_list.len(),
    //     (zero_pos.0 + 3000) % initial_list.len()
    // );
    Some(
        [
            updated_list
                .get((zero_pos.0 + 1000) % &initial_list.len())
                .unwrap()
                .1,
            updated_list
                .get((zero_pos.0 + 2000) % initial_list.len())
                .unwrap()
                .1,
            updated_list
                .get((zero_pos.0 + 3000) % initial_list.len())
                .unwrap()
                .1,
        ]
        .into_iter()
        .sum::<isize>(),
    )
}

pub fn part_two(input: &str) -> Option<isize> {
    let initial_list = input
        .lines()
        .enumerate()
        .map(|(idx, num)| (idx, num.parse::<isize>().unwrap()))
        .map(|(idx, num)| (idx, num * 811589153))
        .collect_vec();

    let mut updated_list = initial_list.clone();

    for _ in 0..10 {
        for (initial_idx, num) in &initial_list {
            let ul_clone = updated_list.clone();
            let ul_idx = ul_clone
                .iter()
                .find_position(|(ul_idx, _)| ul_idx == initial_idx)
                .unwrap();

            // &updated_list.get(ul_idx.0).unwrap();
            if *num >= 0 {
                // println!("Num: {} | UL_Idx: {:?}", num, ul_idx);
                updated_list.remove(ul_idx.0);
                updated_list.insert((ul_idx.0 + *num as usize) % (ul_clone.len() - 1), *ul_idx.1);
            } else {
                let idx =
                    ((ul_idx.0 as isize + num).rem_euclid((ul_clone.len() - 1) as isize)) as usize;
                // println!("Num: {} | UL_Idx: {:?} | Idx: {}", num, ul_idx, idx);
                updated_list.remove(ul_idx.0);
                updated_list.insert(idx, *ul_idx.1);
            }

            // println!("{:?}", updated_list);
        }
    }

    let zero_pos = updated_list
        .iter()
        .find_position(|(_, elem)| *elem == 0)
        .unwrap();
    // println!(
    //     "{:?} -> {:?} -> {:?} -> {:?}",
    //     zero_pos,
    //     (zero_pos.0 + 1000) % &initial_list.len(),
    //     (zero_pos.0 + 2000) % initial_list.len(),
    //     (zero_pos.0 + 3000) % initial_list.len()
    // );
    Some(
        [
            updated_list
                .get((zero_pos.0 + 1000) % &initial_list.len())
                .unwrap()
                .1,
            updated_list
                .get((zero_pos.0 + 2000) % initial_list.len())
                .unwrap()
                .1,
            updated_list
                .get((zero_pos.0 + 3000) % initial_list.len())
                .unwrap()
                .1,
        ]
        .into_iter()
        .sum::<isize>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
