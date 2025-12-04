advent_of_code::solution!(3);

fn max_bank_joltage(line: &str) -> u32 {
    let mut best: u32 = 0;
    let mut best_first: Option<u32> = None;

    for ch in line.chars() {
        if !ch.is_ascii_digit() {
            continue;
        }
        let d = ch.to_digit(10).expect("non-digit in bank");

        // Use previous best_first as first digit, current as second digit
        if let Some(first) = best_first {
            let val = first * 10 + d;
            if val > best {
                best = val;
            }
        }

        // Update best_first (largest digit so far to the left)
        best_first = Some(match best_first {
            Some(prev) if prev >= d => prev,
            _ => d,
        });
    }

    best
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum_joltage = input
        .lines()
        .map(|line| max_bank_joltage(line.trim()) as u64)
        .sum();
    Some(sum_joltage)
}

fn max_bank_joltage_2(line: &str, k: usize) -> u64 {
    // Collect digits
    let digits: Vec<u8> = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let n = digits.len();
    assert!(n >= k, "bank too short");

    let mut to_drop = n - k;          // how many digits we may remove
    let mut stack: Vec<u8> = Vec::with_capacity(k);

    for d in digits {
        // While we can drop and last digit is smaller, drop it
        while let Some(&last) = stack.last() {
            if to_drop > 0 && last < d {
                stack.pop();
                to_drop -= 1;
            } else {
                break;
            }
        }
        stack.push(d);
    }

    // If we still have digits left to drop, drop them from the end
    while to_drop > 0 {
        stack.pop();
        to_drop -= 1;
    }

    // Now stack.len() == k; turn it into a number
    let mut value: u64 = 0;
    for &d in &stack {
        value = value * 10 + d as u64;
    }
    value
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum_joltage: u64 = input
        .lines()
        .map(|line| {
            let line = line.trim();
            if line.is_empty() {
                0_u64
            } else {
                max_bank_joltage_2(line, 12)
            }
        })
        .sum();
    Some(sum_joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
