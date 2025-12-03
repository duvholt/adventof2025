pub fn part1(contents: String) -> String {
    let mut sum = 0;
    for line in contents.lines() {
        let numbers = parse(line);

        let (max_i, max) = greedy_max(&numbers, 0, 2);
        let (_max_j, max2) = greedy_max(&numbers, max_i + 1, 1);

        sum += max * 10 + max2;
    }
    sum.to_string()
}

fn parse(line: &str) -> Vec<i64> {
    let numbers: Vec<i64> = line
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    numbers
}

pub fn part2(contents: String) -> String {
    let mut sum = 0;
    for line in contents.lines() {
        let numbers = parse(line);

        let mut i = 0;

        for battery in (1..=12).rev() {
            let (max_i, max) = greedy_max(&numbers, i, battery);
            i = max_i + 1;

            sum += 10_i64.pow(battery as u32 - 1) * max;
        }
    }
    sum.to_string()
}

fn greedy_max(numbers: &[i64], start_i: usize, left: usize) -> (usize, i64) {
    let mut max_i = 0;
    let mut max = -1;
    for (i, number) in numbers.iter().enumerate().skip(start_i) {
        if i == numbers.len() - (left - 1) {
            break;
        }
        if *number > max {
            max = *number;
            max_i = i;
        }
    }
    (max_i, max)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/3/real.txt").unwrap()),
            "17554"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/3/real.txt").unwrap()),
            "175053592950232"
        );
    }
}
