use regex::regex;

type Input = str;
#[aoc(day3, part1)]
fn part1(input: &Input) -> u64 {
    let reg = regex!(r"mul\(([0-9]+),([0-9]+)\)");
    reg.captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<u64>().unwrap();
            let b = cap[2].parse::<u64>().unwrap();
            a * b
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Input) -> u64 {
    let reg = regex!(r"do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\)");
    let mut do_mul = true;
    reg.captures_iter(input)
        .map(|cap| match &cap[0] {
            "do()" => {
                do_mul = true;
                0
            }
            "don't()" => {
                do_mul = false;
                0
            }
            _ => {
                if do_mul {
                    let a = cap[1].parse::<u64>().unwrap();
                    let b = cap[2].parse::<u64>().unwrap();
                    a * b
                } else {
                    0
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), 161);
    }
    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), 48);
    }
}
