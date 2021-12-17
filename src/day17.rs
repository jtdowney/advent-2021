use eyre::ContextCompat;
use itertools::iproduct;
use regex::Regex;
use std::{cmp::Ordering, iter};

type Pair = (i16, i16);

#[derive(Clone, Copy, Debug)]
struct Target {
    x1: i16,
    x2: i16,
    y1: i16,
    y2: i16,
}

impl Target {
    fn contains(&self, (x, y): Pair) -> bool {
        (self.x1..=self.x2).contains(&x) && (self.y1..=self.y2).contains(&y)
    }

    fn below(&self, (_, y): Pair) -> bool {
        y < self.y1
    }
}

struct Probe {
    position: Pair,
    velocity: Pair,
}

fn simulate(starting_velocity: Pair) -> impl Iterator<Item = Pair> {
    iter::successors(
        Some(Probe {
            position: (0, 0),
            velocity: starting_velocity,
        }),
        |probe| {
            let (x, y) = probe.position;
            let (vx, vy) = probe.velocity;

            let position = (x + vx, y + vy);
            let velocity = match vx.cmp(&0) {
                Ordering::Less => (vx + 1, vy - 1),
                Ordering::Equal => (0, vy - 1),
                Ordering::Greater => (vx - 1, vy - 1),
            };

            Some(Probe { position, velocity })
        },
    )
    .map(|probe| probe.position)
}

#[aoc_generator(day17)]
fn generator(input: &str) -> eyre::Result<Target> {
    let re =
        Regex::new(r"target area: x=(?P<x1>\d+)..(?P<x2>\d+), y=(?P<y1>-?\d+)..(?P<y2>-?\d+)")?;
    let captures = re.captures(input).context("unable to match input")?;
    let x1 = captures["x1"].parse()?;
    let x2 = captures["x2"].parse()?;
    let y1 = captures["y1"].parse()?;
    let y2 = captures["y2"].parse()?;

    Ok(Target { x1, x2, y1, y2 })
}

#[aoc(day17, part1)]
fn part1(target: &Target) -> Option<i16> {
    iproduct!((1..=target.x2), (target.y1..=(1 - target.y1)))
        .filter_map(|starting_velocity| {
            simulate(starting_velocity)
                .scan(i16::MIN, |prev_max_y, position @ (_, y)| {
                    *prev_max_y = y.max(*prev_max_y);
                    Some((*prev_max_y, position))
                })
                .take_while(|&(_, position)| !target.below(position))
                .find(|&(_, position)| target.contains(position))
                .map(|(max_y, _)| max_y)
        })
        .max()
}

#[aoc(day17, part2)]
fn part2(target: &Target) -> usize {
    iproduct!((1..=target.x2), target.y1..=(1 - target.y1))
        .filter(|&starting_velocity| {
            simulate(starting_velocity)
                .take_while(|&position| !target.below(position))
                .any(|position| target.contains(position))
        })
        .count()
}
