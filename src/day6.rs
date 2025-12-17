use grid::Grid;
use itertools::Itertools;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
}
impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '*' => Self::Multiply,
            _ => panic!("invalid operator"),
        }
    }
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }
}

type Input = (Vec<String>, Vec<Operator>);
#[aoc_generator(day6)]
fn parse(input: &str) -> Input {
    let lines = input.lines().filter(|line| !line.is_empty()).collect::<Vec<_>>();
    let (operators, lines) = lines.split_last().unwrap();
    let operators = operators.chars().filter(|c| !c.is_whitespace()).map_into().collect();
    let lines = lines.iter().copied().map_into().collect();
    (lines, operators)
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> u64 {
    let (value_lines, operators) = input;
    // Row major by default
    let grid: Grid<u64> = value_lines
        .into_iter()
        .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .into();
    // use the grid to transpose for us
    grid.iter_cols()
        .enumerate()
        .map(|(i, col)| (operators[i], col))
        .map(|(op, col)| col.copied().reduce(|a, b| op.apply(a, b)).expect("empty values!"))
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> u64 {
    let (value_lines, operators) = input;
    // A grid of digits, but with None for empty cells
    let digit_grid: Grid<Option<u64>> = value_lines
        .into_iter()
        .map(|line| {
            // Add a column of Nones to the left for easier parsing
            std::iter::once(None)
                .chain(line.chars().map(|s| s.to_digit(10).map(|d| d as u64)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .into();

    // right to left!
    let mut iter = digit_grid.iter_cols().rev();
    operators
        .iter()
        .rev()
        .copied()
        .map(|operator| {
            iter.by_ref()
                // We've reached the end of a problem's input values when we hit a column of Nones (which results in reduce returning None)
                .map_while(|row| row.copied().filter_map(|v| v).reduce(|a, b| a * 10 + b))
                .reduce(|a, b| operator.apply(a, b))
                .expect("empty values!")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +
";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 4277556);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 3263827);
    }
}
