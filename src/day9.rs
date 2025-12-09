use gxhash::{HashMap, HashMapExt, HashSet, HashSetExt};

pub fn part1(contents: String) -> String {
    let coordinates = parse(contents);

    let mut max_area = 0;

    // bruteforce
    for (i1, &c1) in coordinates.iter().enumerate() {
        for (i2, &c2) in coordinates.iter().enumerate().skip(i1 + 1) {
            let area = area(c1, c2);
            if area > max_area {
                dbg!(area, c1, c2);
                max_area = area;
            }
        }
    }

    max_area.to_string()
}

pub fn part2(contents: String) -> String {
    let red_tiles = parse(contents);

    let mut green_tiles = HashSet::new();
    for (i1, &tile) in red_tiles.iter().enumerate() {
        let i2 = if i1 + 1 == red_tiles.len() { 0 } else { i1 + 1 };
        let other = &red_tiles[i2];
        if tile.0 == other.0 {
            // horizontal
            let min = tile.1.min(other.1);
            let max = tile.1.max(other.1);
            for y in min..=max {
                green_tiles.insert((tile.0, y));
            }
        } else if tile.1 == other.1 {
            // vertical
            let min = tile.0.min(other.0);
            let max = tile.0.max(other.0);
            for x in min..=max {
                green_tiles.insert((x, tile.1));
            }
        } else {
            dbg!(tile, other);
        }
    }

    let mut green_tiles_by_x: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut green_tiles_by_y: HashMap<i64, Vec<i64>> = HashMap::new();
    // dbg!(&green_tiles);
    for &(gx, gy) in green_tiles.iter() {
        green_tiles_by_x
            .entry(gx)
            .and_modify(|v| v.push(gy))
            .or_insert_with(|| vec![gy]);

        green_tiles_by_y
            .entry(gy)
            .and_modify(|v| v.push(gx))
            .or_insert_with(|| vec![gx]);
    }
    for v in green_tiles_by_x.values_mut() {
        v.sort();
    }
    for v in green_tiles_by_y.values_mut() {
        v.sort();
    }

    let mut max_area = 0;
    let mut rect = ((0, 0), (0, 0));

    for x in 0..14 {
        let min_y = 3;
        let max_y = 5;
        // println!("Checking {} {}-{}", x, min_y, max_y);
        if (check_no_gaps(x, min_y, max_y, &green_tiles_by_x)) {
            // println!("no gaps!");
        } else {
            // println!("gap found!!!")
        }
    }

    // bruteforce
    for (i1, &c1) in red_tiles.iter().enumerate() {
        'tile: for (i2, &c2) in red_tiles.iter().enumerate().skip(i1 + 1) {
            let (x1, y1) = c1;
            let (x2, y2) = c2;

            // check for holes
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            for x in min_x..=max_x {
                if (check_no_gaps(x, min_y, max_y, &green_tiles_by_x)) {
                    // println!("no gaps! {} {}-{}", x, min_y, max_y);
                } else {
                    continue 'tile;
                }
            }

            for y in min_y..=max_y {
                if (check_no_gaps(y, min_x, max_x, &green_tiles_by_y)) {
                    // println!("no gaps! {} {}-{}", x, min_y, max_y);
                } else {
                    continue 'tile;
                }
            }

            let area = area(c1, c2);
            if area > max_area {
                dbg!(area, c1, c2);
                max_area = area;
                rect = ((min_x, min_y), (max_x, max_y));
            }
        }
    }

    // print_map(red_tiles, green_tiles, rect);

    max_area.to_string()
}

fn check_no_gaps(
    x: i64,
    min_y: i64,
    max_y: i64,
    green_tiles_by_x: &HashMap<i64, Vec<i64>>,
) -> bool {
    let mut start = true;
    let mut hole_y = true;
    let mut possible_overlap = false;
    match green_tiles_by_x.get(&x) {
        Some(gy) => {
            for i in 0..gy.len() - 1 {
                let gy1 = gy[i];
                let gy2 = gy[i + 1];
                let gap = gy1 + 1 != gy2;

                let oob =  (i == gy.len() - 2 && gy2 < max_y);
                if possible_overlap && gy2 >= max_y {
                    // println!(
                    //     "From possible to complete! x={}, {}-{} ({}-{})",
                    //     x, gy1, gy2, min_y, max_y
                    // );
                    return true;
                }

                if start {
                    hole_y = false;
                    start = false;
                } else if gap {
                    hole_y = !hole_y;
                }

                if possible_overlap && ((gap && hole_y) || oob) {
                    // println!(
                    //     "Possible but no cigar! x={}, {}-{} ({}-{}) gap={} oob={}",
                    //     x, gy1, gy2, min_y, max_y, gap, oob
                    // );
                    return false;
                }

                let overlaps = min_y <= gy1 && max_y <= gy2;
                if !hole_y && gy1 <= min_y {
                    // println!(
                    //     "Possible start! x={}, {}-{} ({}-{})",
                    //     x, gy1, gy2, min_y, max_y
                    // );
                    possible_overlap = true;
                }

                if !hole_y && gy1 <= min_y && gy2 >= max_y {
                    // println!(
                    //     "Complete overlap! x={}, {}-{} ({}-{})",
                    //     x, gy1, gy2, min_y, max_y
                    // );
                    return true;
                }

                // if gap {
                //     hole_y = !hole_y;
                //     if x == 3 {
                //         println!("[hole={}, gap={}] Checking for hole x={} y={}-{}", hole_y, gap, x, gy1, gy2);
                //     }
                //     // hole overlapping area

                //     if hole_y  && overlaps {
                //         if min_y == gy2 || max_y == gy1 {}
                //         else {
                //             continue 'tile;
                //         }
                //     }
                // } else {
                //     if x == 3 {
                //         println!("No gap {}, {}-{}", x, gy1, gy2);
                //     }
                // }
            }
        }
        None => {
            return false;
        }
    }
    // println!("Didn't find shit");
    false
}

fn area(c1: (i64, i64), c2: (i64, i64)) -> i64 {
    let width = (c1.0 - c2.0).abs() + 1;
    let height = (c1.1 - c2.1).abs() + 1;
    let area = width * height;
    area
}

fn print_map(
    red_tiles: Vec<(i64, i64)>,
    green_tiles: HashSet<(i64, i64)>,
    rect: ((i64, i64), (i64, i64)),
) {
    let min_x = red_tiles.iter().min_by(|c1, c2| c1.0.cmp(&c2.0)).unwrap().0;
    let min_y = red_tiles.iter().min_by(|c1, c2| c1.1.cmp(&c2.1)).unwrap().1;
    let max_x = red_tiles.iter().max_by(|c1, c2| c1.0.cmp(&c2.0)).unwrap().0;
    let max_y = red_tiles.iter().max_by(|c1, c2| c1.1.cmp(&c2.1)).unwrap().1;

    let (rect1_x, rect1_y) = rect.0;
    let (rect2_x, rect2_y) = rect.1;

    dbg!(rect);

    for y in min_y - 1..max_y + 2 {
        let mut line = Vec::new();
        for x in min_x - 1..max_x + 2 {
            let inside_rect = rect1_x <= x && rect2_x >= x && rect1_y <= y && rect2_y >= y;
            let icon = if red_tiles.contains(&(x, y)) {
                if inside_rect { '$' } else { '#' }
            } else if green_tiles.contains(&(x, y)) {
                if inside_rect { 'Y' } else { 'X' }
            } else {
                if inside_rect { '%' } else { '.' }
            };
            line.push(icon);
        }
        println!("{}", line.into_iter().collect::<String>());
    }
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
