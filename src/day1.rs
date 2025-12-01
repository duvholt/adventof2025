pub fn part1(contents: String) -> String {
    let mut dial = 50;

    dbg!(&contents);

    let mut zero = 0;

    for line in contents.lines() {
        let left = line.starts_with('L');
        let count: i64 = line
            .trim_start_matches("L")
            .trim_start_matches("R")
            .parse()
            .unwrap();

        let val = if left { -count } else { count };

        dbg!(dial);
        dial = (dial + val).rem_euclid(100);

        println!("Left {}, count {}", left, count);
        if dial == 0 {
            zero += 1;
        }
    }

    zero.to_string()
}

pub fn part2(contents: String) -> String {
    let mut dial = 50;

    let mut zero = 0;

    for line in contents.lines() {
        let left = line.starts_with('L');
        let count: i64 = line
            .trim_start_matches("L")
            .trim_start_matches("R")
            .parse()
            .unwrap();

        let val = if left { -count } else { count };

        let full_rounds = count / 100;
        let new_dial = dial + (val % 100);
        if (new_dial <= 0 && dial > 0) || new_dial >= 100 {
            zero += 1;
        }
        zero += full_rounds;

        dial = (dial + val).rem_euclid(100);
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
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/1/real.txt").unwrap()),
            "example2"
        );
    }
}
