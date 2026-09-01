#[derive(Debug, Clone, PartialEq, Eq)]
struct Report(pub Vec<u64>);
type Input = Vec<Report>;
#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| Report(line.split_whitespace().map(|s| s.parse().unwrap()).collect()))
        .collect()
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Increasing,
    Decreasing,
}
impl Direction {
    fn from(a: u64, b: u64) -> Self {
        if a < b { Direction::Increasing } else { Direction::Decreasing }
    }

    fn check(&self, a: u64, b: u64) -> bool {
        match self {
            Direction::Increasing => a <= b,
            Direction::Decreasing => a >= b,
        }
    }
}

fn pair_ok(direction: Direction, a: u64, b: u64) -> bool {
    let diff = a.abs_diff(b);
    direction.check(a, b) && diff >= 1 && diff <= 3
}
enum ReportResult {
    Ok,
    NotOk(usize),
}
impl From<ReportResult> for bool {
    fn from(r: ReportResult) -> Self {
        match r {
            ReportResult::Ok => true,
            ReportResult::NotOk(_) => false,
        }
    }
}
fn report_ok(report: &[u64]) -> ReportResult {
    let direction = Direction::from(report[0], report[1]);
    for (i, &[a, b]) in report.array_windows().enumerate() {
        if !pair_ok(direction, a, b) {
            return ReportResult::NotOk(i);
        }
    }
    ReportResult::Ok
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> u64 {
    input.iter().filter(|r| report_ok(&r.0).into()).count() as u64
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> u64 {
    input
        .iter()
        .filter(|r| match report_ok(&r.0) {
            ReportResult::Ok => return true,
            ReportResult::NotOk(i) => {
                let mut array = r.0.clone();
                for j in i.saturating_sub(1)..=i.saturating_add(1).min(array.len() - 1) {
                    let removed = array.remove(j);
                    if report_ok(&array).into() {
                        return true;
                    }
                    array.insert(j, removed);
                }
                false
            }
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 2);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 4);
    }
    #[test]
    fn test_part2_hard() {
        let input = parse("71 69 70 71 72 75");
        assert_eq!(part2(&input), 1);
    }
}
