use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

type Point = (i16, i16);

#[aoc_generator(day15)]
fn generator(input: &str) -> Option<HashMap<Point, u32>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| c.to_digit(10).map(|n| ((x as i16, y as i16), n)))
        })
        .collect()
}

fn neighbors((x, y): Point) -> impl Iterator<Item = Point> {
    [(0, -1), (-1, 0), (1, 0), (0, 1)]
        .into_iter()
        .map(move |(dx, dy)| (x + dx, y + dy))
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct SearchEntry {
    point: Point,
    risk: u32,
}

impl PartialOrd for SearchEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.risk.partial_cmp(&other.risk).map(|o| o.reverse())
    }
}

impl Ord for SearchEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.risk.cmp(&other.risk).reverse()
    }
}

fn search(grid: &HashMap<Point, u32>) -> u32 {
    let goal = grid.keys().max().cloned().unwrap();

    let mut total_risk = HashMap::new();
    total_risk.insert((0, 0), 0);

    let mut search = BinaryHeap::new();
    search.push(SearchEntry {
        point: (0, 0),
        risk: 0,
    });

    while let Some(entry) = search.pop() {
        if entry.point == goal {
            break;
        }

        neighbors(entry.point)
            .filter(|next| grid.contains_key(next))
            .for_each(|next| {
                if !total_risk.contains_key(&next)
                    || total_risk[&entry.point] + grid[&next] < total_risk[&next]
                {
                    let risk = total_risk[&entry.point] + grid[&next];
                    total_risk.insert(next, risk);

                    search.push(SearchEntry { point: next, risk });
                }
            })
    }

    total_risk[&goal]
}

fn expand(grid: &HashMap<Point, u32>) -> HashMap<Point, u32> {
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
fn part1(input: &HashMap<Point, u32>) -> u32 {
    search(input)
}

#[aoc(day15, part2)]
fn part2(input: &HashMap<Point, u32>) -> u32 {
    let grid = expand(input);
    search(&grid)
}
