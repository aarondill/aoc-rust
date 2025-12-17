use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    x: i64,
    y: i64,
    z: i64,
}
impl From<[i64; 3]> for Node {
    fn from([x, y, z]: [i64; 3]) -> Self {
        Self { x, y, z }
    }
}

type Input = (Vec<Node>, Vec<(Node, Node, u64)>);
#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    let nodes: Vec<Node> = input
        .lines()
        .map(|line| {
            line.split(',').map(|s| s.trim().parse().unwrap()).next_array::<3>().unwrap().into()
        })
        .collect();
    let map = nodes
        .iter()
        .array_combinations()
        .map(|[n, m]| {
            let d = n.x.abs_diff(m.x).pow(2) + n.y.abs_diff(m.y).pow(2) + n.z.abs_diff(m.z).pow(2);
            (n.clone(), m.clone(), d)
        })
        .sorted_unstable_by_key(|(.., d)| *d)
        .collect();
    (nodes, map)
}
fn count_nodes_graph(
    n: &Node,
    connections: &HashMap<Node, Vec<Node>>,
    visited: &mut HashSet<Node>,
) -> usize {
    if visited.contains(n) {
        return 0;
    }
    visited.insert(*n);
    1 + connections
        .get(n)
        .map(|v| v.iter().map(|n| count_nodes_graph(n, connections, visited)).sum())
        .unwrap_or(0)
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    let (nodes, map) = input;
    let l = if nodes.len() == 1000 { 1000 } else { 10 };

    // Node -> vec<Node>
    let mut connections = HashMap::new();

    for &(a, b, _) in map.iter().take(l) {
        // Bi-directional
        connections.entry(a).or_insert_with(Vec::new).push(b);
        connections.entry(b).or_insert_with(Vec::new).push(a);
    }

    let mut visited = HashSet::new();
    nodes
        .iter()
        .map(|n| count_nodes_graph(n, &connections, &mut visited))
        .sorted()
        .rev()
        .filter(|&n| n > 0)
        .take(3)
        .product()
}
fn is_fully_connected(connections: &HashMap<Node, Vec<Node>>, nodes: &Vec<Node>) -> bool {
    count_nodes_graph(&nodes[0], connections, &mut HashSet::new()) == nodes.len()
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> i64 {
    let (nodes, map) = input;
    let mut connections = HashMap::<_, Vec<_>>::new();
    let mut i = 0;
    let prev = loop {
        let (a, b, _) = map[i];
        i += 1;
        // Bi-directional
        connections.entry(a).or_default().push(b);
        connections.entry(b).or_default().push(a);
        if is_fully_connected(&connections, nodes) {
            break (a, b);
        }
    };
    prev.0.x * prev.1.x
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 40);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 25272);
    }
}
