type Input = &str;
#[aoc_generator(day_)]
fn parse(input: &str) -> Input {}

#[aoc(day_, part1)]
fn part1(input: &Input) -> u64 {
    todo!("part 1 is not implemented yet")
}

// #[aoc(day_, part2)]
// fn part2(input: &Input) -> u64 {
//     todo!("part 2 is not implemented yet")
// }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), _);
    }
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&parse(INPUT)), _);
    // }
}
