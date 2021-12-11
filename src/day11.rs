use std::collections::{HashMap, HashSet};

type Point = (i16, i16);

#[aoc_generator(day11)]
fn generator(input: &str) -> Option<HashMap<Point, u8>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| c.to_digit(10).map(|n| ((x as i16, y as i16), n as u8)))
        })
        .collect()
}

fn neighbors((x, y): Point) -> impl Iterator<Item = Point> {
    [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .into_iter()
    .map(move |(dx, dy)| (x + dx, y + dy))
}

fn step(state: &mut HashMap<Point, u8>) -> usize {
    for (_, energy) in state.iter_mut() {
        *energy += 1;
    }

    let mut flashed = HashSet::new();
    let mut search = state
        .iter()
        .filter(|(_, &e)| e > 9)
        .map(|(p, _)| p)
        .cloned()
        .collect::<Vec<Point>>();
    while let Some(point) = search.pop() {
        if flashed.contains(&point) {
            continue;
        }

        flashed.insert(point);

        for neighbor in neighbors(point) {
            if let Some(energy) = state.get_mut(&neighbor) {
                *energy += 1;

                if *energy > 9 && !flashed.contains(&neighbor) {
                    search.push(neighbor);
                }
            }
        }
    }

    for &point in &flashed {
        state.insert(point, 0);
    }

    flashed.len()
}

#[aoc(day11, part1)]
fn part1(input: &HashMap<Point, u8>) -> usize {
    (0..100)
        .scan(input.clone(), |state, _| Some(step(state)))
        .sum()
}

#[aoc(day11, part2)]
fn part2(input: &HashMap<Point, u8>) -> Option<usize> {
    (1..)
        .scan(input.clone(), |state, round| {
            let flashed = step(state);
            if flashed == state.len() {
                None
            } else {
                Some(round)
            }
        })
        .last()
        .map(|round| round + 1)
}
