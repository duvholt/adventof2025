use rayon::prelude::*;
use std::{collections::VecDeque, vec};

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
        .par_iter()
        .enumerate()
        .map(|(i, machine)| run_machine(machine, i, machines.len()))
        .sum();

    button_presses.to_string()
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
                        if p.iter().enumerate().all(|(i, p1)| *p1 < machine.joltage[i]) {
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
        println!("States {:?} with {} presses", states.len(), presses);
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
