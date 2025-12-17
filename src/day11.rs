use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Index(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    name: String,
    connections: Vec<Index>,
}
impl<T: ToString> From<T> for Node {
    fn from(name: T) -> Self {
        Self { name: name.to_string(), connections: Vec::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Input {
    nodes: Vec<Node>,
    names: HashMap<String, Index>,
}
impl std::ops::IndexMut<&Index> for Input {
    fn index_mut(&mut self, &Index(index): Index) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
impl std::ops::Index<&Index> for Input {
    type Output = Node;

    fn index(&self, &Index(index): &Index) -> &Self::Output {
        &self.nodes[index]
    }
}

impl Input {
    fn get(&self, name: impl Borrow<str>) -> Option<&Index> {
        self.names.get(name.borrow())
    }

    fn get_or_insert(&mut self, name: impl ToString) -> &Index {
        self.names.entry(name.to_string()).or_insert_with(|| {
            self.nodes.push(name.into());
            Index(self.nodes.len() - 1)
        })
    }
}
// --- END BOILERPLATE  ---

#[aoc_generator(day11)]
fn parse(input: &str) -> Input {
    let mut ret = Input::default();
    let mut connections_buf = Vec::new(); // Allocate once to avoid reallocations 
    for line in input.lines() {
        let (name, connections) = line.split_once(": ").unwrap();
        let new = ret.get_or_insert(name.trim());
        // ideally, i'd just extend directly into the connections field, but that doesn't work due to the mutable borrow on ret
        connections_buf.extend(connections.split_whitespace().map(|name| ret.get_or_insert(name)));
        ret[new].connections.extend(connections_buf.drain(..));
    }
    ret
}

fn dfs_count<'a>(
    input: &'a Input,
    start: &'a Index,
    end: &'a Index,
    exclude: Option<&'a Index>,
) -> usize {
    let mut path = vec![start];
    let mut result: Vec<Option<usize>> = vec![None; input.nodes.len()];
    let mut child_index = vec![0; input.nodes.len()];
    while let Some(&node) = path.last() {
        if node == end || exclude.map_or(false, |exclude| node == exclude) {
            result[node.0] = if node == end { Some(1) } else { Some(0) };
            path.pop().unwrap(); // go back
            continue;
        }
        let ind = child_index[node.0];
        let children = &input[node].connections;
        if let Some(child) = children.get(ind) {
            if result[child.0] == None {
                path.push(child);
            }
            child_index[node.0] += 1;
        } else {
            // no more children
            let sum = children.iter().map(|child| result[child.0].unwrap()).sum();
            result[node.0] = Some(sum);
            path.pop().unwrap(); // go back
        }
    }
    result[start.0].unwrap()
}

#[aoc(day11, part1)]
fn part1(input: &Input) -> usize {
    let you = input.get("you").unwrap();
    let out = input.get("out").unwrap();
    dfs_count(input, you, out, None)
}

#[aoc(day11, part2)]
fn part2(input: &Input) -> usize {
    let start = input.get("svr").unwrap();
    let end = input.get("out").unwrap();
    let dac = input.get("dac").unwrap();
    let fft = input.get("fft").unwrap();

    let start_dac = dfs_count(input, start, dac, Some(fft));
    let dac_fft = dfs_count(input, dac, fft, None);
    let fft_end = dfs_count(input, fft, end, Some(dac));

    let start_fft = dfs_count(input, start, fft, Some(dac));
    let fft_dac = dfs_count(input, fft, dac, None);
    let dac_end = dfs_count(input, dac, end, Some(fft));

    start_dac * dac_fft * fft_end + start_fft * fft_dac * dac_end
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = parse(
            "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
        );
        assert_eq!(part1(&input), 5);
    }
    #[test]
    fn test_part2() {
        let input = parse(
            "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
        );
        assert_eq!(part2(&input), 2);
    }
}
