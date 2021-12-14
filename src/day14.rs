use eyre::ContextCompat;
use itertools::Itertools;
use std::{collections::HashMap, iter};

type Pair = (char, char);

#[derive(Debug)]
struct Input {
    template: Vec<char>,
    rules: HashMap<Pair, char>,
}

#[aoc_generator(day14)]
fn generator(input: &str) -> eyre::Result<Input> {
    let template = input
        .lines()
        .next()
        .map(|line| line.chars().collect())
        .context("unable to find template")?;

    let rules = input
        .lines()
        .skip(2)
        .map(|line| {
            let (from, to) = line.split_once(" -> ").context("unable to split rule")?;
            let from_parts = from.chars().collect::<Vec<char>>();
            let from = (from_parts[0], from_parts[1]);
            let to = to
                .chars()
                .next()
                .context("unable to find second rule part")?;
            Ok((from, to))
        })
        .collect::<eyre::Result<_>>()?;

    Ok(Input { template, rules })
}

fn count_pairs(template: &[char]) -> HashMap<Pair, u64> {
    template.windows(2).fold(
        HashMap::with_capacity(template.len() * 2),
        |mut acc, parts| {
            let pair = (parts[0], parts[1]);
            *acc.entry(pair).or_default() += 1;
            acc
        },
    )
}

fn count_elements(polymer: &HashMap<Pair, u64>) -> HashMap<char, u64> {
    polymer.iter().fold(
        HashMap::with_capacity(polymer.len()),
        |mut acc, (&(left, _), &count)| {
            *acc.entry(left).or_default() += count;
            acc
        },
    )
}

fn expand(
    initial: HashMap<Pair, u64>,
    rules: &HashMap<Pair, char>,
) -> impl Iterator<Item = HashMap<Pair, u64>> + '_ {
    iter::successors(Some(initial), |initial| {
        let next = initial.iter().fold(
            HashMap::with_capacity(initial.len()),
            |mut acc, (&pair @ (left, right), count)| {
                if let Some(&middle) = rules.get(&pair) {
                    *acc.entry((left, middle)).or_default() += count;
                    *acc.entry((middle, right)).or_default() += count;
                }

                acc
            },
        );

        Some(next)
    })
    .skip(1)
}

fn solve(input: &Input, rounds: usize) -> eyre::Result<u64> {
    let last = input
        .template
        .last()
        .cloned()
        .context("unable to get last element")?;

    let initial = count_pairs(&input.template);
    let polymer = expand(initial, &input.rules)
        .take(rounds)
        .last()
        .context("unable to expand polymer")?;
    let mut counts = count_elements(&polymer);
    *counts.entry(last).or_default() += 1;

    let ((_, min), (_, max)) = counts
        .into_iter()
        .minmax_by_key(|&(_, count)| count)
        .into_option()
        .context("unable to find min/max")?;

    Ok(max - min)
}

#[aoc(day14, part1)]
fn part1(input: &Input) -> eyre::Result<u64> {
    solve(input, 10)
}

#[aoc(day14, part2)]
fn part2(input: &Input) -> eyre::Result<u64> {
    solve(input, 40)
}
