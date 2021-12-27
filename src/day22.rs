use eyre::ContextCompat;
use itertools::iproduct;
use regex::Regex;
use std::collections::HashSet;

type Point = (i64, i64, i64);
type Input = Vec<(bool, (Point, Point))>;

#[aoc_generator(day22)]
fn generator(input: &str) -> eyre::Result<Input> {
    let re = Regex::new(
        r"(?P<state>on|off) x=(?P<x1>-?\d+)..(?P<x2>-?\d+),y=(?P<y1>-?\d+)..(?P<y2>-?\d+),z=(?P<z1>-?\d+)..(?P<z2>-?\d+)",
    )?;

    input
        .lines()
        .map(|line| {
            let captures = re.captures(line)?;
            let state = &captures["state"] == "on";
            let x1 = captures["x1"].parse().ok()?;
            let x2 = captures["x2"].parse().ok()?;
            let y1 = captures["y1"].parse().ok()?;
            let y2 = captures["y2"].parse().ok()?;
            let z1 = captures["z1"].parse().ok()?;
            let z2 = captures["z2"].parse().ok()?;
            Some((state, ((x1, y1, z1), (x2, y2, z2))))
        })
        .collect::<Option<Input>>()
        .context("unable to parse input")
}

#[aoc(day22, part1)]
fn part1(input: &Input) -> usize {
    let cubes = input
        .iter()
        .filter(|&&(_, ((x1, y1, z1), (x2, y2, z2)))| {
            x1 >= -50 && x2 <= 50 && y1 >= -50 && y2 <= 50 && z1 >= -50 && z2 <= 50
        })
        .fold(
            HashSet::new(),
            |mut acc, &(state, ((x1, y1, z1), (x2, y2, z2)))| {
                for point in iproduct!(x1..=x2, y1..=y2, z1..=z2) {
                    if state {
                        acc.insert(point);
                    } else {
                        acc.remove(&point);
                    }
                }

                acc
            },
        );

    cubes.len()
}

#[derive(Debug, PartialEq)]
struct Cube {
    min: Point,
    max: Point,
    subtracted: Vec<Cube>,
}

impl Cube {
    fn new(min: Point, max: Point) -> Self {
        Self {
            min,
            max,
            subtracted: vec![],
        }
    }

    fn volume(&self) -> usize {
        let (x1, y1, z1) = self.min;
        let (x2, y2, z2) = self.max;

        let volume = ((x2 - x1 + 1) * (y2 - y1 + 1) * (z2 - z1 + 1)) as usize;
        let subtracted_volume = self.subtracted.iter().map(Cube::volume).sum::<usize>();

        volume - subtracted_volume
    }

    fn subtract(&mut self, other: &Cube) {
        if let Some(intersection) = self.intersection(other) {
            for sub in self.subtracted.iter_mut() {
                sub.subtract(&intersection);
            }

            self.subtracted.push(intersection);
        }
    }

    fn intersection(&self, other: &Cube) -> Option<Cube> {
        let (x1, y1, z1) = self.min;
        let (x2, y2, z2) = self.max;
        let (ox1, oy1, oz1) = other.min;
        let (ox2, oy2, oz2) = other.max;

        let ix1 = x1.max(ox1);
        let ix2 = x2.min(ox2);
        let iy1 = y1.max(oy1);
        let iy2 = y2.min(oy2);
        let iz1 = z1.max(oz1);
        let iz2 = z2.min(oz2);

        if ix2 - ix1 < 0 || iy2 - iy1 < 0 || iz2 - iz1 < 0 {
            None
        } else {
            Some(Cube::new((ix1, iy1, iz1), (ix2, iy2, iz2)))
        }
    }
}

#[aoc(day22, part2)]
fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|&(state, (min, max))| (state, Cube::new(min, max)))
        .fold(Vec::<Cube>::new(), |mut acc, (state, cube)| {
            for existing in acc.iter_mut() {
                existing.subtract(&cube);
            }

            if state {
                acc.push(cube);
            }

            acc
        })
        .iter()
        .map(Cube::volume)
        .sum()
}
