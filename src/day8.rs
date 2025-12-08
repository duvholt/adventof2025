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

    while let Some(State { cost, from, to }) = heap.pop() {
        if iters >= 1000 {
        // if iters >= 10 {
            break;
        }
        println!(
            "Working on pair {:?}->{:?}",
            coordinates[from], coordinates[to]
        );

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
                println!(
                    "Brand new set with cost {} {:?}->{:?}",
                    cost, coordinates[from], coordinates[to]
                );
                let mut s = HashSet::new();
                s.insert(from);
                s.insert(to);
                sets.push(s);
            }
            (None, Some(i)) => {
                let set = &mut sets[i];
                set.insert(from);
                let debug_set: Vec<_> = set.iter().map(|i| &coordinates[*i]).collect();
                println!(
                    "Updated to set with cost {} {:?} -> {:?}",
                    cost, coordinates[from], debug_set
                );
            },
            (Some(i), None) => {
                let set = &mut sets[i];
                set.insert(to);

                let debug_set: Vec<_> = set.iter().map(|i| &coordinates[*i]).collect();
                println!(
                    "Updated from set with cost {} {:?} -> {:?}",
                    cost, coordinates[to], debug_set
                );
            },
            (Some(i1), Some(i2)) => {
                if i1 == i2 {
                    println!("ALREADY CONNECTED");
                } else {
                    println!("MERGE CONNECTED");
                    let set2 = sets[i2].clone();
                    let set1 = &mut sets[i1];
                    set1.extend(set2.into_iter());
                    sets.remove(i2);
                }
            },
        }

        iters += 1;
    }

    let mut max: Vec<u64> = sets.iter().map(|s| s.len() as u64).collect();
    max.sort();
    max.reverse();

    // // dbg!(&sets);

    // for s in sets.iter() {
    //     println!("New set!");
    //     for i in s.iter() {
    //         println!("{:?}", coordinates[*i]);
    //     }
    // }

    (max[0] * max[1] * max[2]).to_string()
}


pub fn part2(contents: String) -> String {
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

    let mut last_merged = None;

    while let Some(State { cost, from, to }) = heap.pop() {
        // println!(
        //     "Working on pair {:?}->{:?}",
        //     coordinates[from], coordinates[to]
        // );

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
                println!(
                    "Brand new set with cost {} {:?}->{:?}",
                    cost, coordinates[from], coordinates[to]
                );
                let mut s = HashSet::new();
                s.insert(from);
                s.insert(to);
                sets.push(s);


                last_merged = Some((from, to));
            }
            (None, Some(i)) => {
                let set = &mut sets[i];
                set.insert(from);
                let debug_set: Vec<_> = set.iter().map(|i| &coordinates[*i]).collect();
                println!(
                    "Updated to set with cost {} {:?} -> {:?}",
                    cost, coordinates[from], debug_set
                );

                last_merged = Some((from, to));
            },
            (Some(i), None) => {
                let set = &mut sets[i];
                set.insert(to);

                let debug_set: Vec<_> = set.iter().map(|i| &coordinates[*i]).collect();
                println!(
                    "Updated from set with cost {} {:?} -> {:?}",
                    cost, coordinates[to], debug_set
                );

                last_merged = Some((from, to));

            },
            (Some(i1), Some(i2)) => {
                if i1 == i2 {
                    // already connected
                } else {
                    last_merged = Some((from, to));

                    let set2 = sets[i2].clone();
                    let set1 = &mut sets[i1];
                    set1.extend(set2.into_iter());
                    sets.remove(i2);
                }
            },
        }

    }

    let (i1, i2) = last_merged.unwrap();

    dbg!(coordinates[i1], coordinates[i2]);

    (coordinates[i1].0 * coordinates[i2].0).to_string()
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
