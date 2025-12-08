use gxhash::{HashSet, HashSetExt};
use std::cmp::Ordering;

type Point3 = (i64, i64, i64);

#[derive(Debug, Copy, Clone)]
struct State {
    cost: f64,
    from: usize,
    to: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost.total_cmp(&other.cost) == Ordering::Equal
    }
}

impl Eq for State {}

// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.total_cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(contents: String) -> String {
    let coordinates = parse(contents);
    let edges = sorted_edge_lengths(&coordinates);

    let mut sets: Vec<HashSet<usize>> = Vec::new();

    for State {
        cost: _cost,
        from,
        to,
    } in edges.into_iter().take(1000)
    {
        build_junctions(&mut sets, from, to);
    }

    let mut max: Vec<u64> = sets.iter().map(|s| s.len() as u64).collect();
    max.sort();
    max.reverse();

    (max[0] * max[1] * max[2]).to_string()
}

pub fn part2(contents: String) -> String {
    let coordinates = parse(contents);
    let edges = sorted_edge_lengths(&coordinates);

    let mut sets: Vec<HashSet<usize>> = Vec::with_capacity(edges.len());

    let mut last_merged = None;

    for State {
        cost: _cost,
        from,
        to,
    } in edges.into_iter()
    {
        let work_done = build_junctions(&mut sets, from, to);
        if work_done {
            last_merged = Some((from, to));
        }
    }

    let (i1, i2) = last_merged.unwrap();

    (coordinates[i1].0 * coordinates[i2].0).to_string()
}

#[inline(never)]
fn sorted_edge_lengths(coordinates: &[Point3]) -> Vec<State> {
    let mut state = Vec::with_capacity(coordinates.len());

    for (i1, c1) in coordinates.iter().enumerate() {
        for (i2, c2) in coordinates.iter().enumerate().skip(i1 + 1) {
            let euclid_dist = (((c1.0 - c2.0).pow(2) + (c1.1 - c2.1).pow(2) + (c1.2 - c2.2).pow(2))
                as f64)
                .sqrt();
            state.push(State {
                cost: euclid_dist,
                from: i1,
                to: i2,
            });
        }
    }
    state.sort_unstable();
    state
}

fn parse(contents: String) -> Vec<Point3> {
    let coordinates: Vec<Point3> = contents
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    coordinates
}

fn build_junctions(sets: &mut Vec<HashSet<usize>>, from: usize, to: usize) -> bool {
    let mut from_set = None;
    let mut to_set = None;

    for (set_i, set) in sets.iter().enumerate() {
        if from_set.is_some() && to_set.is_some() {
            break;
        }
        if set.contains(&from) {
            from_set = Some(set_i);
        }
        if set.contains(&to) {
            to_set = Some(set_i);
        }
    }

    match (from_set, to_set) {
        (None, None) => {
            let mut s = HashSet::new();
            s.insert(from);
            s.insert(to);
            sets.push(s);

            true
        }
        (None, Some(i)) => {
            let set = &mut sets[i];
            set.insert(from);

            true
        }
        (Some(i), None) => {
            let set = &mut sets[i];
            set.insert(to);

            true
        }
        (Some(i1), Some(i2)) => {
            if i1 == i2 {
                // already connected
                false
            } else {
                let set2 = sets[i2].clone();
                let set1 = &mut sets[i1];
                set1.extend(set2);
                sets.remove(i2);
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/8/real.txt").unwrap()),
            "102816"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/8/real.txt").unwrap()),
            "100011612"
        );
    }
}
