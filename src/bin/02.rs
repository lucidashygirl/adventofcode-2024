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
    let mut safe_reports: u32 = 0;
    let input_parsed = parse_input(input);
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

fn ascending(report: &[u32]) -> (u32, Option<usize>) {
    let mut errors = 0;
    let mut error: Option<usize> = None;
    for i in 1..report.len() {
        if report[i - 1] >= report[i] {
            if error.is_none() {
                error = Some(i - 1);
            }
            errors += 1;
        }
    }
    (errors, error)
}

fn descending(report: &[u32]) -> (u32, Option<usize>) {
    let mut errors = 0;
    let mut error: Option<usize> = None;
    for i in 1..report.len() {
        if report[i - 1] <= report[i] {
            if error.is_none() {
                error = Some(i - 1);
            }
            errors += 1;
        }
    }
    (errors, error)
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
                println!("{report:?}");
                return false;
            }
        }
        return true;
    }
    false
}

fn is_safe_damp(mut report: Vec<u32>) -> bool {
    /*
    let asc_err = ascending(&report);
    let des_err = descending(&report);


    let ascending = asc_err.0 <= 1;
    let descending = des_err.0 <= 1;
    */

    if report[0] > report[1] {
        report.reverse();
    }

    let mut errors = 0;
    for i in 1..report.len() {
        if report[i - 1] >= report[i] {
            errors += 1;
            continue;
        }

        let diff = report[i].abs_diff(report[i - 1]);
        let in_bounds = (1..=3).contains(&(diff));
        if !in_bounds {
            errors += 1;
        }
    }
    if errors > 1 {
        return false;
    }
    true
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
