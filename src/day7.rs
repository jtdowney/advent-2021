use eyre::eyre;
use itertools::Itertools;

#[aoc_generator(day7)]
fn generator(input: &str) -> eyre::Result<Vec<i32>> {
    input
        .lines()
        .next()
        .ok_or_else(|| eyre!("unable to read input"))?
        .split(',')
        .map(|value| value.parse().map_err(|_| eyre!("unable to parse number")))
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[i32]) -> Option<i32> {
    let (&start, &end) = input.iter().minmax().into_option()?;
    (start..=end)
        .map(|destination| {
            input
                .iter()
                .map(|&position| (position - destination).abs())
                .sum()
        })
        .min()
}

#[aoc(day7, part2)]
fn part2(input: &[i32]) -> Option<i32> {
    let (&start, &end) = input.iter().minmax().into_option()?;
    (start..=end)
        .map(|destination| {
            input
                .iter()
                .map(|&position| {
                    let steps = (position - destination).abs();
                    (1..=steps).sum::<i32>()
                })
                .sum()
        })
        .min()
}
