use eyre::{bail, ContextCompat};
use std::{collections::HashSet, fmt::Write};

type Point = (i16, i16);
type Paper = HashSet<Point>;

#[derive(Debug, Clone, Copy)]
enum Fold {
    Up(i16),
    Left(i16),
}

#[derive(Debug)]
struct Input {
    paper: Paper,
    folds: Vec<Fold>,
}

#[aoc_generator(day13)]
fn generator(input: &str) -> eyre::Result<Input> {
    let paper = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').context("unable to split point")?;
            let x = x.parse()?;
            let y = y.parse()?;

            Ok((x, y))
        })
        .collect::<eyre::Result<Paper>>()?;

    let folds = input
        .lines()
        .skip(paper.len() + 1)
        .map(|line| {
            let (instruction, position) = line.split_once('=').context("unable to split fold")?;
            let position = position.parse()?;

            let fold = match instruction {
                "fold along x" => Fold::Left(position),
                "fold along y" => Fold::Up(position),
                _ => bail!("bad fold"),
            };

            Ok(fold)
        })
        .collect::<eyre::Result<Vec<Fold>>>()?;

    Ok(Input { paper, folds })
}

fn solve(input: &Input) -> impl Iterator<Item = Paper> + '_ {
    input
        .folds
        .iter()
        .scan(input.paper.clone(), |paper, &fold| {
            let next = paper
                .iter()
                .map(|&(x, y)| match fold {
                    Fold::Left(fx) if fx < x => (-(x - fx - fx), y),
                    Fold::Up(fy) if fy < y => (x, -(y - fy - fy)),
                    _ => (x, y),
                })
                .collect::<Paper>();

            *paper = next.clone();
            Some(next)
        })
}

#[aoc(day13, part1)]
fn part1(input: &Input) -> eyre::Result<usize> {
    Ok(solve(input)
        .next()
        .context("unable to find solution")?
        .len())
}

#[aoc(day13, part2)]
fn part2(input: &Input) -> eyre::Result<String> {
    let paper = solve(input).last().context("unable to find solution")?;

    let &(maxx, _) = paper
        .iter()
        .max_by_key(|(x, _)| x)
        .context("unable to find max x")?;
    let &(_, maxy) = paper
        .iter()
        .max_by_key(|(_, y)| y)
        .context("unable to find max y")?;

    let mut buffer = String::new();
    writeln!(buffer)?;
    for y in 0..=maxy {
        for x in 0..=maxx {
            if paper.contains(&(x, y)) {
                write!(buffer, "#")?;
            } else {
                write!(buffer, ".")?;
            }
        }

        writeln!(buffer)?;
    }

    Ok(buffer)
}
