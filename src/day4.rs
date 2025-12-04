pub fn part1(contents: String) -> String {
    let map = parse(contents);

    let mut accessed = 0;

    for (y, rows) in map.iter().enumerate() {
        for (x, cell) in rows.iter().enumerate() {
            // empty
            if !*cell {
                continue;
            }

            let rolls = adjacent_rolls(&map, y, x);
            if rolls < 4 {
                accessed += 1;
            }
        }
    }

    accessed.to_string()
}

pub fn part2(contents: String) -> String {
    let mut map = parse(contents);

    let mut total_accessed = 0;

    loop {
        let mut accessed = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                let cell = map[y][x];
                // empty
                if !cell {
                    continue;
                }
                let rolls = adjacent_rolls(&map, y, x);
                if rolls < 4 {
                    accessed += 1;
                    map[y][x] = false;
                }
            }
        }
        if accessed == 0 {
            break;
        }
        total_accessed += accessed;
    }

    total_accessed.to_string()
}

fn parse(contents: String) -> Vec<Vec<bool>> {
    let mut map = Vec::new();
    for row in contents.lines() {
        let mut map_row = Vec::new();
        for cell in row.chars() {
            let value = match cell {
                '@' => true,
                '.' => false,
                _ => panic!("Unknown"),
            };
            map_row.push(value);
        }
        map.push(map_row);
    }
    map
}

fn adjacent_rolls(map: &[Vec<bool>], y: usize, x: usize) -> i32 {
    let mut rolls = 0;
    for rel_x in -1..=1 {
        let new_x = (x as i64) + rel_x;
        // bounds
        if new_x < 0 || new_x >= map[y].len() as i64 {
            continue;
        }
        let new_x = new_x as usize;
        for rel_y in -1..=1 {
            // self check
            if rel_x == 0 && rel_y == 0 {
                continue;
            }
            let new_y = (y as i64) + rel_y;
            // bounds
            if new_y < 0 || new_y >= map.len() as i64 {
                continue;
            }
            let new_y = new_y as usize;
            let is_roll = map[new_y][new_x];
            if is_roll {
                rolls += 1;
            }
        }
    }
    rolls
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/4/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/4/real.txt").unwrap()),
            "example2"
        );
    }
}
