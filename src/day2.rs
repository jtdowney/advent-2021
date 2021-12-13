use eyre::{bail, Context, ContextCompat};
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let tag = parts.next().context("no command")?;
        let amount: u32 = parts
            .next()
            .context("no amount")
            .and_then(|value| value.parse().context("unable to parse amount"))?;

        let command = match tag {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            _ => bail!("unable to construct command"),
        };

        Ok(command)
    }
}

#[derive(Default)]
struct State {
    position: u32,
    depth: u32,
    aim: u32,
}

#[aoc_generator(day2)]
fn generator(input: &str) -> eyre::Result<Vec<Command>> {
    input.lines().map(str::parse).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Command]) -> u32 {
    let State {
        position, depth, ..
    } = input.iter().fold(State::default(), |mut state, &command| {
        match command {
            Command::Forward(amount) => state.position += amount,
            Command::Down(amount) => state.depth += amount,
            Command::Up(amount) => state.depth -= amount,
        }
        state
    });

    position * depth
}

#[aoc(day2, part2)]
fn part2(input: &[Command]) -> u32 {
    let State {
        position, depth, ..
    } = input.iter().fold(State::default(), |mut state, &command| {
        match command {
            Command::Forward(amount) => {
                state.position += amount;
                state.depth += state.aim * amount;
            }
            Command::Down(amount) => state.aim += amount,
            Command::Up(amount) => state.aim -= amount,
        }
        state
    });

    position * depth
}
