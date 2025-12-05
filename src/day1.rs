#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let mut dial: i32 = 50;
    input
        .lines()
        // Yeah, yeah, mutation in a filter is bad, but it works.
        .filter(|line| {
            let dir = line.chars().next().expect("Invalid input: no direction");
            let dir: i8 = match dir {
                'L' => -1,
                'R' => 1,
                _ => panic!("Invalid direction"),
            };
            let mag: i32 = line
                .chars()
                .skip(1)
                .collect::<String>()
                .parse()
                .expect("Invalid input: not a number");
            let change: i32 = (mag * dir as i32) % 100;
            dial = (dial + change) % 100;
            if dial < 0 {
                dial += 100;
            }
            dial == 0
        })
        .count()
}
