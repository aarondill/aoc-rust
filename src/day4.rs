use grid::Grid;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Space {
    Empty,
    Paper,
}
impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '.' => Space::Empty,
            '@' => Space::Paper,
            _ => panic!("invalid character"),
        }
    }
}
type Input = Grid<Space>;

#[aoc_generator(day4)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map_into().collect())
        .collect::<Vec<_>>()
        .into()
}
// Returns true if the given position is removable
fn is_removeable(input: &Input, (y, x): (usize, usize)) -> bool {
    (y.saturating_sub(1)..=y.saturating_add(1))
        .cartesian_product(x.saturating_sub(1)..=x.saturating_add(1))
        .filter(|&p| p != (y, x))
        .unique()
        .filter_map(|(y, x)| input.get(y, x))
        .filter(|&&pos| pos == Space::Paper)
        .count()
        < 4
}
// Returns an iterator over the coordinates of removable Papers
fn removeable_iter(input: &Input) -> impl Iterator<Item = (usize, usize)> {
    input
        .indexed_iter()
        .filter_map(|(pos, &space)| match space {
            Space::Paper => Some(pos),
            _ => None,
        })
        .filter(|&pos| is_removeable(input, pos))
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> usize {
    // Just count the number of removable Papers
    removeable_iter(input).count()
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> u64 {
    // the aoc macro doesn't support mutable references, so we have to clone
    let input = &mut input.clone();
    // Continue removing Papers until there are no more removable Papers
    let mut count = 0;
    loop {
        // We *must* evaluate the full iterator before we mutate the input
        let removable = removeable_iter(input).collect::<Vec<_>>();
        if removable.is_empty() {
            break; // no do-whiles here
        }
        count += removable.len() as u64;
        for pos in removable {
            input[pos] = Space::Empty;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 13);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 43);
    }
}
