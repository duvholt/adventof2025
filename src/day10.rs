use std::collections::VecDeque;

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

    let mut button_presses = 0;

    for (i, machine) in machines.iter().enumerate() {
        println!("Initial state {:?} [{}/{}]", machine, i + 1, machines.len());
        let mut queue = Vec::new();

        // check if all lights are already on
        if machine.lights.iter().all(|v| !*v) {
            println!("all lights are good");
            continue;
        }

        for (button_i, button) in machine.buttons.iter().enumerate() {
            queue.push(State2 {
                total: 1,
                last_button: button_i,
                joltage: switch_joltage(vec![0; machine.lights.len()], button),
            });
        }

        'state: while let Some(state) = queue.pop() {

            // println!("wtf {:?} {}", state, queue.len());
            if state.joltage == machine.joltage {
                println!("Solution found {}", state.total);
                button_presses += state.total;
                break;
            }

            for (i, jolt) in state.joltage.iter().enumerate() {
                if *jolt > machine.joltage[i] {
                    continue 'state;
                }
            }

            for (button_i, button) in machine.buttons.iter().enumerate() {
                queue.push(State2 {
                    total: state.total + 1,
                    last_button: button_i,
                    joltage: switch_joltage(state.joltage.clone(), button),
                });
            }
        }
    }

    button_presses.to_string()
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
