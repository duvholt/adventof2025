use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    i64,
};

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
        other.cost.total_cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(contents: String) -> String {
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

    let mut heap = BinaryHeap::new();

    for (i1, c1) in coordinates.iter().enumerate() {
        for i2 in i1 + 1..coordinates.len() {
            let c2 = coordinates[i2];
            if i1 == i2 {
                continue;
            }
            let euclid_dist = (((c1.0 - c2.0).pow(2) + (c1.1 - c2.1).pow(2) + (c1.2 - c2.2).pow(2))
                as f64)
                .sqrt();
            heap.push(State {
                cost: euclid_dist,
                from: i1,
                to: i2,
            });
        }
    }

    let mut sets: Vec<HashSet<usize>> = Vec::new();

    let mut iters = 0;

    'outer: while let Some(State { cost, from, to }) = heap.pop() {
        // if iters >= 1000 {
        if iters >= 10 {
            let mut orphan_from = true;
            for set in sets.iter_mut() {
                if set.contains(&from) {
                    orphan_from = false;
                    break;
                }
            }
            if orphan_from {
                sets.push(vec![from].into_iter().collect());
            }

            let mut orphan_to = true;
            for set in sets.iter_mut() {
                if set.contains(&to) {
                    orphan_to = false;
                    break;
                }
            }
            if orphan_to {
                sets.push(vec![to].into_iter().collect());
            }
            continue;
        }
        // dbg!(iters);
        let mut inserted = false;
        println!(
            "Working on pair {:?}->{:?}",
            coordinates[from], coordinates[to]
        );

        for set in sets.iter_mut() {
            if set.contains(&from) && set.contains(&to) {
                println!("ALREADY CONNECTED");
                continue 'outer;
            }

            if set.contains(&from) {
                set.insert(to);
                inserted = true;

                let debug_set: Vec<_> = set.iter().map(|i| &coordinates[*i]).collect();
                println!(
                    "Updated from set with cost {} {:?} -> {:?}",
                    cost, coordinates[to], debug_set
                );
                break;
            }
            if set.contains(&to) {
                set.insert(from);
                inserted = true;
                let debug_set: Vec<_> = set.iter().map(|i| &coordinates[*i]).collect();
                println!(
                    "Updated to set with cost {} {:?} -> {:?}",
                    cost, coordinates[from], debug_set
                );
                break;
            }
        }
        if !inserted {
            println!(
                "Brand new set with cost {} {:?}->{:?}",
                cost, coordinates[from], coordinates[to]
            );
            let mut s = HashSet::new();
            s.insert(from);
            s.insert(to);
            sets.push(s);
        }

        iters += 1;
    }

    let mut max: Vec<u64> = sets.iter().map(|s| s.len() as u64).collect();
    max.sort();
    max.reverse();

    // dbg!(&sets);

    for s in sets.iter() {
        println!("New set!");
        for i in s.iter() {
            println!("{:?}", coordinates[*i]);
        }
    }

    (max[0] * max[1] * max[2]).to_string()
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
            part1(fs::read_to_string("./input/8/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/8/real.txt").unwrap()),
            "example2"
        );
    }
}
