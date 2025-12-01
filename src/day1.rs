pub fn part1(contents: String) -> String {
    let mut dial = 50;

    let mut zero = 0;

    for line in contents.lines() {
        let (left, clicks) = parse(line);
        let summable_click = if left { -clicks } else { clicks };

        dial = (dial + summable_click).rem_euclid(100);

        if dial == 0 {
            zero += 1;
        }
    }

    zero.to_string()
}

fn parse(line: &str) -> (bool, i64) {
    let left = line.starts_with('L');
    let count: i64 = line
        .trim_start_matches("L")
        .trim_start_matches("R")
        .parse()
        .unwrap();
    (left, count)
}

pub fn part2(contents: String) -> String {
    let mut dial_position = 50;

    let mut zero = 0;

    for line in contents.lines() {
        let (left, clicks) = parse(line);
        let summable_click = if left { -clicks } else { clicks };

        let full_rounds = clicks / 100;
        let new_dial = dial_position + (summable_click % 100);
        if (new_dial <= 0 && dial_position > 0) || new_dial >= 100 {
            zero += 1;
        }
        zero += full_rounds;

        dial_position = (dial_position + summable_click).rem_euclid(100);
    }

    zero.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/1/real.txt").unwrap()),
            "1036"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/1/real.txt").unwrap()),
            "6228"
        );
    }
}
