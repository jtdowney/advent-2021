use eyre::{eyre, ContextCompat};

#[aoc_generator(day6)]
fn generator(input: &str) -> eyre::Result<Vec<usize>> {
    input
        .lines()
        .next()
        .context("unable to read input")?
        .split(',')
        .map(|value| value.parse().map_err(|_| eyre!("unable to parse number")))
        .collect()
}

fn solve(input: &[usize], rounds: usize) -> usize {
    let mut population = input.iter().fold([0; 9], |mut acc, &cycle| {
        acc[cycle as usize] += 1;
        acc
    });

    for _ in 0..rounds {
        let mut next = [0; 9];
        for (cycle, &count) in population.iter().enumerate() {
            let cycle = if cycle == 0 {
                next[8] += count;
                6
            } else {
                cycle - 1
            };

            next[cycle] += count;
        }

        population = next;
    }

    population.iter().sum()
}

#[aoc(day6, part1)]
fn part1(input: &[usize]) -> usize {
    solve(input, 80)
}

#[aoc(day6, part2)]
fn part2(input: &[usize]) -> usize {
    solve(input, 256)
}
