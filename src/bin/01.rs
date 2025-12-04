advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let n: i32 = 100;
    let mut pos: i32 = 50; // starting position
    let mut zero_count: u64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // First character is direction: 'L' or 'R'
        let mut chars = line.chars();
        let dir = chars.next().expect("empty line after trim?");
        let dist_str: String = chars.collect();
        let dist: i32 = dist_str.parse().expect("invalid distance");

        let d = dist % n;

        match dir {
            'L' => {
                // left: towards lower numbers
                // pos = (pos - d) mod 100
                pos = (pos + n - d) % n;
            }
            'R' => {
                // right: towards higher numbers
                pos = (pos + d) % n;
            }
            _ => panic!("unexpected direction: {dir}"),
        }

        if pos == 0 {
            zero_count += 1;
        }
    }

    Some(zero_count) 
}


pub fn part_two(input: &str) -> Option<u64> {
    let n: i64 = 100;
    let mut pos: i64 = 50; // starting position
    let mut zero_count: i64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Split into direction and distance
        let (dir_part, dist_part) = line.split_at(1);
        let dir = dir_part.chars().next().expect("missing direction");
        let dist: i64 = dist_part.parse().expect("invalid distance");

        let d = dist;
        let d_mod = d % n;
        let r = pos; // pos is always in 0..=99

        // Count how many clicks in this rotation land on 0
        let first_k = match dir {
            'R' => {
                if r == 0 {
                    n
                } else {
                    n - r
                }
            }
            'L' => {
                if r == 0 {
                    n
                } else {
                    r
                }
            }
            _ => panic!("unexpected direction: {dir}"),
        };

        if first_k <= d && d > 0 {
            let hits:i64 = 1 + (d - first_k) / n;
            zero_count += hits;
        }

        // Update position for next rotation
        pos = match dir {
            'R' => (pos + d_mod) % n,
            'L' => (pos + n - d_mod) % n,
            _ => unreachable!(),
        };
    }

    Some(zero_count as u64) 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
