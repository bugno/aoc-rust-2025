advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
       // Split into "ranges" part and "ids" part around the first blank line
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();

    let mut after_blank = false;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            after_blank = true;
            continue;
        }

        if !after_blank {
            // Parse range: "a-b"
            let (start_str, end_str) = line
                .split_once('-')
                .expect("range line must be of form a-b");
            let start: u64 = start_str.parse().expect("invalid range start");
            let end: u64 = end_str.parse().expect("invalid range end");
            ranges.push((start, end));
        } else {
            // Parse ID line: just a number
            let id: u64 = line.parse().expect("invalid ingredient ID");
            ids.push(id);
        }
    }

    // For each id, check if it's within ANY of the ranges (inclusive)
    let mut fresh_count = 0usize;

    'outer: for id in ids {
        for &(start, end) in &ranges {
            if id >= start && id <= end {
                fresh_count += 1;
                continue 'outer;
            }
        }
        // otherwise spoiled; do nothing
    }

    Some(fresh_count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
     let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut seen_blank = false;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            seen_blank = true;
            continue;
        }
        if seen_blank {
            // second section is irrelevant for part 2
            continue;
        }

        let (start_str, end_str) = line
            .split_once('-')
            .expect("range line must be of form a-b");
        let start: u64 = start_str.parse().expect("invalid range start");
        let end: u64 = end_str.parse().expect("invalid range end");
        assert!(start <= end);
        ranges.push((start, end));
    }

    if ranges.is_empty() {
        return Some(0);
    }

    // Sort ranges by start, then end
    ranges.sort_by_key(|&(s, e)| (s, e));

    // Merge overlapping/touching ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();
    let mut cur_start = ranges[0].0;
    let mut cur_end = ranges[0].1;

    for &(s, e) in &ranges[1..] {
        if s <= cur_end + 1 {
            // overlap or touch, extend current
            if e > cur_end {
                cur_end = e;
            }
        } else {
            // disjoint, push current and start new
            merged.push((cur_start, cur_end));
            cur_start = s;
            cur_end = e;
        }
    }
    merged.push((cur_start, cur_end));

    // Sum lengths of merged ranges
    let mut total: u64 = 0;
    for (s, e) in merged {
        total += (e as u64) - (s as u64) + 1;
    }

    Some(total)
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
        assert_eq!(result, Some(14));
    }
}
