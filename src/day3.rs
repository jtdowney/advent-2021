use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day3)]
fn generator(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[String]) -> eyre::Result<u32> {
    let input = (0..)
        .map_while(|i| {
            let counts = input
                .iter()
                .map(|line| line.chars().nth(i))
                .collect::<Option<Vec<char>>>()?
                .into_iter()
                .counts();

            Some(counts)
        })
        .collect::<Vec<HashMap<char, usize>>>();

    let gamma = input
        .iter()
        .map(|counts| {
            let (&value, _) = counts.iter().max_by_key(|(_, &v)| v).unwrap();
            value
        })
        .collect::<String>();
    let gamma = u32::from_str_radix(&gamma, 2)?;

    let epsilon = input
        .iter()
        .map(|counts| {
            let (&value, _) = counts.iter().min_by_key(|(_, &v)| v).unwrap();
            value
        })
        .collect::<String>();
    let epsilon = u32::from_str_radix(&epsilon, 2)?;

    Ok(gamma * epsilon)
}

#[aoc(day3, part2)]
fn part2(input: &[String]) -> eyre::Result<u32> {
    let input = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let generator = (0..)
        .try_fold(input.clone(), |mut candidates, digit| {
            if candidates.len() == 1 {
                let value = candidates[0].iter().collect::<String>();
                return Err(value);
            }

            let counts = candidates.iter().map(|line| line[digit]).counts();
            let pick = if counts[&'0'] == counts[&'1'] {
                '1'
            } else {
                let (&pick, _) = counts.iter().max_by_key(|(_, &v)| v).unwrap();
                pick
            };
            candidates.retain(|candidate| candidate[digit] == pick);

            Ok(candidates)
        })
        .unwrap_err();
    let generator = u32::from_str_radix(&generator, 2)?;

    let scrubber = (0..)
        .try_fold(input, |mut candidates, digit| {
            if candidates.len() == 1 {
                let value = candidates[0].iter().collect::<String>();
                return Err(value);
            }

            let counts = candidates.iter().map(|line| line[digit]).counts();
            let pick = if counts[&'0'] == counts[&'1'] {
                '0'
            } else {
                let (&pick, _) = counts.iter().min_by_key(|(_, &v)| v).unwrap();
                pick
            };
            candidates.retain(|candidate| candidate[digit] == pick);

            Ok(candidates)
        })
        .unwrap_err();
    let scrubber = u32::from_str_radix(&scrubber, 2)?;

    Ok(generator * scrubber)
}
