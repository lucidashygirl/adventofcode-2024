advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_best_combos(parse_input(input)))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> Vec<Input> {
    let mut output: Vec<Input> = Vec::new();
    let input: Vec<&str> = input.split('\n').filter(|x| !x.is_empty()).collect();
    for i in input {
        let split: Vec<&str> = i.split(':').collect();
        let result: u64 = split[0].parse::<u64>().unwrap();
        let products: Vec<u32> = split[1]
            .split_whitespace()
            .map(|x| x.parse::<u32>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();

        output.push(Input { result, products });
    }
    output
}

use rayon::prelude::*;
fn find_best_combos(input: Vec<Input>) -> u32 {
    let combos: Vec<u32> = Vec::new();
    for i in input {
        let mut states: Vec<Operators> = Vec::new();
    }

    combos.par_iter().sum()
}

#[derive(Debug)]
struct Input {
    result: u64,
    products: Vec<u32>,
}

enum Operators {
    Sum,
    Product,
}

impl Input {
    fn new() -> Self {
        Input {
            result: 0,
            products: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
