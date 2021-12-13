use eyre::{eyre, ContextCompat};
use itertools::Itertools;

#[aoc_generator(day7)]
fn generator(input: &str) -> eyre::Result<Vec<i32>> {
    input
        .lines()
        .next()
        .context("unable to read input")?
        .split(',')
        .map(|value| value.parse().map_err(|_| eyre!("unable to parse number")))
        .collect()
}

fn solve<F>(input: &[i32], cost: F) -> Option<i32>
where
    F: Fn(i32, i32) -> i32,
{
    let (&start, &end) = input.iter().minmax().into_option()?;
    (start..=end)
        .map(|destination| {
            input
                .iter()
                .map(|&position| cost(destination, position))
                .sum()
        })
        .min()
}

#[aoc(day7, part1)]
fn part1(input: &[i32]) -> Option<i32> {
    solve(input, |destination, position| {
        (position - destination).abs()
    })
}

#[aoc(day7, part2)]
fn part2(input: &[i32]) -> Option<i32> {
    solve(input, |destination, position| {
        let steps = (position - destination).abs();
        steps * (steps + 1) / 2
    })
}
