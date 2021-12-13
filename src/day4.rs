use eyre::ContextCompat;
use std::{collections::HashSet, num::ParseIntError};

struct Input {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

#[derive(Default)]
struct Board {
    rows: Vec<Vec<u32>>,
}

#[derive(Default)]
struct Game {
    called_numbers: HashSet<u32>,
    winners: HashSet<usize>,
}

impl Board {
    fn is_winner(&self, numbers: &HashSet<u32>) -> bool {
        (0..5).any(|i| self.rows[i].iter().all(|n| numbers.contains(n)))
            || (0..5).any(|j| {
                self.rows
                    .iter()
                    .map(|row| row[j])
                    .all(|n| numbers.contains(&n))
            })
    }

    fn score(&self, numbers: &HashSet<u32>) -> u32 {
        self.rows
            .iter()
            .flat_map(|row| row.iter())
            .filter(|n| !numbers.contains(n))
            .sum()
    }
}

#[aoc_generator(day4)]
fn generator(input: &str) -> eyre::Result<Input> {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .map(|s| {
            s.split(',')
                .map(str::parse)
                .collect::<Result<_, ParseIntError>>()
        })
        .transpose()?
        .context("unable to parse numbers")?;

    let boards = lines.try_fold::<_, _, eyre::Result<_>>(vec![], |mut boards, line| {
        if line.is_empty() {
            boards.push(Board::default());
            return Ok(boards);
        }

        let row = line
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()?;
        let current_board = boards.last_mut().context("unable to find board")?;
        current_board.rows.push(row);

        Ok(boards)
    })?;

    Ok(Input { numbers, boards })
}

fn game_stream(input: &Input) -> impl Iterator<Item = u32> + '_ {
    input
        .numbers
        .iter()
        .scan(Game::default(), |game, &number| {
            game.called_numbers.insert(number);

            let round_winners = input
                .boards
                .iter()
                .enumerate()
                .filter(|(board_index, _)| !game.winners.contains(board_index))
                .filter(|(_, board)| board.is_winner(&game.called_numbers))
                .map(|(board_index, _)| board_index)
                .collect::<Vec<usize>>();
            game.winners.extend(&round_winners);

            let scores = round_winners
                .into_iter()
                .map(|board_index| number * input.boards[board_index].score(&game.called_numbers))
                .collect::<Vec<u32>>();
            Some(scores)
        })
        .flatten()
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> Option<u32> {
    game_stream(input).next()
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> Option<u32> {
    game_stream(input).last()
}
