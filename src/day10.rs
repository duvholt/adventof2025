use std::{
    cmp::min, collections::{HashMap, VecDeque}, vec
};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

#[derive(Debug)]
struct State {
    total: usize,
    lights: Vec<bool>,
}

pub fn part1(contents: String) -> String {
    let machines = parse(contents);

    let mut button_presses = 0;

    for (i, machine) in machines.iter().enumerate() {
        println!("Initial state {:?} [{}/{}]", machine, i + 1, machines.len());
        let mut queue = VecDeque::new();

        // check if all lights are already on
        if machine.lights.iter().all(|v| !*v) {
            println!("all lights are good");
            continue;
        }

        for button in machine.buttons.iter() {
            queue.push_back(State {
                total: 1,
                lights: switch_lights(vec![false; machine.lights.len()], button),
            });
        }

        while let Some(state) = queue.pop_front() {
            if state.lights == machine.lights {
                println!("Solution found {}", state.total);
                button_presses += state.total;
                break;
            }

            for button in machine.buttons.iter() {
                queue.push_back(State {
                    total: state.total + 1,
                    lights: switch_lights(state.lights.clone(), button),
                });
            }
        }
    }

    button_presses.to_string()
}

fn switch_lights(mut lights: Vec<bool>, button: &[usize]) -> Vec<bool> {
    for &b in button {
        lights[b] = !lights[b];
    }
    lights
}

#[derive(Debug)]
struct State2 {
    total: usize,
    last_button: usize,
    joltage: Vec<usize>,
}

pub fn part2(contents: String) -> String {
    let machines = parse(contents);

    let button_presses: u64 = machines
        .iter()
        .enumerate()
        .map(|(i, machine)| gauss_machine(machine, i, machines.len()))
        .sum();

    button_presses.to_string()
}

