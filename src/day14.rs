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
            let mut from_iter = from.chars();
            let from = (
                from_iter.next().context("unable to split from")?,
                from_iter.next().context("unable to split from")?,
            );
            let to = to
                .chars()
                .next()
                .context("unable to find second rule part")?;
            Ok((from, to))
        })
        .collect::<eyre::Result<_>>()?;

    Ok(Input { template, rules })
}

fn count_pairs(template: &[char]) -> HashMap<Pair, usize> {
    template.windows(2).fold(HashMap::new(), |mut acc, parts| {
        let pair = (parts[0], parts[1]);
        *acc.entry(pair).or_default() += 1 as usize;
        acc
    })
}

fn count_elements(polymer: &HashMap<Pair, usize>) -> HashMap<char, usize> {
    polymer
        .into_iter()
        .fold(HashMap::new(), |mut acc, (&(left, _), &count)| {
            *acc.entry(left).or_default() += count;
            acc
        })
}

fn expand(
    initial: HashMap<Pair, usize>,
    rules: &HashMap<Pair, char>,
) -> impl Iterator<Item = HashMap<Pair, usize>> + '_ {
    iter::successors(Some(initial), |initial| {
        let mut next = HashMap::new();
        for (&pair @ (left, right), count) in initial {
            if let Some(&middle) = rules.get(&pair) {
                *next.entry((left, middle)).or_default() += count;
                *next.entry((middle, right)).or_default() += count;
            } else {
                *next.entry(pair).or_default() += count;
            }
        }

        Some(next)
    })
}

fn solve(input: &Input, rounds: usize) -> eyre::Result<usize> {
    let last = input
        .template
        .last()
        .cloned()
        .context("unable to get last element")?;

    let initial = count_pairs(&input.template);
    let polymer = expand(initial, &input.rules)
        .take(rounds + 1)
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
fn part1(input: &Input) -> eyre::Result<usize> {
    solve(input, 10)
}

#[aoc(day14, part2)]
fn part2(input: &Input) -> eyre::Result<usize> {
    solve(input, 40)
}