pub fn find_x_mas_in_matrix(input: &[&[char]]) -> u32 {
    let mut count: u32 = 0;
    for (r, row) in input.iter().enumerate() {
        for (c, item) in row.iter().enumerate() {
            if item != &'A' {
                continue;
            }
            if check_x_mas(input, r, c) {
                count += 1;
            }
        }
    }
    count
}

pub fn check_x_mas(table: &[&[char]]) {}
