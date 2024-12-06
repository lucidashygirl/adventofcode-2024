advent_of_code::solution!(5);
use multimap::MultiMap;
pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);

    let mut middle_pages: Vec<u32> = Vec::new();
    for keys in updates {
        if let Some(_) = rule_broken(&rules, &keys) {
            continue;
        }
        let index = ((keys.len() / 2) as f32).round() as usize;
        middle_pages.push(keys[index]);
    }
    Some(middle_pages.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);

    let mut middle_pages: Vec<u32> = Vec::new();
    for keys in updates {
        if let None = rule_broken(&rules, &keys) {
            continue;
        }
        let new_keys = sort_keys(&rules, keys);
        let index = ((new_keys.len() / 2) as f32).round() as usize;
        middle_pages.push(new_keys[index]);
    }
    Some(middle_pages.iter().sum())
}

fn parse_input(input: &str) -> (MultiMap<u32, u32>, Vec<Vec<u32>>) {
    let input: Vec<&str> = input.split("\n\n").collect();
    let rules: Vec<&str> = input[0].split('\n').filter(|x| !x.is_empty()).collect();
    let mut rule: MultiMap<u32, u32> = MultiMap::new();
    for i in rules {
        let split: Vec<u32> = i.split('|').map(|x| x.parse::<u32>().unwrap()).collect();
        rule.insert(split[0], split[1]);
    }
    let updates: Vec<Vec<u32>> = input[1]
        .split('\n')
        .map(|x| {
            x.split(',')
                .map(|y| y.parse::<u32>())
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|x| !x.is_empty())
        .collect();

    (rule, updates)
}

fn rule_broken(rules: &MultiMap<u32, u32>, keys: &[u32]) -> Option<usize> {
    let mut current_keys: Vec<u32> = Vec::new();
    for key in keys {
        current_keys.push(*key);
        if let Some(value) = rules.get_vec(key) {
            for val in value {
                let troublemaker = current_keys.iter().position(|x| x == val);
                if let Some(t) = troublemaker {
                    return Some(t);
                }
            }
        }
    }
    None
}

fn sort_keys(rules: &MultiMap<u32, u32>, mut keys: Vec<u32>) -> Vec<u32> {
    loop {
        if let Some(i) = rule_broken(&rules, &keys) {
            let value = keys[i];
            keys.remove(i);
            keys.push(value);
        } else {
            return keys;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
