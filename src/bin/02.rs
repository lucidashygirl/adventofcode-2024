advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let input_parsed = parse_input(input);
    let mut safe_reports: u32 = 0;
    for report in input_parsed {
        if is_safe(&report) {
            safe_reports += 1;
        }
    }
    if safe_reports == 0 {
        return None;
    }
    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_parsed = parse_input(input);
    let mut safe_reports: u32 = 0;
    for report in input_parsed {
        if is_safe_damp(report) {
            safe_reports += 1;
        }
    }
    if safe_reports == 0 {
        return None;
    }
    Some(safe_reports)
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let input: Vec<&str> = input.split('\n').collect();
    let mut input_parsed: Vec<Vec<u32>> = Vec::new();
    let mut input_split: Vec<Vec<&str>> = Vec::new();
    for i in input {
        input_split.push(i.split(' ').collect::<Vec<&str>>());
    }
    for i in input_split {
        let mut arr: Vec<u32> = Vec::new();
        for x in i {
            if let Ok(p) = x.parse::<u32>() {
                arr.push(p);
            }
        }
        if !arr.is_empty() {
            input_parsed.push(arr);
        }
    }
    input_parsed
}

fn is_safe(report: &[u32]) -> bool {
    let ascending = report.is_sorted_by(|a, b| a < b);
    if ascending {
        for i in 1..report.len() {
            let diff = report[i] - report[i - 1];
            let in_bounds = (1..=3).contains(&(diff));
            if !in_bounds {
                return false;
            }
        }
        return true;
    }
    let descending = report.is_sorted_by(|a, b| a > b);
    if descending {
        for i in 1..report.len() {
            let diff = report[i - 1] - report[i];
            let in_bounds = (1..=3).contains(&(diff));
            if !in_bounds {
                return false;
            }
        }
        return true;
    }
    false
}

fn is_safe_damp(mut report: Vec<u32>) -> bool {
    let ascending = report.is_sorted_by(|a, b| a < b);
    let descending = report.is_sorted_by(|a, b| a > b);
    let mut has_errored = false;
    if descending {
        report.reverse();
    }
    if !ascending && !descending {
        has_errored = true;
        let brute_force = brute_force_is_ascending(&report);
        report = brute_force.1;
        match brute_force.0 {
            Some(true) => (),
            Some(false) => report.reverse(),
            None => return false,
        }
    }
    if !has_errored {
        for i in 1..report.len() {
            if report[i - 1] > report[i] {
                has_errored = true;
                report.remove(i);
                break;
            }

            let diff = report[i] - report[i - 1];
            let in_bounds = (1..=3).contains(&(diff));
            if !in_bounds {
                has_errored = true;
                report.remove(i);
                break;
            }
        }
    }
    if has_errored {
        for i in 1..report.len() {
            if report[i - 1] > report[i] {
                return false;
            }

            let diff = report[i] - report[i - 1];
            let in_bounds = (1..=3).contains(&(diff));
            if !in_bounds {
                return false;
            }
        }
    }
    true
}

fn brute_force_is_ascending(report: &[u32]) -> (Option<bool>, Vec<u32>) {
    for i in 0..report.len() {
        let mut truncated_report = report.to_vec();
        truncated_report.remove(i);
        let ascending = truncated_report.is_sorted_by(|a, b| a < b);
        let descending = truncated_report.is_sorted_by(|a, b| a > b);
        if ascending {
            return (Some(true), truncated_report);
        }
        if descending {
            return (Some(false), truncated_report);
        }
    }
    (None, Vec::new())
}
/*
fn dampen(mut report: Vec<u32>) -> Vec<u32> {
    let index = maybe_unordered(&report);
    if let Some(e) = index {
        report.remove(e);
        return report;
    }
    for i in 0..(report.len() - 1) {
        let arg = report[i] as i32 - report[i + 1] as i32;
        let arg = match arg {
            0.. => arg,
            _ => -arg,
        };
        let in_bounds = (1..=3).contains(&(arg));
        if !in_bounds {
            report.remove(i + 1);
            return report;
        }
    }
    report
}

fn maybe_unordered(report: &[u32]) -> Option<usize> {
    if report[0] < report[1] {
        return ascending(&report);
    }
    if report[0] > report[1] {
        return descending(&report);
    }
    None
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
