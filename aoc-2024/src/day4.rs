use std::borrow::Borrow;

use grid::Grid;

type Input = Grid<char>;
#[aoc_generator(day4)]
fn parse(input: &str) -> Input {
    input.lines().map(|r| r.chars().collect()).collect::<Vec<_>>().into()
}

pub fn diagonal_neg_slope(matrix: &Grid<char>) -> impl Iterator<Item = Vec<char>> {
    let end = matrix.cols().min(matrix.rows());
    (0..end)
        .flat_map(|off| {
            let diag1 = matrix // bottom-left to center
                .indexed_iter()
                .filter(|&((i, j), _)| i.abs_diff(j) == off && j >= i) // note: >= to include when off == 0
                .map(|(_, v)| *v)
                .collect::<Vec<_>>();
            let diag2 = matrix // top-right to center
                .indexed_iter()
                .filter(|&((i, j), _)| i.abs_diff(j) == off && j < i)
                .map(|(_, v)| *v)
                .collect::<Vec<_>>();
            if off == 0 {
                assert!(diag2.is_empty())
            }
            [diag1, diag2]
        })
        .filter(|v| !v.is_empty())
}
pub fn diagonal_pos_slope(matrix: &Grid<char>) -> impl Iterator<Item = Vec<char>> {
    let end = matrix.cols().min(matrix.rows());
    (0..end)
        .flat_map(move |off| {
            let diag1 = matrix // top-left to center
                .indexed_iter()
                .filter(|&((i, j), _)| i + j == off)
                .map(|(_, v)| *v)
                .collect::<Vec<_>>();
            let diag2 = matrix // bottom-right to center
                .indexed_iter()
                .filter(|&((i, j), _)| i + j == end + off)
                .map(|(_, v)| *v)
                .collect::<Vec<_>>();
            [diag1, diag2]
        })
        .filter(|v| !v.is_empty())
}
fn count_xmas<T: IntoIterator<Item: Borrow<char>>>(i: T) -> usize {
    let line = i.into_iter().map(|c| *c.borrow()).collect::<String>();
    line.matches("XMAS").count() + line.matches("SAMX").count()
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> usize {
    let horiz: usize = input.iter_rows().map(count_xmas).sum();
    let vert: usize = input.iter_cols().map(count_xmas).sum();

    // grab the diagonals
    let diag1: usize = diagonal_pos_slope(input).map(count_xmas).sum();
    let diag2: usize = diagonal_neg_slope(input).map(count_xmas).sum();

    let sz = input.rows() * input.cols();
    assert_eq!(input.iter_rows().flat_map(|r| r).count(), sz);
    assert_eq!(input.iter_cols().flat_map(|c| c).count(), sz);
    assert_eq!(diagonal_pos_slope(input).flatten().count(), sz);
    assert_eq!(diagonal_neg_slope(input).flatten().count(), sz);

    horiz + vert + diag1 + diag2
}

// #[aoc(day4, part2)]
// fn part2(input: &Input) -> u64 {
//     todo!("part 2 is not implemented yet")
// }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 18);
    }
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&parse(INPUT)), _);
    // }
}
