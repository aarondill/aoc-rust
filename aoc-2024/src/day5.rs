use std::collections::HashMap;

// .0 must come before .1 in the input
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Sequence(u64, u64);
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Input {
    sequences: Vec<Sequence>,
    orders: Vec<Vec<u64>>,
}
#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let mut iter = input.lines().map(|line| line.trim());
    let sequences = (&mut iter)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            Sequence(a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    let orders = iter.map(|line| line.split(",").map(|s| s.parse().unwrap()).collect()).collect();
    Input { sequences, orders }
}

fn is_correct(order: &[u64], sequences: &[Sequence]) -> bool {
    let ind_map = order.iter().enumerate().map(|(i, order)| (order, i)).collect::<HashMap<_, _>>();
    sequences
        .iter()
        .filter(|seq| ind_map.contains_key(&seq.0))
        .filter(|seq| ind_map.contains_key(&seq.1))
        .all(|seq| ind_map[&seq.0] < ind_map[&seq.1])
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> u64 {
    input
        .orders
        .iter()
        .filter(|order| is_correct(order, &input.sequences))
        .map(|order| order[order.len() / 2])
        .sum()
}

// #[aoc(day5, part2)]
// fn part2(input: &Input) -> u64 {
//     todo!("part 2 is not implemented yet")
// }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 143);
    }
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&parse(INPUT)), _);
    // }
}
