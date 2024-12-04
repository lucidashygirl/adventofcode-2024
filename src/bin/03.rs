advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let input_chars: Vec<char> = input.chars().collect();
    let mut mult_chars: Vec<String> = Vec::new();
    for i in 4..input_chars.len() {
        if input_chars[i - 4] != 'm' {
            continue;
        }
        if input_chars[i - 3] != 'u' {
            continue;
        }
        if input_chars[i - 2] != 'l' {
            continue;
        }
        if input_chars[i - 1] != '(' {
            continue;
        }
        let mut chars: Vec<char> = Vec::new();
        for j in 0..8 {
            let current = input_chars[i + j];
            if current == ')' {
                break;
            }
            chars.push(current);
        }
        mult_chars.push(chars.iter().collect());
    }
    let mut parsed_mult: Vec<u32> = Vec::new();
    for i in mult_chars {
        let split: Vec<&str> = i.split(',').collect();
        for j in 1..split.len() {
            if split.len() != 2 {
                continue;
            }
            if let (Ok(e), Ok(f)) = (split[j - 1].parse::<u32>(), split[j].parse::<u32>()) {
                parsed_mult.push(e * f);
            }
        }
    }
    Some(parsed_mult.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_chars: Vec<char> = input.chars().collect();
    let mut mult_chars: Vec<String> = Vec::new();
    let mut can_do = true;

    for i in 4..input_chars.len() {
        if input_chars[i - 4] == 'd' && input_chars[i - 3] == 'o' {
            match input_chars[i - 2] {
                '(' => can_do = true,
                'n' => can_do = false,
                _ => (),
            }
            continue;
        }
        if !can_do {
            continue;
        }
        if input_chars[i - 4] != 'm' {
            continue;
        }
        if input_chars[i - 3] != 'u' {
            continue;
        }
        if input_chars[i - 2] != 'l' {
            continue;
        }
        if input_chars[i - 1] != '(' {
            continue;
        }
        let mut chars: Vec<char> = Vec::new();
        for j in 0..8 {
            let current = input_chars[i + j];
            if current == ')' {
                break;
            }
            chars.push(current);
        }
        mult_chars.push(chars.iter().collect());
    }
    let mut parsed_mult: Vec<u32> = Vec::new();
    for i in mult_chars {
        let split: Vec<&str> = i.split(',').collect();
        for j in 1..split.len() {
            if split.len() != 2 {
                continue;
            }
            if let (Ok(e), Ok(f)) = (split[j - 1].parse::<u32>(), split[j].parse::<u32>()) {
                parsed_mult.push(e * f);
            }
        }
    }
    Some(parsed_mult.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