fn gauss_machine(machine: &Machine, i: usize, length: usize) -> u64 {
    println!("Initial state {:?} [{}/{}]", machine, i + 1, length);

    let (mut a, n, m) = if false {
        // let mut a = vec![
        //     vec![2.0, 1.0, -1.0, 8.0],
        //     vec![-3.0, -1.0, 2.0, -11.0],
        //     vec![-2.0, 1.0, 2.0, -3.0],
        // ];
        let mut a = vec![
            vec![3.0, -3.0, 3.0, 9.0],
            vec![2.0, -1.0, 4.0, 7.0],
            vec![3.0, -5.0, -1.0, 7.0],
        ];
        let n = a.len();
        let m = a[0].len() - 1;
        (a, n, m)
    } else {
        let m = machine.buttons.len();
        let n = machine.joltage.len();
        let mut a: Vec<Vec<f64>> = vec![vec![0.0; m + 1]; n];
        for (m_i, button) in machine.buttons.iter().enumerate() {
            for &n_i in button.iter() {
                a[n_i][m_i] = 1.0;
            }
        }
        for (n_i, value) in machine.joltage.iter().enumerate() {
            a[n_i][m] = *value as f64;
        }
        (a, n, m)
    };

    let mut where_v: Vec<Option<usize>> = vec![None; m];

    print_matrix(&a);

    let mut row: usize = 0;
    for col in 0..m {
        // println!("iter col={col} row={row}");
        if row >= n {
            break;
        }
        let mut sel = row;
        for i in row..n {
            if (a[i][col].abs()) > a[sel][col].abs() {
                sel = i;
            }
        }
        // println!("Selection {}", sel);
        // not sure about this
        if a[sel][col].abs() < f64::EPSILON {
            // println!("Skipping?");
            continue;
        }
        for i in col..=m {
            // swap
            let tmp = a[sel][i];
            a[sel][i] = a[row][i];
            a[row][i] = tmp;
        }
        // println!("Swapped sel={sel} row={row}");
        // print_matrix(&a);
        where_v[col] = Some(row);

        // not part of original
        let c = a[row][col];
        for j in col..=m {
            a[row][j] /= c;
        }
        // println!("Reduced");
        // print_matrix(&a);

        for i in 0..n {
            if i != row {
                let c = a[i][col] / a[row][col];
                // println!("c={c} i={i} row={row}");
                for j in col..=m {
                    a[i][j] -= a[row][j] * c
                }
            }
        }
        // println!("Subbed");
        // print_matrix(&a);
        row += 1;
    }

    println!("After");

    print_matrix(&a);

    // let mut ans: Vec<f64> = vec![0.0; m];

    // for i in 0..m {
    //     if let Some(w) = where_v[i] {
    //         ans[i] = a[w][m] / a[w][i];
    //     }
    // }
    // println!("ans={:?}", ans);
    // for i in 0..n {
    //     let mut sum: f64 = 0.0;
    //     for j in 0..m {
    //         sum += ans[j] * a[i][j];
    //     }
    //     if (sum - a[i][m]).abs() > 1e-9 {
    //         println!("Infinite solutions for {}", i);
    //         // panic!("wtf {} {}", sum, a[i][m]);
    //     }
    // }

    println!("{:?}", where_v);

    // x_1 = 2 - z_1 + z_2
    // x_2 = 5 - z_2
    // x_3 = 1 - z_1 + z_2
    // x_4 = z_1
    // x_5 = 3 - z_2
    // x_6 = z_2
    let mut equations: Vec<Vec<(f64, Option<usize>)>> = vec![];
    // let x_1 = vec![(2, None), (-1, Some(0)), (1, Some(1))];

    let mut free_map: HashMap<usize, usize> = HashMap::new();
    let mut free_index = 0;

    for (col, w) in where_v.iter().enumerate() {
        if w.is_none() {
            free_map.insert(col, free_index);

            free_index += 1;
        }
    }

    dbg!(&free_map);

    for (col, w) in where_v.iter().enumerate() {
        match w {
            Some(row) => {
                let mut equation: Vec<(f64, Option<usize>)> = vec![(round_float(a[*row][m]), None)];
                let mut f = false;
                // println!("{:?}", &a[*row]);
                for i in 0..m {
                    let v = a[*row][i];
                    let fixed_v = round_float(v);
                        
                    
                    if !f {
                        if v == 1.0 {
                            // skip until pivot
                            f = true;
                        }
                        // skip pivot too
                        continue;
                    }

                    // dbg!(i, v, fixed_v);

                    if v != 0.0 {
                        let free_index = *free_map.get(&i).unwrap();
                        equation.push((-fixed_v, Some(free_index)));
                    }
                }
                equations.push(equation);
            }
            None => {
                let free_index = *free_map.get(&col).unwrap();
                equations.push(vec![(1.0, Some(free_index))]);
            }
        }
    }
    // dbg!(&equations);


    // find minimal solution

    let mut minimum = f64::MAX;


    let max_joltage = machine.joltage.iter().max().unwrap();

    let free_combi = free_map.keys().map(|key| {
        0..=*machine.joltage.iter().max().unwrap()
    }).multi_cartesian_product();

    'combi: for free_values in free_combi {
        let mut button_presses = vec![];
        for equation in equations.iter() {
            let presses = equation.iter().map(|(value, free_var)| {
                match free_var {
                    Some(free_var) => value * (free_values[*free_var] as f64),
                    None => *value,
                }
            }).sum::<f64>();
            // dbg!(&presses);
            if presses < 0.0  {
                continue 'combi;
            }
            button_presses.push(presses);
        }
        let button_sum = button_presses.iter().sum::<f64>();
        if button_sum < minimum {
            println!("New best! {button_sum} {:?} {:?}", free_values, button_presses);
            minimum = round_float(button_sum);
            if (minimum - button_sum).abs() > 1e-9 {
                panic!("Float sum error");
            }
            if button_presses.len() != machine.buttons.len() {
                panic!("Wrong number of buttons");
            }
        }
    }


    dbg!(minimum);

    if minimum == f64::MAX {
        panic!("BROKEN!!!!");
    }

    // for i in i..m {
    //     if where_v[i].is_none() {
    //         panic!("wtf2");
    //     }
    // }

    // println!("Solution found? {:?}", ans);

    return minimum as u64;
}

