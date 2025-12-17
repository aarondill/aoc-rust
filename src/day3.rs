type Joltage = u8;
type BatteryBank = Vec<Joltage>;

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<BatteryBank> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c.to_digit(10).expect("invalid digit") as u8).collect())
        .collect()
}
fn find_max_ind(bank: &[Joltage]) -> (usize, &Joltage) {
    bank.iter().enumerate().rev().max_by_key(|&(_, &j)| j).expect("Empty battery bank")
}

#[aoc(day3, part1)]
fn part1(input: &Vec<BatteryBank>) -> u64 {
    input
        .iter()
        .map(|bank| {
            let (max_i, max) = find_max_ind(&bank[..bank.len() - 1]);
            let other = bank[max_i + 1..].into_iter().max().unwrap();
            (10 * max + other) as u64
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Vec<BatteryBank>) -> u64 {
    input
        .iter()
        .map(|bank| {
            let mut digits = Vec::with_capacity(12);
            let mut start_index = 0;
            for n_remain in (1..=12).rev() {
                let (idx, &max) = find_max_ind(&bank[start_index..=bank.len() - n_remain]);
                digits.push(max as u64);
                // returned index is relative to the start of the slice
                start_index += idx + 1;
            }
            digits.iter().fold(0, |acc, &n| acc * 10 + n)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
987654321111111
811111111111119
234234234234278
818181911112111
";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 357);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 3121910778619);
    }
}
