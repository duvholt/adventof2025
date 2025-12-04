pub fn part1(contents: String) -> String {
    let mut eqs = 0;

    for line in contents.split(',') {
        let (first, second) = parse(line);

        for i in first..=second {
            let len = i.to_string().len();
            let chunk = len / 2;
            if len != chunk * 2 {
                continue;
            }
            if equal_parts(i, chunk) {
                eqs += i;
            }
        }
    }
    eqs.to_string()
}

fn parse(line: &str) -> (i64, i64) {
    let mut parts = line.split('-');

    let first: i64 = parts.next().unwrap().trim().parse().unwrap();
    let second: i64 = parts.next().unwrap().trim().parse().unwrap();
    (first, second)
}

fn equal_parts(first: i64, parts: usize) -> bool {
    if first < 10 {
        return false;
    }
    let first_s = first.to_string();
    let vec: Vec<_> = first_s.chars().collect();
    let mut chunks = vec.chunks(parts);
    let f1 = chunks.next().unwrap();
    chunks.all(|f2| f2 == f1)
}

pub fn part2(contents: String) -> String {
    let mut eqs = 0;

    for line in contents.split(',') {
        let (first, second) = parse(line);

        for i in first..=second {
            let len = i.to_string().len();
            let chunk = len / 2;
            for c in 1..=chunk {
                if equal_parts(i, c) {
                    eqs += i;
                    break;
                }
            }
        }
    }
    eqs.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/2/real.txt").unwrap()),
            "24747430309"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/2/real.txt").unwrap()),
            "30962646823"
        );
    }
}
