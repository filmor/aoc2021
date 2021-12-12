use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
pub fn input_generator(s: &str) -> Graph {
    let mut graph = Graph::new();
    for (l, r) in s.lines().filter_map(|l| l.split_once("-")) {
        graph.add_edge(l, r);
    }

    graph
}

#[aoc(day12, part1)]
pub fn solve_part1(graph: &Graph) -> usize {
    // let mut path = HashSet::new();
    // path.insert("start".to_owned());
    // do_walk("start".to_owned(), graph, path)
    let mut path_count: HashMap<String, usize> = HashMap::new();
    path_count.entry("start".to_owned()).or_insert(1);

    let path = vec!["start".to_owned()];
    do_walk2("start", graph, path, &mut path_count, &|node, counts| {
        node != "start" && (is_large_cave(node) || counts[node] < 1)
    })
}

#[aoc(day12, part2)]
pub fn solve_part2(graph: &Graph) -> usize {
    let mut path_count: HashMap<String, usize> = HashMap::new();
    path_count.entry("start".to_owned()).or_insert(1);

    let path = vec!["start".to_owned()];
    do_walk2("start", graph, path, &mut path_count, &|node, counts| {
        if node == "start" {
            return false;
        }
        if is_large_cave(node) {
            return true;
        }
        let count = counts[node];
        if count > 2 {
            return false;
        }
        if count == 0 {
            return true;
        }
        if count == 1 {
            return counts.iter().all(|(node, n)| is_large_cave(node) || *n < 2);
        }
        return false;
        // unreachable!("Node: {} Counts: {:?}", node, counts)
    })
}

fn do_walk2<F: Fn(&str, &HashMap<String, usize>) -> bool>(
    node: &str,
    graph: &Graph,
    path: Vec<String>,
    path_count: &mut HashMap<String, usize>,
    check_cave: &F,
) -> usize {
    if node == "end" {
        panic!("{:?}", path_count);
    }
    let mut res = 0;
    for out in graph.edges[node].iter() {
        if out == "end" {
            // println!("{},end", path.join(","));
            res += 1;
            continue;
        }
        // println!("...{}", out);
        path_count.entry(out.to_owned()).or_default();
        if !check_cave(out, &path_count) {
            continue;
        }

        *path_count.get_mut(out).unwrap() += 1;
        let mut path = path.clone();
        path.push(out.to_owned());
        res += do_walk2(out, graph, path, path_count, check_cave);

        *path_count.get_mut(out).unwrap() -= 1;
    }

    res
}

pub struct Graph {
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            edges: Default::default(),
        }
    }

    fn add_edge(&mut self, l: &str, r: &str) {
        self.edges
            .entry(l.to_owned())
            .or_default()
            .insert(r.to_owned());
        self.edges
            .entry(r.to_owned())
            .or_default()
            .insert(l.to_owned());
    }
}

fn is_large_cave(n: &str) -> bool {
    n.chars().next().unwrap().is_ascii_uppercase()
}
