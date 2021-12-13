use std::collections::{HashMap, HashSet};

#[aoc_generator(day12)]
fn generator(input: &str) -> HashMap<String, Vec<String>> {
    input.lines().filter_map(|line| line.split_once("-")).fold(
        HashMap::new(),
        |mut acc, (start, end)| {
            acc.entry(start.to_string())
                .or_default()
                .push(end.to_string());
            acc.entry(end.to_string())
                .or_default()
                .push(start.to_string());
            acc
        },
    )
}

#[derive(Default, Clone)]
struct SearchEntry<'a> {
    cave: &'a str,
    path: Vec<&'a str>,
    visited: HashSet<&'a str>,
    small_double_visited: bool,
}

fn is_small_cave(input: &str) -> bool {
    match input {
        "start" | "end" => false,
        value => value == value.to_ascii_lowercase(),
    }
}

fn solve(input: &HashMap<String, Vec<String>>, small_double_visited: bool) -> usize {
    let mut complete = HashSet::new();
    let mut search = vec![SearchEntry {
        cave: "start",
        small_double_visited,
        ..Default::default()
    }];

    while let Some(mut entry) = search.pop() {
        entry.path.push(entry.cave);

        match (
            entry.cave,
            is_small_cave(entry.cave),
            entry.small_double_visited,
            entry.visited.contains(entry.cave),
        ) {
            ("end", _, _, _) => {
                complete.insert(entry.path);
                continue;
            }
            (_, true, true, true) => continue,
            _ => {
                let small_double_visited = entry.small_double_visited
                    || (is_small_cave(entry.cave) && entry.visited.contains(entry.cave));
                entry.visited.insert(entry.cave);

                let neightbors = input[entry.cave]
                    .iter()
                    .filter(|&neighbor| neighbor != "start")
                    .map(|neighbor| SearchEntry {
                        cave: neighbor,
                        small_double_visited,
                        ..entry.clone()
                    });

                search.extend(neightbors);
            }
        }
    }

    complete.len()
}

#[aoc(day12, part1)]
fn part1(input: &HashMap<String, Vec<String>>) -> usize {
    solve(input, true)
}

#[aoc(day12, part2)]
fn part2(input: &HashMap<String, Vec<String>>) -> usize {
    solve(input, false)
}
