advent_of_code::solution!(4);

fn count_accessible_rolls(input: &str) -> usize {
    // Parse grid into Vec<Vec<u8>>
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let h = grid.len();
    if h == 0 {
        return 0;
    }
    let w = grid[0].len();

    let directions: &[(isize, isize)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut accessible = 0usize;

    for r in 0..h {
        for c in 0..w {
            if grid[r][c] != b'@' {
                continue;
            }

            let mut neighbors = 0;

            for &(dr, dc) in directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr < 0 || nr >= h as isize || nc < 0 || nc >= w as isize {
                    continue;
                }

                if grid[nr as usize][nc as usize] == b'@' {
                    neighbors += 1;
                }
            }

            if neighbors < 4 {
                accessible += 1;
            }
        }
    }

    accessible
}

pub fn part_one(input: &str) -> Option<u64> {
    let ans = count_accessible_rolls(input);
    Some(ans as u64)
}

fn total_removed_rolls(input: &str) -> usize {
    // Parse grid into Vec<Vec<u8>>
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let h = grid.len();
    if h == 0 {
        return 0;
    }
    let w = grid[0].len();

    let directions: &[(isize, isize)] = &[
        (-1, -1),
        (-1,  0),
        (-1,  1),
        ( 0, -1),
        ( 0,  1),
        ( 1, -1),
        ( 1,  0),
        ( 1,  1),
    ];

    let mut total_removed = 0usize;

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for r in 0..h {
            for c in 0..w {
                if grid[r][c] != b'@' {
                    continue;
                }

                let mut neighbors = 0;

                for &(dr, dc) in directions {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;

                    if nr < 0 || nr >= h as isize || nc < 0 || nc >= w as isize {
                        continue;
                    }

                    if grid[nr as usize][nc as usize] == b'@' {
                        neighbors += 1;
                    }
                }

                if neighbors < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (r, c) in to_remove.iter().copied() {
            grid[r][c] = b'.';
        }

        total_removed += to_remove.len();
    }

    total_removed
}

pub fn part_two(input: &str) -> Option<u64> {
    let ans = total_removed_rolls(input);
    Some(ans as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
