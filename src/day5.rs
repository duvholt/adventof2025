pub fn part1(contents: String) -> String {
    let (ranges, ids) = parse(contents);

    let mut sum = 0;

    for id in ids {
        for (start, end) in ranges.iter() {
            if id >= *start && id <= *end {
                sum += 1;
                break;
            }
        }
    }
    sum.to_string()
}

fn parse(contents: String) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    let mut range = true;
    for line in contents.lines() {
        if line.is_empty() {
            range = false;
            continue;
        }
        if range {
            let mut range_parts = line.split("-");
            ranges.push((
                range_parts.next().unwrap().parse().unwrap(),
                range_parts.next().unwrap().parse().unwrap(),
            ));
        } else {
            ids.push(line.parse().unwrap());
        }
    }
    (ranges, ids)
}

pub fn part2(contents: String) -> String {
    let (ranges, _ids) = parse(contents);

    let mut combined_ranges: Vec<(u64, u64)> = Vec::new();

    for (start1, end1) in ranges.iter() {
        let matched = combine_ranges(&mut combined_ranges, *start1, *end1, None);
        if !matched {
            combined_ranges.push((*start1, *end1));
        } else {
            let mut dupe_found = true;
            while dupe_found {
                dupe_found = false;
                for (i, c1) in combined_ranges.clone().iter().enumerate() {
                    let matched = combine_ranges(&mut combined_ranges, c1.0, c1.1, Some(i));
                    if matched {
                        dupe_found = matched;
                        combined_ranges.remove(i);
                        break;
                    }
                }
            }
        }
    }
    combined_ranges
        .into_iter()
        .map(|(s, e)| 1 + e - s)
        .sum::<u64>()
        .to_string()
}

fn combine_ranges(
    combined_ranges: &mut [(u64, u64)],
    start1: u64,
    end1: u64,
    index: Option<usize>,
) -> bool {
    for (i, (start2, end2)) in combined_ranges.iter_mut().enumerate() {
        if let Some(i2) = index
            && i2 == i
        {
            continue;
        }
        if start1 <= *end2 && *start2 <= end1 {
            let start = start1.min(*start2);
            let end = end1.max(*end2);
            *start2 = start;
            *end2 = end;
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/5/real.txt").unwrap()),
            "848"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/5/real.txt").unwrap()),
            "334714395325710"
        );
    }
}
