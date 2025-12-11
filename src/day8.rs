use std::collections::{HashMap, HashSet};
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    x: i64,
    y: i64,
    z: i64,
}
type Input = Vec<Node>;
#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',').map(|s| s.trim().parse().unwrap());
            let n = Node {
                x: parts.next().unwrap(),
                y: parts.next().unwrap(),
                z: parts.next().unwrap(),
            };
            assert_eq!(parts.next(), None);
            n
        })
        .collect()
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
    let l = if input.len() == 1000 { 1000 } else { 10 };

    let mut map = HashMap::new();
    for &n in input {
        for &m in input {
            if n == m {
                continue;
            }
            let d = (((n.x - m.x).pow(2) + (n.y - m.y).pow(2) + (n.z - m.z).pow(2)) as f64).sqrt();
            map.insert((n, m), d);
        }
    }
    // Node -> vec<Node>
    let mut connections = HashMap::<_, Vec<_>>::new();

    for _ in 0..l {
        let (a, b) = map.iter().min_by(|(_, a), (_, b)| a.total_cmp(b)).map(|(n, _)| *n).unwrap();
        map.remove(&(a, b));
        map.remove(&(b, a));
        // Bi-directional
        connections.entry(a).or_default().push(b);
        connections.entry(b).or_default().push(a);
    }

    let mut visited = HashSet::<Node>::new();
    let mut graph_sizes =
        input.iter().map(|n| count_nodes_graph(n, &connections, &mut visited)).collect::<Vec<_>>();
    graph_sizes.sort();
    graph_sizes.iter().rev().filter(|&&n| n > 0).take(3).product()
}
fn is_fully_connected(connections: &HashMap<Node, Vec<Node>>, input: &Input) -> bool {
    count_nodes_graph(&input[0], connections, &mut HashSet::new()) == input.len()
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> i64 {
    let mut map = HashMap::new();
    for &n in input {
        for &m in input {
            if n == m {
                continue;
            }
            let d = (((n.x - m.x).pow(2) + (n.y - m.y).pow(2) + (n.z - m.z).pow(2)) as f64).sqrt();
            map.insert((n, m), d);
        }
    }
    // Node -> vec<Node>
    let mut connections = HashMap::<_, Vec<_>>::new();

    let mut prev;
    loop {
        let (a, b) = map.iter().min_by(|(_, a), (_, b)| a.total_cmp(b)).map(|(n, _)| *n).unwrap();
        map.remove(&(a, b));
        map.remove(&(b, a));
        // Bi-directional
        connections.entry(a).or_default().push(b);
        connections.entry(b).or_default().push(a);
        prev = (a, b);
        if is_fully_connected(&connections, input) {
            break;
        }
    }
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
