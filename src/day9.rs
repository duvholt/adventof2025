pub fn part1(contents: String) -> String {
    let coordinates = parse(contents);

    let mut max_area = 0;

    // bruteforce
    for (i1, &c1) in coordinates.iter().enumerate() {
        for (_i2, &c2) in coordinates.iter().enumerate().skip(i1 + 1) {
            let area = area(c1, c2);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area.to_string()
}

pub fn part2(contents: String) -> String {
    let red_tiles = parse(contents);

    let mut max_area = 0;

    for (i1, &c1) in red_tiles.iter().enumerate() {
        'tile: for (_i2, &c2) in red_tiles.iter().enumerate().skip(i1 + 1) {
            let (x1, y1) = c1;
            let (x2, y2) = c2;

            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            // check if any edge lines cross the area
            for (i1, line_point1) in red_tiles.iter().enumerate() {
                let i2 = if i1 + 1 == red_tiles.len() { 0 } else { i1 + 1 };
                let line_point2 = red_tiles[i2];

                let line_min_x = line_point1.0.min(line_point2.0);
                let line_max_x = line_point1.0.max(line_point2.0);
                let line_min_y = line_point1.1.min(line_point2.1);
                let line_max_y = line_point1.1.max(line_point2.1);

                // vertical line
                if line_min_x == line_max_x {
                    let crosses_y = (line_min_y < min_y && line_max_y > min_y)
                        || (line_min_y < max_y && line_max_y > max_y)
                        || (line_min_y >= min_y && line_max_y <= max_y);
                    let crosses_x = min_x < line_min_x && line_min_x < max_x;
                    if crosses_x && crosses_y {
                        continue 'tile;
                    }
                // horizontal
                } else if line_min_y == line_max_y {
                    let crosses_x = (line_min_x < min_x && line_max_x > min_x)
                        || (line_min_x < max_x && line_max_x > max_x)
                        || (line_min_x >= min_x && line_max_x <= max_x);
                    let crosses_y = min_y < line_min_y && line_min_y < max_y;
                    if crosses_x && crosses_y {
                        continue 'tile;
                    }
                }
            }

            let area = area(c1, c2);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area.to_string()
}

fn area(c1: (i64, i64), c2: (i64, i64)) -> i64 {
    let width = (c1.0 - c2.0).abs() + 1;
    let height = (c1.1 - c2.1).abs() + 1;

    width * height
}

fn parse(contents: String) -> Vec<(i64, i64)> {
    let coordinates: Vec<(i64, i64)> = contents
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    coordinates
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/9/real.txt").unwrap()),
            "4738108384"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/9/real.txt").unwrap()),
            "1513792010"
        );
    }
}
