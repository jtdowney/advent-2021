use eyre::eyre;
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day6)]
fn generator(input: &str) -> eyre::Result<Vec<i8>> {
    input
        .lines()
        .next()
        .ok_or_else(|| eyre!("unable to read input"))?
        .split(',')
        .map(|value| value.parse().map_err(|_| eyre!("unable to parse number")))
        .collect()
}

fn solve(input: &[i8], rounds: usize) -> usize {
    let mut population = input.iter().copied().counts();
    for _ in 0..rounds {
        let mut next = HashMap::new();
        for (&cycle, &count) in population.iter() {
            let mut cycle = cycle - 1;

            if cycle < 0 {
                cycle = 6;
                *next.entry(8).or_default() += count;
            }

            *next.entry(cycle).or_default() += count;
        }

        population = next;
    }

    population.values().sum()
}

#[aoc(day6, part1)]
fn part1(input: &[i8]) -> usize {
    solve(input, 80)
}

#[aoc(day6, part2)]
fn part2(input: &[i8]) -> usize {
    solve(input, 256)
}
