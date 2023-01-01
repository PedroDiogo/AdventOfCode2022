use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Position = (isize, isize);

trait Rotate {
    fn rotate90(&self) -> Self;
}

impl Rotate for Position {
    fn rotate90(&self) -> Self {
        (-self.1, self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn turn(&self, c: &char) -> Self {
        match (self, c) {
            (Facing::Down, 'L') => Facing::Right,
            (Facing::Down, 'R') => Facing::Left,
            (Facing::Right, 'L') => Facing::Up,
            (Facing::Right, 'R') => Facing::Down,
            (Facing::Up, 'L') => Facing::Left,
            (Facing::Up, 'R') => Facing::Right,
            (Facing::Left, 'L') => Facing::Down,
            (Facing::Left, 'R') => Facing::Up,
            _ => unimplemented!(),
        }
    }

    fn to_number(&self) -> isize {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, EnumIter)]
enum CubeFace {
    Top,
    Front,
    Back,
    Left,
    Right,
    Bottom,
}

fn next_position(
    current_position: &Position,
    facing: &Facing,
    map: &HashMap<Position, char>,
) -> Option<Position> {
    let delta: (isize, isize) = match facing {
        Facing::Right => (1, 0),
        Facing::Down => (0, 1),
        Facing::Left => (-1, 0),
        Facing::Up => (0, -1),
    };

    let mut new_position = (current_position.0 + delta.0, current_position.1 + delta.1);

    loop {
        match map.get(&new_position) {
            Some('.') => return Some(new_position),
            Some('#') => return Some(*current_position),
            None => new_position = find_opposite(current_position, facing, map),
            _ => unimplemented!(),
        }
    }
}

fn find_opposite(
    current_position: &Position,
    facing: &Facing,
    map: &HashMap<Position, char>,
) -> Position {
    let max = (
        map.keys().max_by_key(|p| p.0).unwrap().0,
        map.keys().max_by_key(|p| p.1).unwrap().1,
    );
    let delta: (isize, isize) = match facing {
        Facing::Right => (1, 0),
        Facing::Down => (0, 1),
        Facing::Left => (-1, 0),
        Facing::Up => (0, -1),
    };
    let mut position = match facing {
        Facing::Right => (0, current_position.1),
        Facing::Down => (current_position.0, 0),
        Facing::Left => (max.0, current_position.1),
        Facing::Up => (current_position.0, max.1),
    };

    loop {
        if let Some(_) = map.get(&position) {
            return position;
        } else {
            position = (position.0 + delta.0, position.1 + delta.1);
        }
    }
}
fn next_position_2(
    face: &CubeFace,
    current_position: &Position,
    facing: &Facing,
    map: &HashMap<CubeFace, HashMap<Position, char>>,
    cube_side_len: &isize,
) -> Option<(CubeFace, Position, Facing)> {
    let delta: (isize, isize) = match facing {
        Facing::Right => (1, 0),
        Facing::Down => (0, 1),
        Facing::Left => (-1, 0),
        Facing::Up => (0, -1),
    };

    let mut new_position = (current_position.0 + delta.0, current_position.1 + delta.1);
    let mut new_face = *face;
    let mut new_facing = *facing;

    loop {
        match map.get(&new_face)?.get(&new_position) {
            Some('.') => return Some((new_face, new_position, new_facing)),
            Some('#') => return Some((*face, *current_position, *facing)),
            None => {
                let a = find_next_face_pos(face, current_position, facing, cube_side_len).unwrap();
                new_face = a.0;
                new_position = a.1;
                new_facing = a.2;
            }
            _ => unimplemented!(),
        }
    }
}

fn find_next_face_pos(
    face: &CubeFace,
    current_position: &Position,
    facing: &Facing,
    cube_side_len: &isize,
) -> Option<(CubeFace, Position, Facing)> {
    let cube_side_len = cube_side_len - 1;
    let result = match (face, facing) {
        (CubeFace::Top, Facing::Down) => (CubeFace::Front, (current_position.0, 0), Facing::Down),
        (CubeFace::Top, Facing::Up) => (
            CubeFace::Back,
            (cube_side_len - current_position.0, 0),
            Facing::Down,
        ),
        (CubeFace::Top, Facing::Left) => (CubeFace::Left, (current_position.1, 0), Facing::Down),
        (CubeFace::Top, Facing::Right) => (
            CubeFace::Right,
            (cube_side_len - current_position.1, 0),
            Facing::Down,
        ),
        (CubeFace::Front, Facing::Down) => {
            (CubeFace::Bottom, (current_position.0, 0), Facing::Down)
        }
        (CubeFace::Front, Facing::Up) => (
            CubeFace::Top,
            (current_position.0, cube_side_len),
            Facing::Up,
        ),
        (CubeFace::Front, Facing::Left) => (
            CubeFace::Left,
            (cube_side_len, current_position.1),
            Facing::Left,
        ),
        (CubeFace::Front, Facing::Right) => {
            (CubeFace::Right, (0, current_position.1), Facing::Right)
        }
        (CubeFace::Right, Facing::Down) => (
            CubeFace::Bottom,
            (cube_side_len, current_position.0),
            Facing::Left,
        ),
        (CubeFace::Right, Facing::Up) => (
            CubeFace::Top,
            (cube_side_len, cube_side_len - current_position.0),
            Facing::Left,
        ),
        (CubeFace::Right, Facing::Left) => (
            CubeFace::Front,
            (cube_side_len, current_position.1),
            Facing::Left,
        ),
        (CubeFace::Right, Facing::Right) => {
            (CubeFace::Back, (0, current_position.1), Facing::Right)
        }
        (CubeFace::Bottom, Facing::Down) => (
            CubeFace::Back,
            (cube_side_len - current_position.0, cube_side_len),
            Facing::Up,
        ),
        (CubeFace::Bottom, Facing::Up) => (
            CubeFace::Front,
            (current_position.0, cube_side_len),
            Facing::Up,
        ),
        (CubeFace::Bottom, Facing::Left) => (
            CubeFace::Left,
            (cube_side_len - current_position.1, cube_side_len),
            Facing::Up,
        ),
        (CubeFace::Bottom, Facing::Right) => (
            CubeFace::Right,
            (current_position.1, cube_side_len),
            Facing::Up,
        ),
        (CubeFace::Back, Facing::Down) => (
            CubeFace::Bottom,
            (cube_side_len - current_position.0, cube_side_len),
            Facing::Up,
        ),
        (CubeFace::Back, Facing::Up) => (
            CubeFace::Top,
            (cube_side_len - current_position.0, 0),
            Facing::Down,
        ),
        (CubeFace::Back, Facing::Left) => (
            CubeFace::Right,
            (cube_side_len, current_position.1),
            Facing::Left,
        ),
        (CubeFace::Back, Facing::Right) => (CubeFace::Left, (0, current_position.1), Facing::Right),

        (CubeFace::Left, Facing::Down) => (
            CubeFace::Bottom,
            (0, cube_side_len - current_position.0),
            Facing::Right,
        ),
        (CubeFace::Left, Facing::Up) => (CubeFace::Top, (0, current_position.0), Facing::Right),
        (CubeFace::Left, Facing::Left) => (
            CubeFace::Back,
            (cube_side_len, current_position.1),
            Facing::Left,
        ),
        (CubeFace::Left, Facing::Right) => {
            (CubeFace::Front, (0, current_position.1), Facing::Right)
        }
        _ => unimplemented!(),
    };
    // println!(
    //     "| Current Position: {:?} | Current: ({:?},{:?}) | New: {:?}",
    //     current_position, face, facing, result
    // );
    Some(result)
}

fn cube_side_len(whole_map: &str) -> isize {
    let mut cube_side_len = isize::MAX;
    for line in whole_map.lines() {
        cube_side_len = cube_side_len.min(line.trim().len() as isize);
    }
    cube_side_len
}

fn map_cube(whole_map: &str) -> HashMap<CubeFace, HashMap<Position, char>> {
    // let mut map = HashMap::new();
    let cube_side_len = cube_side_len(whole_map);
    let faces = map_faces(whole_map);
    println!("Faces: {:?}", faces);

    let mut map = HashMap::new();
    let mapping = map_face_to_cubeface(&faces);
    for (y, line) in whole_map.lines().enumerate() {
        for (x, c) in line.char_indices() {
            let (x, y) = (x as isize, y as isize);
            let face = (x / cube_side_len, y / cube_side_len);
            if c == '.' || c == '#' {
                let mapping = mapping.get(&face).unwrap();
                let (face, angle) = (mapping.0, mapping.1);
                let initial_pos = (x % cube_side_len, y % cube_side_len);
                let mut pos = initial_pos;
                for _ in 0..(mapping.1 / 90) {
                    pos = pos.rotate90();
                }
                let offset = match mapping.1 % 360 {
                    90 => (cube_side_len - 1, 0),
                    180 => (cube_side_len - 1, cube_side_len - 1),
                    270 => (0, cube_side_len - 1),
                    _ => (0, 0),
                };
                // println!(
                //     "Offset: {:?}, angle: {:?}, Initial: {:?}, Pos: {:?}",
                //     offset, angle, initial_pos, pos
                // );
                let pos = (pos.0 + offset.0, pos.1 + offset.1);
                map.entry(face)
                    .and_modify(|f: &mut HashMap<(isize, isize), char>| {
                        f.insert(pos, c);
                    })
                    .or_insert(HashMap::from([(pos, c)]));
            }
        }
    }

    map
}

fn map_faces(whole_map: &str) -> HashSet<Position> {
    let cube_side_len = cube_side_len(whole_map);
    let mut faces = HashSet::new();
    for (y, line) in whole_map.lines().enumerate() {
        for (x, c) in line.char_indices() {
            let (x, y) = (x as isize, y as isize);
            let face = (x / cube_side_len, y / cube_side_len);
            if c == '.' || c == '#' {
                faces.insert(face);
            }
        }
    }
    faces
}

fn map_face_to_cubeface(faces: &HashSet<Position>) -> HashMap<(isize, isize), (CubeFace, isize)> {
    let top_face = faces
        .iter()
        .min_by(|x, y| match x.1.cmp(&y.1) {
            std::cmp::Ordering::Equal => x.0.cmp(&y.0),
            other => other,
        })
        .unwrap();

    let mut visited = HashMap::new();
    let mut to_visit = vec![(*top_face, CubeFace::Top, 0)];
    // Create queue. Consume from queue until empty or all faces set
    while !to_visit.is_empty() {
        let mut value = to_visit.pop().unwrap();
        if (value.1 == CubeFace::Back) {
            // value = (value.0, value.1, 180);
        }
        let neighbours = neighbours_face(&value.1, &value.0, &faces);
        println!("v: {:?} -> n: {:?}", value, neighbours);
        visited.insert(value.0, (value.1, value.2));
        for neighbour in neighbours {
            if !visited.contains_key(&neighbour.0) {
                to_visit.push((neighbour.0, neighbour.1, neighbour.2 + value.2));
            }
        }
    }
    visited
}

fn neighbours_face(
    face: &CubeFace,
    position: &Position,
    faces: &HashSet<Position>,
) -> Vec<(Position, CubeFace, isize)> {
    let mut neighbours = vec![];
    let deltas = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
    for delta in deltas {
        let neighbour_pos = (position.0 + delta.0, position.1 + delta.1);

        if faces.contains(&neighbour_pos) {
            let a = match (face, delta) {
                (CubeFace::Top, (0, 1)) => (CubeFace::Front, 0),
                (CubeFace::Top, (1, 0)) => (CubeFace::Right, 90),
                (CubeFace::Front, (0, 1)) => (CubeFace::Bottom, 0),
                // (CubeFace::Front, (1, 0)) => (CubeFace::Right, 0),
                (CubeFace::Front, (0, -1)) => (CubeFace::Top, 0),
                (CubeFace::Front, (-1, 0)) => (CubeFace::Left, 0),
                (CubeFace::Left, (0, 1)) => (CubeFace::Back, 0),
                (CubeFace::Left, (1, 0)) => (CubeFace::Front, 0),
                // (CubeFace::Left, (0, -1)) => (CubeFace::Top, 0),
                (CubeFace::Left, (-1, 0)) => (CubeFace::Back, 0),
                (CubeFace::Back, (0, 1)) => (CubeFace::Bottom, 0),
                (CubeFace::Back, (1, 0)) => (CubeFace::Left, 0),
                (CubeFace::Back, (0, -1)) => (CubeFace::Top, 0),
                (CubeFace::Back, (-1, 0)) => (CubeFace::Right, 0),
                (CubeFace::Right, (0, 1)) => (CubeFace::Bottom, 0),
                (CubeFace::Right, (1, 0)) => (CubeFace::Front, 0),
                (CubeFace::Right, (0, -1)) => (CubeFace::Top, 0),
                (CubeFace::Right, (-1, 0)) => (CubeFace::Bottom, 0),
                (CubeFace::Bottom, (0, 1)) => (CubeFace::Back, 0),
                (CubeFace::Bottom, (1, 0)) => (CubeFace::Right, 270),
                (CubeFace::Bottom, (0, -1)) => (CubeFace::Front, 0),
                (CubeFace::Bottom, (-1, 0)) => (CubeFace::Left, 90),
                _ => unimplemented!(),
            };

            neighbours.push((neighbour_pos, a.0, a.1));
        }
    }
    neighbours
}

pub fn part_one(input: &str) -> Option<isize> {
    let (whole_map, directions) = match input.split("\n\n").collect_tuple() {
        Some((map, directions)) => (map, directions),
        _ => unimplemented!(),
    };

    let directions = directions
        .split_inclusive(['L', 'R'])
        .flat_map(|s| {
            let s = s.to_string();
            match s.parse::<usize>() {
                Result::Ok(_) => vec![s],
                _ => vec![s[0..s.len() - 1].to_string(), s[s.len() - 1..].to_string()],
            }
        })
        .collect_vec();

    println!("{:?}", directions);
    let mut start_position = (isize::MAX, isize::MAX);
    let mut map = HashMap::new();
    for (y, line) in whole_map.lines().enumerate() {
        for (x, c) in line.char_indices() {
            let (x, y) = (x as isize, y as isize);
            if c == '.' || c == '#' {
                map.insert((x, y), c);
                if x <= start_position.0 && y <= start_position.1 {
                    start_position = (x, y);
                }
            }
        }
    }
    println!("Start: {:?}", start_position);
    println!("map: {:?}", map);

    let mut pos = start_position;
    let mut facing = Facing::Right;

    for direction in directions {
        if direction == "L" || direction == "R" {
            facing = facing.turn(&direction.chars().next().unwrap());
        } else {
            for _ in 0..direction.parse().unwrap() {
                if let Some(new_pos) = next_position(&pos, &facing, &map) {
                    pos = new_pos;
                } else {
                    break;
                }
            }
        }
    }
    println!("Position: {:?}", pos);
    Some((pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + facing.to_number())
}

pub fn part_two(input: &str) -> Option<isize> {
    let (whole_map, directions) = match input.split("\n\n").collect_tuple() {
        Some((map, directions)) => (map, directions),
        _ => unimplemented!(),
    };

    let directions = directions
        .split_inclusive(['L', 'R'])
        .flat_map(|s| {
            let s = s.to_string();
            match s.parse::<usize>() {
                Result::Ok(_) => vec![s],
                _ => vec![s[0..s.len() - 1].to_string(), s[s.len() - 1..].to_string()],
            }
        })
        .collect_vec();

    let cube = map_cube(&whole_map);
    let cube_side_len = cube_side_len(whole_map);

    for face in CubeFace::iter() {
        let face_map = cube.get(&face).unwrap();

        println!("{:?}", face);
        for y in 0..cube_side_len {
            for x in 0..cube_side_len {
                match face_map.get(&(x, y)) {
                    Some(x) => print!("{}", x),
                    None => print!(" "),
                }
            }
            println!("");
        }
    }

    let mut face = CubeFace::Top;
    let mut pos = (0, 0);
    let mut facing = Facing::Right;

    for direction in directions {
        // println!("Direction: {:?}", direction);
        if direction == "L" || direction == "R" {
            facing = facing.turn(&direction.chars().next().unwrap());
            // println!(
            //     "Turn -> Face: {:?} | Position: {:?} | Facing: {:?}",
            //     face, pos, facing
            // );
        } else {
            for _ in 0..direction.parse().unwrap() {
                if let Some((new_face, new_pos, new_facing)) =
                    next_position_2(&face, &pos, &facing, &cube, &cube_side_len)
                {
                    face = new_face;
                    pos = new_pos;
                    facing = new_facing;
                } else {
                    break;
                }

                // println!(
                //     "Face: {:?} | Position: {:?} | Facing: {:?}",
                //     face, pos, facing
                // );
            }
        }
    }
    let faces = map_face_to_cubeface(&map_faces(whole_map));
    let face_opts = faces
        .iter()
        .find(|(key, value)| (*value).0 == face)
        .unwrap();

    println!("Position: {:?}", pos);
    println!("Face - {:?}: {:?}", face, face_opts);
    println!("Facing: {:?}", facing);
    for _ in 0..((360 - face_opts.1 .1) / 90) {
        pos = pos.rotate90();
        pos = (pos.0 + cube_side_len - 1, pos.1);
        facing = facing.turn(&'R');
    }
    println!("Position After Rotate: {:?}", pos);
    println!("Facing After Rotate: {:?}", facing);
    // TODO:
    let offset = face_opts.0;
    let pos = (
        pos.0 + offset.0 * cube_side_len,
        pos.1 + offset.1 * cube_side_len,
    );
    println!("Position After Offset: {:?}", pos);
    Some((pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + facing.to_number())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), Some(5031));
    }

    #[test]
    fn test_positions() {
        let all = [
            (CubeFace::Top, Facing::Up, (2, 0)),
            (CubeFace::Top, Facing::Down, (2, 9)),
            (CubeFace::Top, Facing::Left, (0, 2)),
            (CubeFace::Top, Facing::Right, (9, 2)),
            (CubeFace::Front, Facing::Up, (2, 0)),
            (CubeFace::Front, Facing::Down, (2, 9)),
            (CubeFace::Front, Facing::Left, (0, 2)),
            (CubeFace::Front, Facing::Right, (9, 2)),
            (CubeFace::Bottom, Facing::Up, (2, 0)),
            (CubeFace::Bottom, Facing::Down, (2, 9)),
            (CubeFace::Bottom, Facing::Left, (0, 2)),
            (CubeFace::Bottom, Facing::Right, (9, 2)),
            (CubeFace::Back, Facing::Up, (2, 0)),
            (CubeFace::Back, Facing::Down, (2, 9)),
            (CubeFace::Back, Facing::Left, (0, 2)),
            (CubeFace::Back, Facing::Right, (9, 2)),
            (CubeFace::Left, Facing::Up, (2, 0)),
            (CubeFace::Left, Facing::Down, (2, 9)),
            (CubeFace::Left, Facing::Left, (0, 2)),
            (CubeFace::Left, Facing::Right, (9, 2)),
            (CubeFace::Right, Facing::Up, (2, 0)),
            (CubeFace::Right, Facing::Down, (2, 9)),
            (CubeFace::Right, Facing::Left, (0, 2)),
            (CubeFace::Right, Facing::Right, (9, 2)),
        ];
        for p in all {
            let first = find_next_face_pos(&p.0, &p.2, &p.1, &10).unwrap();
            let second =
                find_next_face_pos(&first.0, &first.1, &first.2.turn(&'L').turn(&'L'), &10)
                    .unwrap();

            assert_eq!(p.0, second.0, "position: {:?}", p);
            assert_eq!(p.1, second.2.turn(&'L').turn(&'L'), "position: {:?}", p);
            assert_eq!(p.2, second.1, "position: {:?}", p);
        }
    }
}
