mod joltage {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Joltage(u8);
    impl std::fmt::Debug for Joltage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl Joltage {
        pub fn new(num: u8) -> Self {
            assert!(num > 0 && num < 10);
            Self(num)
        }

        pub fn value(&self) -> u8 {
            self.0
        }
    }
}
use joltage::Joltage;

type BatteryBank = Vec<Joltage>;

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<BatteryBank> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("invalid digit") as u8)
                .map(Joltage::new)
                .collect()
        })
        .collect()
}
fn find_max_ind(bank: &[Joltage]) -> (usize, &Joltage) {
    bank.iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(&b.0).reverse())) // prefer lower indices when there are multiple maxes
        .expect("Empty battery bank")
}

#[aoc(day3, part1)]
fn part1(input: &Vec<BatteryBank>) -> u64 {
    input
        .iter()
        .map(|bank| {
            let (max_i, max) = find_max_ind(&bank[..bank.len() - 1]);
            let other = bank[max_i + 1..].into_iter().max().unwrap();
            (10 * max.value() + other.value()) as u64
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
                let (idx, max) = find_max_ind(&bank[start_index..=bank.len() - n_remain]);
                digits.push(max.value() as u64);
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
