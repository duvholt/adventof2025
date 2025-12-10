use std::collections::VecDeque;

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

    let mut total_button_presses = 0;

    for (i, machine) in machines.iter().enumerate() {
        println!("Initial state {:?} [{}/{}]", machine, i + 1, machines.len());
        // let mut queue = Vec::new();

        // check if all lights are already on
        if machine.lights.iter().all(|v| !*v) {
            println!("all lights are good");
            continue;
        }

        let mut actions = Vec::new();

        for (button_i, button) in machine.buttons.iter().enumerate() {
            let mut joltage = vec![0; machine.lights.len()];

            'action: loop {
                joltage = switch_joltage(joltage, button);
                for (i, jolt) in joltage.iter().enumerate() {
                    if *jolt > machine.joltage[i] {
                        break 'action;
                    }
                }
                actions.push(button_i);
            }
        }

        'presses: for button_presses in 1.. {
            println!("Checking {}", button_presses);
            'combination: for selection in actions.iter().combinations(button_presses) {
                let mut joltage = vec![0; machine.lights.len()];
                for action in selection {
                    let button = &machine.buttons[*action];
                    joltage = switch_joltage(joltage, button);
                    if joltage == machine.joltage {
                        println!("Solution found {}", button_presses);
                        total_button_presses += button_presses;
                        break 'presses;
                    }
                    for (i, jolt) in joltage.iter().enumerate() {
                        if *jolt > machine.joltage[i] {
                            continue 'combination;
                        }
                    }
                }
            }
        }
    }

    total_button_presses.to_string()
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
