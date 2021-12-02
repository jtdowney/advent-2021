use eyre::eyre;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space1, u32},
    Finish, IResult,
};

#[derive(Copy, Clone)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Default)]
struct State {
    position: u32,
    depth: u32,
    aim: u32,
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, tag) = alt((tag("forward"), tag("down"), tag("up")))(input)?;
    let (input, _) = space1(input)?;
    let (input, amount) = u32(input)?;
    let command = match tag {
        "forward" => Command::Forward(amount),
        "down" => Command::Down(amount),
        "up" => Command::Up(amount),
        _ => unreachable!(),
    };

    Ok((input, command))
}

#[aoc_generator(day2)]
fn generator(input: &str) -> eyre::Result<Vec<Command>> {
    let commands = input
        .lines()
        .map(|line| {
            parse_command(line)
                .finish()
                .map(|(_, command)| command)
                .map_err(|e| eyre!("failed to parse line: {:?}", e))
        })
        .collect::<eyre::Result<_>>()?;
    Ok(commands)
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
