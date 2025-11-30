use std::collections::HashMap;

pub mod example;

type Task = fn(String) -> String;

pub fn day_tasks() -> HashMap<&'static str, Task> {
    let mut map: HashMap<&str, Task> = HashMap::new();
    map.insert("example-1", example::part1);
    map.insert("example-2", example::part2);
    map
}
