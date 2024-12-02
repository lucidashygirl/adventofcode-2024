#![warn(clippy::nursery, clippy::pedantic)]
advent_of_code::solution!(1);

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);

    let mut parsed_inputs: Vec<(u32, u32)> = Vec::new();
    for i in 0..input.0.len() {
        parsed_inputs.push((input.0[i], input.1[i]));
    }

    let mut distances: Vec<u32> = Vec::new();

    for inputs in parsed_inputs {
        println!("{inputs:?}");
        let (l, r) = inputs;
        let dist = if l > r { l - r } else { r - l };
        distances.push(dist);
    }

    let total = distances.iter().sum::<u32>();
    if total == 0 {
        return None;
    }

    Some(total)
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);

    let mut lengths: Vec<u32> = Vec::new();

    for left in &parsed_input.0 {
        let sim = parsed_input.1.iter().filter(|x| x == &left).count();
        let score = left * u32::try_from(sim).unwrap_or(0);
        lengths.push(score);
    }

    let score = lengths.iter().sum::<u32>();

    if score == 0 {
        return None;
    }

    Some(score)
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let all_inputs: Vec<&str> = input.split('\n').collect();
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for input in all_inputs {
        if input.is_empty() {
            break;
        }
        let input: Vec<&str> = input.split("   ").collect();
        let left_parsed = input[0].parse::<u32>().unwrap();
        let right_parsed = input[1].parse::<u32>().unwrap();
        left.push(left_parsed);
        right.push(right_parsed);
    }

    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
