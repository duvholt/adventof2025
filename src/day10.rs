use std::{
    collections::{HashMap, VecDeque},
    vec,
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

pub fn part2(contents: String) -> String {
    let machines = parse(contents);

    let button_presses: u64 = machines
        .iter()
        .enumerate()
        .map(|(i, machine)| gauss_machine(machine, i, machines.len()))
        .sum();

    button_presses.to_string()
}


// Gauss-Jordan elimination
fn gauss_machine(machine: &Machine, i: usize, length: usize) -> u64 {
    if false {
        println!("Initial state {:?} [{}/{}]", machine, i + 1, length);
    }


    // linear equations in the form of Ax = b
    // converted to augmented matrix where m+1 is joltage
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

    let mut where_v: Vec<Option<usize>> = vec![None; m];

    const EPSILON: f64 = 1e-9;

    // reduced row echelon form
    // https://cp-algorithms.com/linear_algebra/linear-system-gauss.html
    let mut row: usize = 0;
    for col in 0..m {
        if row >= n {
            break;
        }
        let mut sel = row;
        for i in row..n {
            if (a[i][col].abs()) > a[sel][col].abs() {
                sel = i;
            }
        }
        if a[sel][col].abs() < EPSILON {
            continue;
        }
        // Row switching
        a.swap(sel, row);

        where_v[col] = Some(row);

        // Row multiplication / reduction
        let c = a[row][col];
        for j in col..=m {
            a[row][j] /= c;
        }

        // Row addition
        for i in 0..n {
            if i != row {
                let c = a[i][col] / a[row][col];
                for j in col..=m {
                    a[i][j] -= a[row][j] * c
                }
            }
        }
        row += 1;
    }

    // find free variables

    let mut equations: Vec<Vec<(f64, Option<usize>)>> = vec![];

    let mut free_map: HashMap<usize, usize> = HashMap::new();
    let mut free_index = 0;

    for (col, w) in where_v.iter().enumerate() {
        if w.is_none() {
            free_map.insert(col, free_index);

            free_index += 1;
        }
    }

    for (col, w) in where_v.iter().enumerate() {
        match w {
            Some(row) => {
                let mut equation: Vec<(f64, Option<usize>)> = vec![(a[*row][m], None)];
                let mut f = false;
                for i in 0..m {
                    let v = a[*row][i];
                    if !f {
                        if v == 1.0 {
                            // skip until pivot
                            f = true;
                        }
                        // skip pivot too
                        continue;
                    }

                    if v != 0.0 {
                        let free_index = *free_map.get(&i).unwrap();
                        equation.push((-v, Some(free_index)));
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

    // find minimal solution by brute force

    let mut minimum = usize::MAX;

    let max_joltage = machine.joltage.iter().max().unwrap();

    let free_combi = free_map
        .iter()
        .map(|_| 0..=*max_joltage)
        .multi_cartesian_product();

    'combi: for free_values in free_combi {
        let mut button_presses = vec![];
        for equation in equations.iter() {
            let presses = equation
                .iter()
                .map(|(value, free_var)| match free_var {
                    Some(free_var) => value * (free_values[*free_var] as f64),
                    None => *value,
                })
                .sum::<f64>();
            let int_presses = presses.round() as i64;
            if int_presses < 0 || (int_presses as f64 - presses).abs() > EPSILON {
                continue 'combi;
            }
            button_presses.push(int_presses as usize);
        }
        let button_sum = button_presses.iter().sum::<usize>();
        if button_sum < minimum {
            minimum = button_sum;
            // verify solution
            if false {
                let mut joltage = vec![0; machine.joltage.len()];
                for (button_i, button_press) in button_presses.iter().enumerate() {
                    for _ in 0..*button_press {
                        joltage = switch_joltage(joltage, &machine.buttons[button_i]);
                    }
                }
                if joltage != machine.joltage {
                    panic!("WRONG JOLTAGE! {:?}, {:?}", joltage, machine.joltage);
                }
            }
        }
    }

    if minimum == usize::MAX {
        panic!("No solution found");
    }

    minimum as u64
}

#[allow(unused)]
fn print_matrix(a: &Vec<Vec<f64>>) {
    for row in a.iter() {
        println!("{:?}", row);
    }
    println!();
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
            "545"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/10/real.txt").unwrap()),
            "22430"
        );
    }
}
