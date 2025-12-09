pub fn part1(contents: String) -> String {
    let coordinates: Vec<(i64, i64)> = contents.lines().map(|line| {
        let mut parts = line.split(",");
        (
                parts.next().unwrap().parse().unwrap(), 
            parts.next().unwrap().parse().unwrap(), 
        )
    }).collect();

    let mut max_area = 0;

    // bruteforce
    for (i1, &c1) in coordinates.iter().enumerate() {
        for (i2, &c2) in coordinates.iter().enumerate().skip(i1 + 1) {
            let width = (c1.0 - c2.0).abs() + 1;
            let height = (c1.1 - c2.1).abs() + 1;
            let area = width * height;
            if area > max_area {
                dbg!(area, c1, c2);
                max_area = area;
            }
        }    
    }

    max_area.to_string()
}

pub fn part2(_contents: String) -> String {
    "example2".to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/9/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/9/real.txt").unwrap()),
            "example2"
        );
    }
}
