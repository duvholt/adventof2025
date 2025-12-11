use std::collections::{HashMap, HashSet, VecDeque};

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

pub fn part2(contents: String) -> String {
    let mut edges = HashMap::new();

    for line in contents.lines() {
        let mut parts = line.split(": ");
        let from = parts.next().unwrap();
        let to: Vec<&str> = parts.next().unwrap().split(" ").collect();
        edges.insert(from, to);
    }

    // found order by inspecting graph using graphviz :)
    // svr -> fft -> dac -> out

    let svr_fft= all_paths_from_node(&edges, "svr", "fft");
    let fft_dac = all_paths_from_node(&edges, "fft", "dac");
    let dac_out = all_paths_from_node(&edges, "dac", "out");

    let paths = svr_fft * fft_dac * dac_out;

    paths.to_string()
}

fn all_paths_from_node<'a>(
    edges: &HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    goal: &'a str,
) -> usize {
     let mut memo = HashMap::new();
    paths_between_nodes(&mut memo, edges, start, goal)
}

fn paths_between_nodes<'a>(
    memo: &mut HashMap<&'a str, usize>,
    edges: &HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    goal: &'a str,
) -> usize {
    if start == goal {
        return 1;
    }
    if let Some(paths) = memo.get(start) {
        return *paths;
    }

    let mut paths = 0;

    if let Some(node_edges) = edges.get(start) {
        for edge in node_edges {
            let sub_paths = paths_between_nodes(memo, edges, edge, goal);
            paths += sub_paths;
        }
    }

    memo.insert(start, paths);
    paths
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/11/real.txt").unwrap()),
            "786"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/11/real.txt").unwrap()),
            "495845045016588"
        );
    }
}
