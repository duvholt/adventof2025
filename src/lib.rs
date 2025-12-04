use std::collections::HashMap;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod example;

type Task = fn(String) -> String;

pub fn day_tasks() -> HashMap<&'static str, Task> {
    let mut map: HashMap<&str, Task> = HashMap::new();
    map.insert("example-1", example::part1);
    map.insert("example-2", example::part2);
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
