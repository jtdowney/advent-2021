use std::{collections::HashSet, num::ParseIntError};

#[derive(Debug, Clone)]
struct Input {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

#[derive(Debug, Default, Clone)]
struct Board {
    rows: Vec<Vec<u32>>,
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
fn generator(input: &str) -> Result<Input, ParseIntError> {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .map(|s| {
            s.split(',')
                .map(str::parse)
                .collect::<Result<_, ParseIntError>>()
        })
        .transpose()?
        .unwrap();
    let boards = lines.try_fold(vec![], |mut boards, line| {
        if line.is_empty() {
            boards.push(Board::default());
            return Ok(boards);
        }

        let row = line
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()?;
        let current_board = boards.last_mut().unwrap();
        current_board.rows.push(row);

        Ok(boards)
    })?;

    Ok(Input { numbers, boards })
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> u32 {
    let mut called_numbers = HashSet::new();
    for &number in &input.numbers {
        called_numbers.insert(number);

        let winner = input.boards.iter().find(|b| b.is_winner(&called_numbers));
        if let Some(w) = winner {
            return w.score(&called_numbers) * number;
        }
    }

    unreachable!()
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> u32 {
    let mut called_numbers = HashSet::new();
    let mut winners = vec![];
    let mut skip = HashSet::new();
    for (round, &number) in input.numbers.iter().enumerate() {
        called_numbers.insert(number);

        let round_winners = input
            .boards
            .iter()
            .enumerate()
            .filter(|(board_index, _)| !skip.contains(board_index))
            .filter(|(_, board)| board.is_winner(&called_numbers))
            .map(|(board_index, _)| (round, board_index))
            .collect::<Vec<(usize, usize)>>();
        winners.extend_from_slice(&round_winners);
        for (_, board_index) in round_winners {
            skip.insert(board_index);
        }
    }

    let &(round, board_index) = winners.last().unwrap();
    let called_numbers = input.numbers.iter().take(round + 1).copied().collect();
    input.boards[board_index].score(&called_numbers) * input.numbers[round]
}
