use std::collections::HashMap;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod example;

type Task = fn(String) -> String;

pub fn day_tasks() -> HashMap<&'static str, Task> {
    let mut map: HashMap<&str, Task> = HashMap::new();
    map.insert("example-1", example::part1);
    map.insert("example-2", example::part2);
    map.insert("11-1", day11::part1);
    map.insert("11-2", day11::part2);
    map.insert("10-1", day10::part1);
    map.insert("10-2", day10::part2);
    map.insert("9-1", day9::part1);
    map.insert("9-2", day9::part2);
    map.insert("8-1", day8::part1);
    map.insert("8-2", day8::part2);
    map.insert("7-1", day7::part1);
    map.insert("7-2", day7::part2);
    map.insert("6-1", day6::part1);
    map.insert("6-2", day6::part2);
    map.insert("5-1", day5::part1);
    map.insert("5-2", day5::part2);
    map.insert("4-1", day4::part1);
    map.insert("4-2", day4::part2);
    map.insert("3-1", day3::part1);
    map.insert("3-2", day3::part2);
    map.insert("2-1", day2::part1);
    map.insert("2-2", day2::part2);
    map.insert("1-1", day1::part1);
    map.insert("1-2", day1::part2);
    map
}
