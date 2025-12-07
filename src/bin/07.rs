advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
        // Parse grid into Vec<Vec<char>>
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let h = grid.len();
    if h == 0 {
        return Some(0);
    }
    let w = grid[0].len();

    // Find S
    let mut start = None;
    for r in 0..h {
        for c in 0..w {
            if grid[r][c] == 'S' {
                start = Some((r, c));
                break;
            }
        }
        if start.is_some() {
            break;
        }
    }
    let (sr, sc) = start.expect("no S found in grid");

    // BFS/queue of active beams; each beam is at (row, col)
    use std::collections::VecDeque;

    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; w]; h];

    q.push_back((sr, sc));
    visited[sr][sc] = true;

    let mut splits: u64 = 0;

    while let Some((r, c)) = q.pop_front() {
        let nr = r + 1;
        if nr >= h {
            // Beam exits the manifold
            continue;
        }

        let cell = grid[nr][c];

        if cell == '^' {
            // Beam hits a splitter and is split
            splits += 1;

            // Spawn beams to immediate left and right (same row as splitter)
            if c > 0 {
                let nc = c - 1;
                if !visited[nr][nc] {
                    visited[nr][nc] = true;
                    q.push_back((nr, nc));
                }
            }
            if c + 1 < w {
                let nc = c + 1;
                if !visited[nr][nc] {
                    visited[nr][nc] = true;
                    q.push_back((nr, nc));
                }
            }
            // Original beam stops here (does not continue downward)
        } else {
            // Empty (or S or anything else non-^) → beam passes through
            if !visited[nr][c] {
                visited[nr][c] = true;
                q.push_back((nr, c));
            }
        }
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
        // Parse grid into Vec<Vec<u8>> (bytes)
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let h = grid.len();
    if h == 0 {
        return Some(0);
    }
    let w = grid[0].len();

    // Find S
    let mut start: Option<(usize, usize)> = None;
    for r in 0..h {
        for c in 0..w {
            if grid[r][c] == b'S' {
                start = Some((r, c));
                break;
            }
        }
        if start.is_some() {
            break;
        }
    }
    let (sr, sc) = start.expect("no S found in grid");

    // dp[r][c] = number of timelines starting from position (r, c)
    let mut dp = vec![vec![0u128; w]; h];

    // Base case: last row – next step exits, so exactly 1 timeline
    for c in 0..w {
        dp[h - 1][c] = 1;
    }

    // Fill from second-to-last row up to row 0
    for r in (0..h - 1).rev() {
        for c in 0..w {
            let nr = r + 1;
            let below = grid[nr][c];
            if below == b'^' {
                let mut ways = 0u128;
                if c > 0 {
                    ways += dp[nr][c - 1];
                }
                if c + 1 < w {
                    ways += dp[nr][c + 1];
                }
                dp[r][c] = ways;
            } else {
                dp[r][c] = dp[nr][c];
            }
        }
    }

    Some( dp[sr][sc] as u64 )
        
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
