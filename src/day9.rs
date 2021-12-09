use std::collections::{HashMap, HashSet, VecDeque};

type Point = (i16, i16);

const NEIGHBOR_DELTAS: [Point; 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

#[aoc_generator(day9)]
fn generator(input: &str) -> Option<HashMap<Point, u16>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| c.to_digit(10).map(|n| ((x as i16, y as i16), n as u16)))
        })
        .collect()
}

fn low_points(input: &HashMap<Point, u16>) -> impl Iterator<Item = Point> + '_ {
    input
        .iter()
        .filter(|&(&(x, y), &height)| {
            NEIGHBOR_DELTAS
                .into_iter()
                .filter_map(|(dx, dy)| input.get(&(x + dx, y + dy)))
                .all(|&neighbor_height| neighbor_height > height)
        })
        .map(|(&point, _)| point)
}

fn basin_size(start: Point, input: &HashMap<Point, u16>) -> usize {
    let mut visited = HashSet::new();
    let mut search = VecDeque::new();
    search.push_back(start);

    while let Some(point @ (x, y)) = search.pop_front() {
        visited.insert(point);

        let neighbors = NEIGHBOR_DELTAS
            .into_iter()
            .map(|(dx, dy)| {
                let nx = x + dx;
                let ny = y + dy;
                (nx, ny)
            })
            .filter(|neighbor| {
                if visited.contains(neighbor) {
                    return false;
                }

                if let Some(&height) = input.get(neighbor) {
                    height < 9
                } else {
                    false
                }
            })
            .collect::<Vec<Point>>();

        search.extend(neighbors);
    }

    visited.len()
}

#[aoc(day9, part1)]
fn part1(input: &HashMap<Point, u16>) -> u16 {
    low_points(&input)
        .filter_map(|point| input.get(&point))
        .map(|&height| height + 1)
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &HashMap<Point, u16>) -> usize {
    let mut basin_sizes = low_points(&input)
        .map(|point| basin_size(point, input))
        .collect::<Vec<usize>>();

    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).copied().product()
}
