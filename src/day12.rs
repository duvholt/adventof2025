use gxhash::{HashMap, HashMapExt, HashSet, HashSetExt};
use itertools::{Itertools, iproduct};
use std::collections::VecDeque;

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    shape_count: Vec<usize>,
}

type BoolShape = Vec<Vec<bool>>;
type CoordinateShape = Vec<(usize, usize)>;

pub fn part1(contents: String) -> String {
    let mut shapes: Vec<Vec<&str>> = Vec::new();
    let mut regions = Vec::new();
    let mut shape = Vec::new();
    for line in contents.lines() {
        if line.contains("#") || line.contains(".") {
            shape.push(line);
        }
        if line.is_empty() {
            if !shape.is_empty() {
                shapes.push(shape.clone());
                shape = Vec::new();
            }
            continue;
        } else if line.contains("x") {
            let mut parts = line.split(": ");
            let mut size_parts = parts.next().unwrap().split("x");
            let shape_count: Vec<usize> = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|v| v.parse().unwrap())
                .collect();
            regions.push(Region {
                width: size_parts.next().unwrap().parse().unwrap(),
                height: size_parts.next().unwrap().parse().unwrap(),
                shape_count,
            });
        }
    }

    // parse shape
    let shapes: Vec<BoolShape> = shapes
        .into_iter()
        .map(|shape| {
            shape
                .into_iter()
                .map(|v| v.chars().map(|v| v == '#').collect())
                .collect()
        })
        .collect();

    // add flipped versions
    let shapes: Vec<Vec<BoolShape>> = shapes
        .into_iter()
        .map(|shape| {
            // vertical flip
            let mut vertical_flipped = shape.clone();
            vertical_flipped.reverse();

            let horizontal_flipped = shape
                .iter()
                .map(|row| {
                    let mut n = row.clone();
                    n.reverse();
                    n
                })
                .collect();

            vec![shape, vertical_flipped, horizontal_flipped]
        })
        .collect();

    // flatten shape
    let shapes: Vec<Vec<CoordinateShape>> = shapes
        .into_iter()
        .map(|shapes| {
            shapes
                .into_iter()
                .map(|shape| {
                    let mut coordinates = Vec::new();
                    for (y, row) in shape.into_iter().enumerate() {
                        for (x, cell) in row.into_iter().enumerate() {
                            if cell {
                                coordinates.push((x, y));
                            }
                        }
                    }
                    coordinates
                })
                .collect()
        })
        .collect();

    // add rotated versions
    let shapes: Vec<Vec<Vec<(usize, usize)>>> = shapes
        .into_iter()
        .map(|shapes| {
            let mut rotated_shapes = Vec::new();
            for shape in shapes {
                rotated_shapes.extend(rotate_shapes(shape));
            }

            // sorting so that we can dedup
            rotated_shapes = rotated_shapes
                .into_iter()
                .map(|mut v| {
                    v.sort();
                    v
                })
                .collect();
            rotated_shapes.sort();

            rotated_shapes.dedup();

            // println!("After");
            // for r in rotated_shapes.iter() {
            //     print_shape(r);
            // }

            rotated_shapes
        })
        .collect();

    let mut sum = 0;

    for region in regions.iter() {
        if solve_region(region, &shapes) {
            sum += 1;
        }
        // break;
    }

    sum.to_string()
}

fn rotate_shapes(shape: Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let mut rotated_shapes = vec![shape.clone()];
    let mut rotate_map: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    rotate_map.insert((0, 0), (2, 0));
    rotate_map.insert((0, 1), (1, 0));
    rotate_map.insert((0, 2), (0, 0));
    rotate_map.insert((1, 0), (2, 1));
    rotate_map.insert((1, 1), (1, 1));
    rotate_map.insert((1, 2), (0, 1));
    rotate_map.insert((2, 0), (2, 2));
    rotate_map.insert((2, 1), (1, 2));
    rotate_map.insert((2, 2), (0, 2));
    let mut prev_shape = shape;
    for _ in 0..3 {
        let new_shape: Vec<(usize, usize)> = prev_shape
            .iter()
            .map(|v| *rotate_map.get(v).unwrap())
            .collect();

        rotated_shapes.push(new_shape.clone());
        prev_shape = new_shape;
    }
    rotated_shapes
}

struct State {
    available_region: Vec<Vec<bool>>,
    shapes_left: Vec<usize>,
    state_key: Vec<((usize, usize), (usize, usize))>,
}