fn round_float(v: f64) -> f64 {
    let fixed_v = if (v - v.round()).abs() < 1e-9 {
            v.round()
        } else { 
            v
        };
    fixed_v
}

fn print_matrix(a: &Vec<Vec<f64>>) {
    for row in a.iter() {
        println!("{:?}", row);
    }
    println!();
}

fn run_machine(machine: &Machine, i: usize, length: usize) -> u64 {
    println!("Initial state {:?} [{}/{}]", machine, i + 1, length);

    let mut states: Vec<Vec<usize>> = Vec::new();
    let start = vec![0; machine.joltage.len()];

    if machine.joltage == start {
        println!("joltage good from beginning");
        return 0;
    }
    states.push(start);

    let mut presses = 0;
    let objectives = machine.joltage.len();

    'solution: loop {
        presses += 1;
        let mut non_dominated: Vec<Vec<usize>> = Vec::with_capacity(states.len());

        'button: for button in machine.buttons.iter() {
            'state: for state_joltage in states.iter() {
                let new_joltage = switch_joltage(state_joltage.clone(), button);
                if new_joltage == machine.joltage {
                    println!("Solution found {}", presses);
                    return presses;
                }
                for (i, jolt) in new_joltage.iter().enumerate() {
                    if *jolt > machine.joltage[i] {
                        continue 'state;
                    }
                }
                let mut dominated = false;
                let p = new_joltage;
                for q_i in (0..non_dominated.len()).rev() {
                    let q = &non_dominated[q_i];
                    if &p == q {
                        dominated = true;
                        break;
                    } else if dominates_inverted_objectives(&p, q, objectives) {
                        let min_presses_left = p
                            .iter()
                            .enumerate()
                            .map(|(i, p1)| machine.joltage[i] - p1)
                            .max()
                            .unwrap();
                        if p.iter()
                            .enumerate()
                            .all(|(i, p1)| machine.joltage[i] - p1 >= min_presses_left)
                        {
                            // make sure to keep states that are next to the goal
                            non_dominated.swap_remove(q_i);
                        }
                    } else if !dominated && dominates_inverted_objectives(q, &p, objectives) {
                        dominated = true;
                        break;
                    }
                }
                if !dominated {
                    non_dominated.push(p);
                }
            }
        }

        if non_dominated.is_empty() {
            panic!("Broken! {states:?}");
        }

        states = non_dominated;
        // println!("State {:?}", states);
        println!(
            "[{}/{}] States {:?} with {} presses",
            i + 1,
            length,
            states.len(),
            presses
        );
    }
}

pub fn dominates_inverted_objectives(a: &Vec<usize>, b: &Vec<usize>, objectives: usize) -> bool {
    let mut equal = true;
    for i in 0..objectives {
        if a[i] < b[i] {
            return false;
        } else if a[i] > b[i] {
            equal = false;
        }
    }
    !equal
}

fn parse(contents: String) -> Vec<Machine> {
    let re = Regex::new(r"\[(.*)\] ([\d|\(|\)\s,]+) (\{[\d,]+\})").unwrap();
    let machines: Vec<_> = contents
        .lines()
        .map(|line| {
            let (_, [lights, buttons, joltage]) = re.captures(line).map(|c| c.extract()).unwrap();

            Machine {
                lights: lights.chars().map(|v| v == '#').collect(),
                buttons: buttons
                    .split_ascii_whitespace()
                    .map(|button| {
                        let button = button.strip_prefix("(").unwrap().strip_suffix(")").unwrap();

                        button.split(",").map(|v| v.parse().unwrap()).collect()
                    })
                    .collect(),
                joltage: {
                    let joltage = joltage
                        .strip_prefix("{")
                        .unwrap()
                        .strip_suffix("}")
                        .unwrap();
                    joltage.split(",").map(|v| v.parse().unwrap()).collect()
                },
            }
        })
        .collect();
    machines
}

fn switch_joltage(mut joltage: Vec<usize>, button: &[usize]) -> Vec<usize> {
    for &b in button {
        joltage[b] += 1;
    }
    joltage
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/10/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/10/real.txt").unwrap()),
            "example2"
        );
    }
}
