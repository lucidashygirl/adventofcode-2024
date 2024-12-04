#![warn(clippy::pedantic, clippy::nursery)]
advent_of_code::solution!(4);

const REMAINING_CHARS: &[char] = &['M', 'A', 'S'];

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let input: Vec<&[char]> = input.iter().map(|x| x.as_slice()).collect();
    let count = find_xmas_in_matrix(&input);
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let input: Vec<&[char]> = input.iter().map(|x| x.as_slice()).collect();
    let count = find_x_mas_in_matrix(&input);
    Some(count)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let rows: Vec<&str> = input.split('\n').collect();
    let mut cols: Vec<Vec<char>> = Vec::new();
    for row in rows {
        let col = row.chars().collect::<Vec<char>>();
        if col.is_empty() {
            break;
        }
        cols.push(col);
    }
    cols
}

fn find_xmas_in_matrix(input: &[&[char]]) -> u32 {
    let mut count: u32 = 0;
    for (r, row) in input.iter().enumerate() {
        for (c, item) in row.iter().enumerate() {
            if item != &'X' {
                continue;
            }
            if check_right(row, c) {
                count += 1;
            }
            if check_left(row, c) {
                count += 1;
            }
            let col: Vec<char> = input.iter().map(|x| x[c]).collect();
            if check_up(&col, r) {
                count += 1;
            }
            if check_down(&col, r) {
                count += 1;
            }
            if check_se(input, r, c) {
                count += 1;
            }
            if check_sw(input, r, c) {
                count += 1;
            }
            if check_ne(input, r, c) {
                count += 1;
            }
            if check_nw(input, r, c) {
                count += 1;
            }
        }
    }
    count
}

fn check_right(row: &[char], index: usize) -> bool {
    if !(0..(row.len() - 3)).contains(&index) {
        return false;
    }
    for (i, c) in REMAINING_CHARS.iter().enumerate() {
        if row[index + i + 1] != *c {
            return false;
        }
    }

    true
}

fn check_left(row: &[char], index: usize) -> bool {
    if !(3..row.len()).contains(&index) {
        return false;
    }
    for (i, c) in REMAINING_CHARS.iter().enumerate() {
        if row[index - i - 1] != *c {
            return false;
        }
    }

    true
}

fn check_down(col: &[char], index: usize) -> bool {
    if !(0..(col.len() - 3)).contains(&index) {
        return false;
    }
    for (i, c) in REMAINING_CHARS.iter().enumerate() {
        if col[index + i + 1] != *c {
            return false;
        }
    }

    true
}

fn check_up(col: &[char], index: usize) -> bool {
    if !(3..col.len()).contains(&index) {
        return false;
    }
    for (i, c) in REMAINING_CHARS.iter().enumerate() {
        if col[index - i - 1] != *c {
            return false;
        }
    }

    true
}

fn check_se(table: &[&[char]], v_index: usize, h_index: usize) -> bool {
    if !(0..(table.len() - 3)).contains(&v_index) {
        return false;
    }
    if !(0..(table[v_index].len() - 3)).contains(&h_index) {
        return false;
    }
    for (i, c) in REMAINING_CHARS.iter().enumerate() {
        if table[v_index + i + 1][h_index + i + 1] != *c {
            return false;
        }
    }

    true
}

fn check_ne(table: &[&[char]], v_index: usize, h_index: usize) -> bool {
    if !(3..table.len()).contains(&v_index) {
        return false;
    }
    if !(0..(table[v_index].len() - 3)).contains(&h_index) {
        return false;
    }
    for (i, c) in REMAINING_CHARS.iter().enumerate() {
        if table[v_index - i - 1][h_index + i + 1] != *c {
            return false;
        }
    }

    true
}

fn check_sw(table: &[&[char]], v_index: usize, h_index: usize) -> bool {
    if !(0..(table.len() - 3)).contains(&v_index) {
        return false;
    }
    if !(3..table[v_index].len()).contains(&h_index) {
        return false;
    }
    for (i, c) in REMAINING_CHARS.iter().enumerate() {
        if table[v_index + i + 1][h_index - i - 1] != *c {
            return false;
        }
    }

    true
}

fn check_nw(table: &[&[char]], v_index: usize, h_index: usize) -> bool {
    if !(3..table.len()).contains(&v_index) {
        return false;
    }
    if !(3..table[v_index].len()).contains(&h_index) {
        return false;
    }
    for (i, c) in REMAINING_CHARS.iter().enumerate() {
        if table[v_index - i - 1][h_index - i - 1] != *c {
            return false;
        }
    }

    true
}

pub fn find_x_mas_in_matrix(input: &[&[char]]) -> u32 {
    let mut count: u32 = 0;
    for (r, row) in input.iter().enumerate() {
        for (c, item) in row.iter().enumerate() {
            if item != &'A' {
                continue;
            }
            if check_xmas_bottom(input, r, c) {
                count += 1;
            }
            if check_xmas_top(input, r, c) {
                count += 1;
            }
            if check_xmas_left(input, r, c) {
                count += 1;
            }
            if check_xmas_right(input, r, c) {
                count += 1;
            }
        }
    }
    count
}

pub fn check_xmas_bottom(table: &[&[char]], v_index: usize, h_index: usize) -> bool {
    if !(1..table.len() - 1).contains(&v_index) {
        return false;
    }
    if !(1..table[v_index].len() - 1).contains(&h_index) {
        return false;
    }

    if table[v_index - 1][h_index - 1] != 'M' {
        return false;
    }
    if table[v_index - 1][h_index + 1] != 'M' {
        return false;
    }
    if table[v_index + 1][h_index - 1] != 'S' {
        return false;
    }
    if table[v_index + 1][h_index + 1] != 'S' {
        return false;
    }
    true
}

pub fn check_xmas_top(table: &[&[char]], v_index: usize, h_index: usize) -> bool {
    if !(1..table.len() - 1).contains(&v_index) {
        return false;
    }
    if !(1..table[v_index].len() - 1).contains(&h_index) {
        return false;
    }

    if table[v_index - 1][h_index - 1] != 'S' {
        return false;
    }
    if table[v_index - 1][h_index + 1] != 'S' {
        return false;
    }
    if table[v_index + 1][h_index - 1] != 'M' {
        return false;
    }
    if table[v_index + 1][h_index + 1] != 'M' {
        return false;
    }
    true
}
pub fn check_xmas_left(table: &[&[char]], v_index: usize, h_index: usize) -> bool {
    if !(1..table.len() - 1).contains(&v_index) {
        return false;
    }
    if !(1..table[v_index].len() - 1).contains(&h_index) {
        return false;
    }

    if table[v_index - 1][h_index - 1] != 'M' {
        return false;
    }
    if table[v_index - 1][h_index + 1] != 'S' {
        return false;
    }
    if table[v_index + 1][h_index - 1] != 'M' {
        return false;
    }
    if table[v_index + 1][h_index + 1] != 'S' {
        return false;
    }
    true
}
pub fn check_xmas_right(table: &[&[char]], v_index: usize, h_index: usize) -> bool {
    if !(1..table.len() - 1).contains(&v_index) {
        return false;
    }
    if !(1..table[v_index].len() - 1).contains(&h_index) {
        return false;
    }

    if table[v_index - 1][h_index - 1] != 'S' {
        return false;
    }
    if table[v_index - 1][h_index + 1] != 'M' {
        return false;
    }
    if table[v_index + 1][h_index - 1] != 'S' {
        return false;
    }
    if table[v_index + 1][h_index + 1] != 'M' {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
