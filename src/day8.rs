use itertools::Itertools;
use std::collections::HashSet;

type Numeral = HashSet<char>;

#[aoc_generator(day8)]
fn generator(input: &str) -> Option<Vec<(Vec<Numeral>, Vec<Numeral>)>> {
    input
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(" | ")?;
            let input = input
                .split_ascii_whitespace()
                .map(|value| value.chars().collect())
                .collect();
            let output = output
                .split_ascii_whitespace()
                .map(|value| value.chars().collect())
                .collect();
            Some((input, output))
        })
        .collect()
}

fn find_segments(input: &[Numeral]) -> Option<[Numeral; 10]> {
    let mut known: [Numeral; 10] = Default::default();
    known[1] = input.iter().find(|candidate| candidate.len() == 2)?.clone();
    known[4] = input.iter().find(|candidate| candidate.len() == 4)?.clone();
    known[7] = input.iter().find(|candidate| candidate.len() == 3)?.clone();
    known[8] = input.iter().find(|candidate| candidate.len() == 7)?.clone();

    let top = known[7].difference(&known[1]).copied().collect();
    let bottom_and_bottom_left = known[8]
        .difference(&known[4])
        .copied()
        .collect::<Numeral>()
        .difference(&top)
        .copied()
        .collect();

    let partial9 = &known[4] | &top;
    known[9] = input
        .iter()
        .find(|&candidate| candidate != &known[8] && candidate & &partial9 == partial9)?
        .clone();

    let partial0 = &bottom_and_bottom_left | &known[1];
    known[0] = input
        .iter()
        .find(|&candidate| candidate != &known[8] && candidate & &partial0 == partial0)?
        .clone();

    let middle = known[9].difference(&known[0]).copied().collect();
    let top_left = known[4]
        .difference(&known[1])
        .copied()
        .collect::<Numeral>()
        .difference(&middle)
        .copied()
        .collect();

    known[3] = known[9].difference(&top_left).copied().collect();
    known[6] = input
        .iter()
        .find(|&candidate| {
            candidate.len() == 6 && candidate != &known[9] && candidate != &known[0]
        })?
        .clone();
    known[5] = input
        .iter()
        .find(|&candidate| !known.iter().contains(candidate) && !candidate.is_disjoint(&top_left))?
        .clone();
    known[2] = input
        .iter()
        .find(|&candidate| !known.iter().contains(candidate))?
        .clone();

    Some(known)
}

#[aoc(day8, part1)]
fn part1(input: &[(Vec<Numeral>, Vec<Numeral>)]) -> usize {
    input
        .iter()
        .flat_map(|(_, output)| {
            output
                .iter()
                .filter(|digit| [2, 3, 4, 7].contains(&digit.len()))
        })
        .count()
}

#[aoc(day8, part2)]
fn part2(input: &[(Vec<Numeral>, Vec<Numeral>)]) -> Option<usize> {
    input
        .iter()
        .map(|(input, output)| {
            let known = find_segments(input)?;
            output
                .iter()
                .rev()
                .enumerate()
                .map(|(index, segments)| {
                    let value = known.iter().position(|known| known == segments)?;
                    Some(value * 10_usize.pow(index as u32))
                })
                .sum::<Option<usize>>()
        })
        .sum()
}
