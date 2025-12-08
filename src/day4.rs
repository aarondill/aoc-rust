use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Space {
    Empty,
    Paper,
}
type Input = Grid<Space>;

#[aoc_generator(day4)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    '@' => Space::Paper,
                    _ => panic!("invalid character"),
                })
                .collect()
        })
        .collect::<Vec<_>>()
        .into()
}
// Returns true if the given position is removable
fn is_removeable(input: &Input, (y, x): (usize, usize)) -> bool {
    let paper_adjacent = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
        .iter()
        .map(|(dy, dx)| (y as isize + dy, x as isize + dx))
        .filter_map(|pos| match pos {
            (y, x) if x < 0 || y < 0 => None,
            (y, x) if x >= input.cols() as isize || y >= input.rows() as isize => None,
            (y, x) => Some((y as usize, x as usize)),
        })
        .filter(|&pos| input[pos] == Space::Paper)
        .count();
    paper_adjacent < 4
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
