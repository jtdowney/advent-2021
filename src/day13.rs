use eyre::{bail, eyre};
use std::{collections::HashSet, fmt::Write};

type Point = (i16, i16);

#[derive(Debug)]
enum Fold {
    Up(i16),
    Left(i16),
}

#[derive(Debug)]
struct Input {
    points: HashSet<Point>,
    folds: Vec<Fold>,
}

#[aoc_generator(day13)]
fn generator(input: &str) -> eyre::Result<Input> {
    let points = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .ok_or_else(|| eyre!("unable to split point"))?;
            let x = x.parse()?;
            let y = y.parse()?;

            Ok((x, y))
        })
        .collect::<eyre::Result<HashSet<Point>>>()?;

    let folds = input
        .lines()
        .skip(points.len() + 1)
        .map(|line| {
            let (instruction, position) = line
                .split_once('=')
                .ok_or_else(|| eyre!("unable to split fold"))?;
            let position = position.parse()?;

            let fold = match instruction {
                "fold along x" => Fold::Left(position),
                "fold along y" => Fold::Up(position),
                _ => bail!("bad fold"),
            };

            Ok(fold)
        })
        .collect::<eyre::Result<Vec<Fold>>>()?;

    Ok(Input { points, folds })
}

fn solve(input: &Input) -> impl Iterator<Item = HashSet<(i16, i16)>> + '_ {
    input
        .folds
        .iter()
        .scan(input.points.clone(), |points, fold| {
            let (inside, outside) =
                points
                    .iter()
                    .partition::<HashSet<Point>, _>(|(x, y)| match fold {
                        Fold::Up(fy) => fy > y,
                        Fold::Left(fx) => fx > x,
                    });

            let next: HashSet<Point> = match fold {
                Fold::Up(fy) => inside
                    .into_iter()
                    .chain(outside.into_iter().map(|(x, y)| (x, -(y - 2 * fy))))
                    .collect(),
                Fold::Left(fx) => inside
                    .into_iter()
                    .chain(outside.into_iter().map(|(x, y)| (-(x - 2 * fx), y)))
                    .collect(),
            };

            *points = next.clone();
            Some(next)
        })
}

#[aoc(day13, part1)]
fn part1(input: &Input) -> eyre::Result<usize> {
    Ok(solve(input)
        .next()
        .ok_or_else(|| eyre!("unable to find solution"))?
        .len())
}

#[aoc(day13, part2)]
fn part2(input: &Input) -> eyre::Result<String> {
    let points = solve(input)
        .last()
        .ok_or_else(|| eyre!("unable to find solution"))?;

    let &(maxx, _) = points
        .iter()
        .max_by_key(|(x, _)| x)
        .ok_or_else(|| eyre!("unable to find max x"))?;
    let &(_, maxy) = points
        .iter()
        .max_by_key(|(_, y)| y)
        .ok_or_else(|| eyre!("unable to find max y"))?;

    let mut buffer = String::new();
    writeln!(buffer)?;
    for y in 0..=maxy {
        for x in 0..=maxx {
            if points.contains(&(x, y)) {
                write!(buffer, "#")?;
            } else {
                write!(buffer, ".")?;
            }
        }

        writeln!(buffer)?;
    }

    Ok(buffer)
}
