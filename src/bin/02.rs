advent_of_code::solution!(2);

fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }
    let mid = len / 2;
    s[..mid] == s[mid..]
}

pub fn part_one(input: &str) -> Option<u64> {
        let mut sum = 0_u64;

    // Ranges separated by commas, possibly with trailing comma/newlines
    for part in input.split([',', '\n', '\r']) {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        let (start_str, end_str) = part
            .split_once('-')
            .expect("each range must be of the form start-end");

        let start: u64 = start_str.parse().expect("invalid start id");
        let end: u64 = end_str.parse().expect("invalid end id");

        for id in start..=end {
            if is_invalid_id(id) {
                // println!("range: {} - {}. Invalid_id {}", start, end, id);
                sum += id;
            }
        }
    }

    Some(sum)
}

fn is_invalid_id_part2(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Try all possible base lengths
    for base_len in 1..=len / 2 {
        // Total length must be a multiple of base_len
        if len % base_len != 0 {
            continue;
        }

        let repeats = len / base_len;
        if repeats < 2 {
            continue;
        }

        let pattern = &s[..base_len];
        let mut ok = true;

        for i in 1..repeats {
            let start = i * base_len;
            let end = start + base_len;
            if &s[start..end] != pattern {
                ok = false;
                break;
            }
        }

        if ok {
            return true;
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0_u64;

    // Ranges separated by commas, possibly with trailing comma/newlines
    for part in input.split([',', '\n', '\r']) {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        let (start_str, end_str) = part
            .split_once('-')
            .expect("each range must be of the form start-end");

        let start: u64 = start_str.parse().expect("invalid start id");
        let end: u64 = end_str.parse().expect("invalid end id");

        for id in start..=end {
            if is_invalid_id_part2(id) {
                sum += id;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
