use itertools::Itertools;

type Id = u64;
type Range = [Id; 2];
struct Input {
    // Sorted in ascending order by start
    // Inclusive end
    fresh: Vec<Range>,
    available: Vec<Id>,
}
fn consolidate(mut input: Vec<Range>) -> Vec<Range> {
    input.sort_by_key(|range| range[0]);
    // remove/consolidate overlapping ranges
    let mut i = 0;
    // Note: can't be a for loop because input.len changes!
    while i < input.len() - 1 {
        let start = &input[i];
        let mut max_end = start[1];
        // j will point to the next range that *doesn't* overlap
        // Note: it may be input.len(), so it's not always safe to use j as an index
        let mut j = i + 1;
        while j < input.len() {
            // doesn't overlap
            if input[j][0] > max_end {
                break;
            }
            // overlaps, update the max end
            max_end = std::cmp::max(max_end, input[j][1]);
            j += 1;
        }
        let last_overlap_idx = j - 1;
        let new = [start[0], max_end]; // we no longer need the references, so it's safe to modify
        // Save time by not cloning the range if it's not overlapping
        if i != last_overlap_idx {
            input.drain(i..=last_overlap_idx);
            input.insert(i, new);
        }
        i += 1;
    }
    input
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let input = input.trim();
    let mut iter = input.lines();
    let fresh_input = iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split('-').next_array().unwrap().map(|s| s.parse().unwrap()))
        .collect::<Vec<_>>();
    let available = iter.map(|line| line.parse().unwrap()).collect();
    Input { fresh: consolidate(fresh_input), available }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> usize {
    input
        .available
        .iter()
        .filter(|&&id| input.fresh.iter().any(|range| id >= range[0] && id <= range[1]))
        .count()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> u64 {
    // Because we already sorted and consolidated overlapping ranges, we can just sum the length of each range
    // Plus one because the end is inclusive
    input.fresh.iter().map(|range| range[1].abs_diff(range[0]) + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 3);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 14);
    }
}
