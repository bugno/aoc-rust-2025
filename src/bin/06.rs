advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    // Collect lines, keep spaces, then drop trailing blank/whitespace-only lines
    let mut lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    while lines.last().map_or(false, |l| l.trim().is_empty()) {
        lines.pop();
    }

    if lines.is_empty() {
        return Some(0);
    }

    let h = lines.len();
    let w = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Build grid padded with spaces
    let mut grid: Vec<Vec<char>> = vec![vec![' '; w]; h];
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            grid[r][c] = ch;
        }
    }

    // Determine which columns are separators (all spaces)
    let mut is_separator = vec![true; w];
    for c in 0..w {
        for row in &grid {
            if row[c] != ' ' {
                is_separator[c] = false;
                break;
            }
        }
    }

    // Find contiguous non-separator column ranges = problems
    let mut segments: Vec<(usize, usize)> = Vec::new();
    let mut c = 0;
    while c < w {
        // Skip separator columns
        if is_separator[c] {
            c += 1;
            continue;
        }
        let start = c;
        let mut end = c;
        c += 1;
        while c < w && !is_separator[c] {
            end = c;
            c += 1;
        }
        segments.push((start, end));
    }

    let mut total: i128 = 0;

    for (start, end) in segments {
        // Find operator in last row
        let op_row = h - 1;
        let mut op: Option<char> = None;
        for col in start..=end {
            let ch = grid[op_row][col];
            if ch == '+' || ch == '*' {
                op = Some(ch);
                break;
            }
        }
        let op = op.expect("operator not found in problem segment");

        // Collect numbers in rows above the operator row
        let mut nums: Vec<i128> = Vec::new();
        for r in 0..op_row {
            let mut s = String::new();
            for col in start..=end {
                s.push(grid[r][col]);
            }
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                let val: i128 = trimmed.parse().expect("invalid integer in problem");
                nums.push(val);
            }
        }

        assert!(!nums.is_empty(), "problem segment without numbers");

        // Compute this problem's result
        let result = match op {
            '+' => nums.into_iter().sum::<i128>(),
            '*' => nums.into_iter().fold(1_i128, |acc, x| acc * x),
            _ => unreachable!(),
        };

        total += result;
    }

    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Collect lines and drop trailing whitespace-only lines
    let mut lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    while lines.last().map_or(false, |l| l.trim().is_empty()) {
        lines.pop();
    }

    if lines.is_empty() {
        return Some(0);
    }

    let h = lines.len();
    let w = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Build grid padded with spaces
    let mut grid: Vec<Vec<char>> = vec![vec![' '; w]; h];
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            grid[r][c] = ch;
        }
    }

    // Separator columns: all spaces
    let mut is_separator = vec![true; w];
    for c in 0..w {
        for row in &grid {
            if row[c] != ' ' {
                is_separator[c] = false;
                break;
            }
        }
    }

    // Find contiguous non-separator column ranges (problems)
    let mut segments: Vec<(usize, usize)> = Vec::new();
    let mut c = 0;
    while c < w {
        if is_separator[c] {
            c += 1;
            continue;
        }
        let start = c;
        let mut end = c;
        c += 1;
        while c < w && !is_separator[c] {
            end = c;
            c += 1;
        }
        segments.push((start, end));
    }

    let op_row = h - 1;
    let mut total: i128 = 0;

    for (start, end) in segments {
        // Find operator somewhere in bottom row of this segment
        let mut op: Option<char> = None;
        for col in start..=end {
            let ch = grid[op_row][col];
            if ch == '+' || ch == '*' {
                op = Some(ch);
                break;
            }
        }
        let op = op.expect("no operator found in problem segment");

        // IMPORTANT: every column in the segment (including the operator column)
        // can be a number column. We only ignore non-digit cells above.
        let mut nums: Vec<i128> = Vec::new();

        for col in start..=end {
            let mut s = String::new();
            for row in 0..op_row {
                let ch = grid[row][col];
                if ch.is_ascii_digit() {
                    s.push(ch);
                }
            }

            if !s.is_empty() {
                let val: i128 = s.parse().expect("invalid integer in column");
                nums.push(val);
            }
        }

        assert!(!nums.is_empty(), "problem segment without numbers");

        let result = match op {
            '+' => nums.into_iter().sum::<i128>(),
            '*' => nums.into_iter().fold(1_i128, |acc, x| acc * x),
            _ => unreachable!(),
        };

        total += result;
    }

    Some(total as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
