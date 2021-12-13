use std::num::ParseIntError;

#[aoc_generator(day1)]
fn generator(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> usize {
    input
        .windows(2)
        .filter(|readings| readings[1] > readings[0])
        .count()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> usize {
    let sums: Vec<u32> = input
        .windows(3)
        .map(|readings| readings.iter().sum())
        .collect();
    sums.windows(2).filter(|parts| parts[1] > parts[0]).count()
}
