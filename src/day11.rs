use std::collections::{HashMap, VecDeque};

pub fn part1(contents: String) -> String {
    let mut edges = HashMap::new();

    for line in contents.lines() {
        let mut parts = line.split(": ");
        let from = parts.next().unwrap();
        let to: Vec<&str> = parts.next().unwrap().split(" ").collect();
        edges.insert(from, to);
    }


    let mut queue = VecDeque::new();
    queue.push_back("you");

    let mut paths = 0;

    while let Some(node) = queue.pop_front() {
        if node == "out" {
            paths += 1;
            continue;
        }
        if let Some(node_edges) = edges.get(node) {
            for edge in node_edges {
                queue.push_back(edge);
            }
        }
    }


    paths.to_string()
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
            part1(fs::read_to_string("./input/11/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/11/real.txt").unwrap()),
            "example2"
        );
    }
}
