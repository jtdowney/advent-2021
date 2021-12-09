use std::collections::{HashMap, HashSet};

type Point = (i16, i16);

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

fn neighbors((x, y): Point) -> impl Iterator<Item = Point> {
    [(0, -1), (-1, 0), (1, 0), (0, 1)]
        .into_iter()
        .map(move |(dx, dy)| (x + dx, y + dy))
}

fn low_points(input: &HashMap<Point, u16>) -> impl Iterator<Item = Point> + '_ {
    input
        .iter()
        .filter(|&(&point, &height)| {
            neighbors(point)
                .filter_map(|neighbor| input.get(&neighbor))
                .all(|&neighbor_height| neighbor_height > height)
        })
        .map(|(&point, _)| point)
}

fn basin_size(start: Point, input: &HashMap<Point, u16>) -> usize {
    let mut visited = HashSet::new();
    let mut search = vec![start];

    while let Some(point) = search.pop() {
        visited.insert(point);

        let basin_neighbors = neighbors(point).filter(|neighbor| {
            match (visited.contains(neighbor), input.get(neighbor)) {
                (true, _) => false,
                (_, Some(&h)) => h < 9,
                _ => false,
            }
        });

        search.extend(basin_neighbors);
    }

    visited.len()
}

#[aoc(day9, part1)]
fn part1(input: &HashMap<Point, u16>) -> u16 {
    low_points(input)
        .filter_map(|point| input.get(&point))
        .map(|&height| height + 1)
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &HashMap<Point, u16>) -> usize {
    let mut basin_sizes = low_points(input)
        .map(|point| basin_size(point, input))
        .collect::<Vec<usize>>();

    basin_sizes.sort_unstable();
    basin_sizes.into_iter().rev().take(3).product()
}
