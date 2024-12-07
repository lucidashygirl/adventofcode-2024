advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let steps = count_steps(input);
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let placements = count_plausible_placements(input);
    Some(placements)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect()
}

fn count_steps(mut input: Vec<Vec<char>>) -> u32 {
    let mut guard = get_guard_indicies(&input);
    let mut count = 1;
    loop {
        if guard.is_none() || !guard.in_bounds(&input) {
            return count;
        }
        let facing = guard.facing(&input);
        if facing == Tile::Block {
            guard = guard.rotate();
            continue;
        }
        if facing == Tile::Empty {
            count += 1;
        }
        guard = guard.move_forward_and_mark(&mut input);
    }
}

fn count_plausible_placements(input: Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let mut guard = get_guard_indicies(&input);
            let mut input_local = input.clone();
            let mut guard_positions: Vec<Guard> = Vec::new();
            input_local[y][x] = '#';
            loop {
                guard_positions.push(guard.clone());
                if guard.is_none() || !guard.in_bounds(&input_local) {
                    break;
                }
                let facing = guard.facing(&input_local);
                if facing == Tile::Block {
                    guard = guard.rotate();
                    continue;
                }
                guard = guard.move_forward();
                if !has_unique_elements(&guard_positions) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

use std::collections::HashSet;
use std::hash::Hash;
fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn get_guard_indicies(input: &[Vec<char>]) -> Guard {
    for i in 0..input.len() {
        if let Some(j) = input[i].iter().position(|x| x == &'^') {
            return Guard::North(j, i);
        }
        if let Some(j) = input[i].iter().position(|x| x == &'V') {
            return Guard::South(j, i);
        }
        if let Some(j) = input[i].iter().position(|x| x == &'>') {
            return Guard::East(j, i);
        }
        if let Some(j) = input[i].iter().position(|x| x == &'<') {
            return Guard::West(j, i);
        }
    }
    Guard::None
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum Guard {
    North(usize, usize),
    South(usize, usize),
    East(usize, usize),
    West(usize, usize),
    None,
}

impl Guard {
    fn is_none(&self) -> bool {
        matches!(*self, Guard::None)
    }

    fn facing(&self, map: &[Vec<char>]) -> Tile {
        match *self {
            Guard::North(x, y) => {
                if y as i32 - 1 < 0 {
                    return Tile::Empty;
                }
                match map[y - 1][x] {
                    '#' => Tile::Block,
                    'X' => Tile::Stepped,
                    _ => Tile::Empty,
                }
            }
            Guard::South(x, y) => {
                if y + 1 > map.len() {
                    return Tile::Empty;
                }
                match map[y + 1][x] {
                    '#' => Tile::Block,
                    'X' => Tile::Stepped,
                    _ => Tile::Empty,
                }
            }
            Guard::East(x, y) => {
                if x + 1 > map.len() {
                    return Tile::Empty;
                }
                match map[y][x + 1] {
                    '#' => Tile::Block,
                    'X' => Tile::Stepped,
                    _ => Tile::Empty,
                }
            }
            Guard::West(x, y) => {
                if x as i32 - 1 < 0 {
                    return Tile::Empty;
                }
                match map[y][x - 1] {
                    '#' => Tile::Block,
                    'X' => Tile::Stepped,
                    _ => Tile::Empty,
                }
            }
            Guard::None => Tile::Empty,
        }
    }

    fn rotate(&self) -> Self {
        match *self {
            Guard::North(x, y) => Guard::East(x, y),
            Guard::East(x, y) => Guard::South(x, y),
            Guard::South(x, y) => Guard::West(x, y),
            Guard::West(x, y) => Guard::North(x, y),
            Guard::None => Guard::None,
        }
    }

    fn move_forward_and_mark(&self, map: &mut Vec<Vec<char>>) -> Self {
        match *self {
            Guard::North(x, y) => {
                map[y][x] = 'X';
                Guard::North(x, y - 1)
            }
            Guard::East(x, y) => {
                map[y][x] = 'X';
                Guard::East(x + 1, y)
            }
            Guard::South(x, y) => {
                map[y][x] = 'X';
                Guard::South(x, y + 1)
            }
            Guard::West(x, y) => {
                map[y][x] = 'X';
                println!("{x}, {y}");
                Guard::West(x - 1, y)
            }
            Guard::None => Guard::None,
        }
    }

    fn move_forward(&self) -> Self {
        match *self {
            Guard::North(x, y) => Guard::North(x, y - 1),
            Guard::East(x, y) => Guard::East(x + 1, y),
            Guard::South(x, y) => Guard::South(x, y + 1),
            Guard::West(x, y) => Guard::West(x - 1, y),
            Guard::None => Guard::None,
        }
    }

    fn in_bounds(&self, map: &[Vec<char>]) -> bool {
        let guard_ind = match *self {
            Guard::North(x, y) => (x, y),
            Guard::South(x, y) => (x, y),
            Guard::East(x, y) => (x, y),
            Guard::West(x, y) => (x, y),
            Guard::None => return false,
        };
        if !(1..map[0].len() - 1).contains(&guard_ind.0) {
            return false;
        }
        if !(1..map.len() - 1).contains(&guard_ind.1) {
            return false;
        }
        true
    }

    fn unwrap(&self) -> Self {
        match *self {
            Guard::North(x, y) => Guard::North(x, y),
            Guard::East(x, y) => Guard::East(x, y),
            Guard::South(x, y) => Guard::South(x, y),
            Guard::West(x, y) => Guard::West(x, y),
            Guard::None => panic!("Cant unwrap None"),
        }
    }
}

#[derive(PartialEq)]
enum Tile {
    Block,
    Stepped,
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
