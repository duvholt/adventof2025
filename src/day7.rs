use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
enum Object {
    Splitter,
}

type Point = (usize, usize);

pub fn part1(contents: String) -> String {
    let (start, splitters, max) = parse(contents);

    let mut split = 0;
    let mut explored = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1 + 1));

    // bfs
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if (y) >= max.1 {
            break;
        }
        let new = (x, y + 1);
        let edges = match splitters.get(&new) {
            Some(_) => {
                split += 1;
                vec![(new.0 - 1, new.1), (new.0 + 1, new.1)]
            }
            None => {
                vec![(new.0, new.1)]
            }
        };
        for edge in edges {
            if explored.contains(&edge) {
                continue;
            }
            explored.insert(edge);
            queue.push_back(edge);
        }
    }

    split.to_string()
}

pub fn part2(contents: String) -> String {
    let (start, splitters, max) = parse(contents);

    let mut explored = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1));

    // keeping track of overlapping beams
    let mut beam_hits = HashMap::new();
    beam_hits.insert(start, 1);
    // separate map for splitters since that's all we really care about
    let mut splitter_hits = HashMap::new();

    // bfs
    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        let (x, y) = pos;
        if (y) >= max.1 {
            break;
        }
        let current_beam_hits: u64 = *beam_hits.get(&pos).unwrap();

        let new = (x, y + 1);
        let edges = match splitters.get(&new) {
            Some(_) => {
                splitter_hits
                    .entry(new)
                    .and_modify(|v| *v += current_beam_hits)
                    .or_insert(current_beam_hits);

                vec![(new.0 - 1, new.1), (new.0 + 1, new.1)]
            }
            None => {
                vec![(new.0, new.1)]
            }
        };
        for edge in edges {
            beam_hits
                .entry(edge)
                .and_modify(|v| *v += current_beam_hits)
                .or_insert(current_beam_hits);
            if explored.contains(&edge) {
                continue;
            }
            explored.insert(edge);
            queue.push_back(edge);
        }
    }

    // count number of times a beam hits a splitter (plus itself)
    (splitter_hits.values().copied().sum::<u64>() + 1).to_string()
}

fn parse(contents: String) -> (Point, HashMap<Point, Object>, Point) {
    let mut start = (0, 0);
    let mut splitters = HashMap::new();
    let mut max = (0, 0);
    for (y, line) in contents.lines().enumerate() {
        for (x, item) in line.chars().enumerate() {
            match item {
                'S' => {
                    start = (x, y);
                }
                '^' => {
                    splitters.insert((x, y), Object::Splitter);
                }
                '.' => {
                    continue;
                }
                o => {
                    unreachable!("Unknown obj {}", o);
                }
            }
            max = (x, y);
        }
    }
    (start, splitters, max)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/7/real.txt").unwrap()),
            "1660"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/7/real.txt").unwrap()),
            "305999729392659"
        );
    }
}
