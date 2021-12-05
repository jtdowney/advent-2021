use eyre::eyre;
use std::collections::HashMap;

type Point = (i16, i16);
type Line = (Point, Point);

#[aoc_generator(day5)]
fn generator(input: &str) -> eyre::Result<Vec<Line>> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line
                .split_once(" -> ")
                .ok_or_else(|| eyre!("unable to split line"))?;
            let (x1, y1) = left
                .split_once(',')
                .ok_or_else(|| eyre!("unable to split left points"))?;
            let (x2, y2) = right
                .split_once(',')
                .ok_or_else(|| eyre!("unable to split right points"))?;

            let x1 = x1.parse()?;
            let y1 = y1.parse()?;
            let x2 = x2.parse()?;
            let y2 = y2.parse()?;

            Ok(((x1, y1), (x2, y2)))
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &[Line]) -> usize {
    let covered = input
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .fold(
            HashMap::<Point, usize>::new(),
            |mut acc, &((x1, y1), (x2, y2))| {
                let ystart = y1.min(y2);
                let yend = y1.max(y2);
                let xstart = x1.min(x2);
                let xend = x1.max(x2);

                for y in ystart..=yend {
                    for x in xstart..=xend {
                        *acc.entry((x, y)).or_default() += 1;
                    }
                }

                acc
            },
        );

    covered.values().filter(|&count| *count >= 2).count()
}

#[aoc(day5, part2)]
fn part2(input: &[Line]) -> usize {
    let covered = input.iter().fold(
        HashMap::<Point, usize>::new(),
        |mut acc, &((x1, y1), (x2, y2))| {
            let ystart = y1.min(y2);
            let yend = y1.max(y2);
            let xstart = x1.min(x2);
            let xend = x1.max(x2);

            for y in ystart..=yend {
                for x in xstart..=xend {
                    if (x1 == x2) || (y1 == y2) || (x - x1).abs() == (y - y1).abs() {
                        *acc.entry((x, y)).or_default() += 1;
                    }
                }
            }

            acc
        },
    );

    covered.values().filter(|&count| *count >= 2).count()
}
