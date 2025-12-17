use std::fmt;

use grid::Grid;
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Empty,
    Source,
    Splitter,
    Beam,
}
impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '^' => Self::Splitter,
            'S' => Self::Source,
            _ => unreachable!("Invalid char"),
        }
    }
}
impl fmt::Debug for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Source => write!(f, "S"),
            Self::Splitter => write!(f, "^"),
            Self::Beam => write!(f, "|"),
        }
    }
}
type Input = (Grid<Space>, (usize, usize));

#[aoc_generator(day7)]
fn parse(input: &str) -> Input {
    // flatten before collecting to avoid some allocations and reuse the same vector for the grid
    let grid: Grid<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map_into().collect())
        .collect::<Vec<_>>()
        .into();
    let p = grid
        .indexed_iter()
        .find(|&(_, &s)| s == Space::Source)
        .map(|(p, _)| p)
        .expect("No source found");
    (grid, p)
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> usize {
    let (grid, p) = input;
    let mut grid = grid.clone();

    let mut ends = vec![(p.0 + 1, p.1)]; // just below the source
    // need a buffer to avoid borrowing issues
    let mut ends_buf = Vec::with_capacity(ends.len());

    // Start at p and follow the path until we hit a splitter
    while !ends.is_empty() {
        for p in ends.drain(..) {
            let Some(Space::Empty) = grid.get(p.0, p.1) else {
                // This space has been filled, so we can't go there anymore
                continue; // don't replace the end
            };
            grid[p] = Space::Beam;
            let next = (p.0 + 1, p.1);
            match grid.get(next.0, next.1) {
                Some(Space::Empty) => ends_buf.push(next),
                Some(Space::Splitter) => {
                    if next.1 > 0 // Avoid overflow
                        && let Some(Space::Empty) = grid.get(next.0, next.1 - 1)
                    {
                        ends_buf.push((next.0, next.1 - 1));
                    };
                    if let Some(Space::Empty) = grid.get(next.0, next.1 + 1) {
                        ends_buf.push((next.0, next.1 + 1));
                    };
                }
                Some(Space::Beam) => {}
                None => {} // end of the line
                Some(Space::Source) => unreachable!("Source should not be reachable"),
            }
        }
        ends.extend(ends_buf.drain(..));
    }
    // Just find the number of unreached splitters
    grid.indexed_iter()
        .filter(|&(_, &s)| s == Space::Splitter)
        .map(|(p, _)| p)
        .map(|(r, c)| (r - 1, c))
        .filter(|(r, c)| grid.get(*r, *c) == Some(&Space::Beam))
        .count()
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> u64 {
    // Algorithm stolen from reddit :(
    let (grid, _) = input;
    let mut vals: Grid<u64> = grid.clone().map(|s| if let Space::Source = s { 1 } else { 0 });
    for r in 0..vals.rows() {
        for c in 0..vals.cols() {
            match grid.get(r + 1, c) {
                Some(Space::Empty) => vals[(r + 1, c)] += vals[(r, c)],
                Some(Space::Splitter) => {
                    if c > 0 {
                        vals[(r + 1, c - 1)] += vals[(r, c)];
                    }
                    vals[(r + 1, c + 1)] += vals[(r, c)];
                }
                _ => {}
            }
        }
    }
    vals.iter_row(vals.rows() - 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 21);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 40);
    }
}
