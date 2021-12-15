use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

type Point = (i16, i16);

#[aoc_generator(day15)]
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
    [(0, -1), (-1, 0), (1, 0), (0, 1)]
        .into_iter()
        .map(move |(dx, dy)| (x + dx, y + dy))
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct SearchEntry {
    point: Point,
    score: u32,
}

fn search(grid: &HashMap<Point, u8>) -> u32 {
    let goal = grid.keys().max().cloned().unwrap();
    let mut cost = HashMap::new();
    cost.insert((0, 0), 0);

    let mut search = BinaryHeap::new();
    search.push(Reverse(SearchEntry {
        point: (0, 0),
        score: 0,
    }));

    while let Some(Reverse(entry)) = search.pop() {
        if entry.point == goal {
            break;
        }

        neighbors(entry.point)
            .filter(|n| grid.contains_key(n))
            .for_each(|neighbor| {
                if !cost.contains_key(&neighbor)
                    || cost[&entry.point] + (grid[&neighbor] as u32) < cost[&neighbor]
                {
                    let distance = manhatten(neighbor, goal);
                    let neighbor_cost = cost[&entry.point] + grid[&neighbor] as u32;
                    cost.insert(neighbor, neighbor_cost);

                    search.push(Reverse(SearchEntry {
                        point: neighbor,
                        score: neighbor_cost + distance,
                    }));
                }
            })
    }

    cost[&goal]
}

fn manhatten((x, y): Point, (gx, gy): Point) -> u32 {
    ((x - gx).abs() + (y - gy).abs()) as u32
}

fn expand(grid: &HashMap<Point, u8>) -> HashMap<Point, u8> {
    let (endx, endy) = grid.keys().max().cloned().unwrap();
    let width = endx + 1;
    let height = endy + 1;

    (0..5)
        .flat_map(|tx| {
            (0..5).flat_map(move |ty| {
                grid.iter().map(move |(&(x, y), &risk)| {
                    let x = x + (tx as i16 * width);
                    let y = y + (ty as i16 * height);
                    let risk = 1 + (risk - 1 + tx + ty) % 9;

                    ((x, y), risk)
                })
            })
        })
        .collect()
}

#[aoc(day15, part1)]
fn part1(input: &HashMap<Point, u8>) -> u32 {
    search(input)
}

#[aoc(day15, part2)]
fn part2(input: &HashMap<Point, u8>) -> u32 {
    let grid = expand(input);
    search(&grid)
}
