type Input = Vec<(u64, Vec<u64>)>;
#[aoc_generator(day7)]
fn parse(input: &str) -> Input {
    let mut equations = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(": ");
        let result = parts.next().unwrap().parse().unwrap();
        let inputs = parts.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        equations.push((result, inputs));
    }
    equations
}

enum Operation {
    Add,
    Multiply,
    Concatenate,
}
impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concatenate => a * 10_u64.pow(b.checked_ilog10().unwrap_or(0) + 1) + b,
        }
    }
}

/// Returns true if the equation can be solved with the given operations
fn validate(accum: u64, goal: u64, nums: &[u64], possible_ops: &[Operation]) -> bool {
    assert!(!possible_ops.is_empty());
    if accum == goal {
        return true;
    }
    if accum > goal {
        return false; // no need to continue, all operation will make accum greater
    }
    if nums.is_empty() {
        return false; // no need to continue, no more numbers to process
    }
    possible_ops.iter().any(|op| validate(op.apply(accum, nums[0]), goal, &nums[1..], possible_ops))
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> u64 {
    input
        .iter()
        .filter(|(g, n)| validate(0, *g, n, &[Operation::Add, Operation::Multiply]))
        .map(|(g, _)| g)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> u64 {
    input
        .iter()
        .filter(|(g, n)| {
            validate(0, *g, n, &[Operation::Add, Operation::Multiply, Operation::Concatenate])
        })
        .map(|(g, _)| g)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 3749);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 11387);
    }
}