fn solve_region(region: &Region, shapes: &[Vec<Vec<(usize, usize)>>]) -> bool {
    // assumption: all shapes are 3x3
    // let mut queue = VecDeque::new();
    let mut start_region = Vec::new();
    for _y in 0..region.height {
        let mut row = Vec::new();
        for _x in 0..region.width {
            row.push(true);
        }
        start_region.push(row);
    }

    let mut all_region_shapes = Vec::new();
    for (shape_i, count) in region.shape_count.iter().enumerate() {
        for _ in 0..*count {
            all_region_shapes.push(shape_i);
        }
    }

    println!("All region shapes: {}", all_region_shapes.len());

    let shapes_permutations = all_region_shapes
        .iter()
        .cloned()
        .permutations(all_region_shapes.len())
        .unique();
        // .collect();

    // let shapes_permutations: HashSet<_> = repeat_n(all_region_shapes
    //     .iter()
    //     .cloned())
    //     .permutations(all_region_shapes.len())
    //     .collect();

    // let all_alt_permutations: Vec<_> = shapes.iter().multi_cartesian_product().collect();

    for shapes_permutation in shapes_permutations {
        let all_alt_permutations = shapes_permutation
            .iter()
            .enumerate()
            .map(|(i, s)| &shapes[*s])
            .multi_cartesian_product().unique();

        'all: for shapes in all_alt_permutations {
            let mut available_region = start_region.clone();
            let mut start = (0, 0);
            for (i, alt_shape) in shapes.iter().enumerate() {
                // let alt_shape = alt_permutation[*shape_i];

                let found = greedy_find_position(region, &available_region, &alt_shape, start);
                if let Some((found)) = found {
                    // println!("{i}: Found");
                    // print_shape(alt_shape);
                    // print_state(region, &found);
                    available_region = found;
                    // start = new_start;
                } else {
                    // println!("{i}: Not found");
                    // print_shape(&alt_shape);
                    // print_state(region, &available_region);

                    // println!("Shape: {:?}", shapes_permutation);
                    // println!("Alt: {:?}", alt_permutation);
                    continue 'all;
                }
            }
            println!("{:?}", shapes_permutation);
            println!("{:?}", shapes);
            println!("Solution found!");
            // let mut solution_region = start_region.clone();
            // let mut start = (0, 0);
            // for shape in shapes.iter() {
            //     print_shape(&shape);
            //     let (s) = greedy_find_position(region, &solution_region, &shape, start).unwrap();
            //     solution_region = s;
            //     // start = start2;
            //     print_state(region, &solution_region);
            // }
            // println!();
            print_state(region, &available_region);
            
            // return true;
        }
    }

    false
}

fn greedy_find_position(
    region: &Region,
    available_region: &Vec<Vec<bool>>,
    shape: &Vec<(usize, usize)>,
    start: (usize, usize),
) -> Option<(Vec<Vec<bool>>)> {
    for y in start.1..region.height - 2 {
        for x in start.0..region.width - 2 {
            let mut fit = true;
            let mut new_available_region = available_region.clone();
            for shape_rel_position in shape {
                let shape_position = (shape_rel_position.0 + x, shape_rel_position.1 + y);
                if !available_region[shape_position.1][shape_position.0] {
                    fit = false;
                    break;
                }
                new_available_region[shape_position.1][shape_position.0] = false;
            }
            if fit {
                return Some((new_available_region));
            }
        }
    }
    None
}

fn print_shape(shape: &[(usize, usize)]) {
    for y in 0..3 {
        let mut line = vec![];
        for x in 0..3 {
            let v = if shape.contains(&(x, y)) { '#' } else { '.' };
            line.push(v);
        }
        println!("{}", line.into_iter().collect::<String>())
    }
    println!()
}

fn print_state(region: &Region, available_region: &Vec<Vec<bool>>) {
    for y in 0..region.height {
        let mut line = vec![];
        for x in 0..region.width {
            let v = if available_region[y][x] { '.' } else { '#' };
            line.push(v);
        }
        println!("{}", line.into_iter().collect::<String>())
    }
    println!()
}

fn print_state_with_numbers(region: &Region, available_region: &Vec<Vec<usize>>) {
    for y in 0..region.height {
        let mut line = vec![];
        for x in 0..region.width {
            let v = available_region[y][x].to_string();
            line.push(v);
        }
        println!("{}", line.into_iter().collect::<String>())
    }
    println!()
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
            part1(fs::read_to_string("./input/12/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/12/real.txt").unwrap()),
            "example2"
        );
    }
}
