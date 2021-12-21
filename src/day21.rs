use eyre::ContextCompat;
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day21)]
fn generator(input: &str) -> eyre::Result<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .last()
                .and_then(|c| c.to_digit(10).map(|d| d as u8))
                .context("can't read start position")
        })
        .collect()
}

struct GameState {
    position: Vec<u8>,
    scores: Vec<usize>,
    rolls: usize,
    previous_score: usize,
}

#[aoc(day21, part1)]
fn part1(input: &[u8]) -> Option<usize> {
    let state = GameState {
        position: input.to_vec(),
        scores: vec![0, 0],
        rolls: 0,
        previous_score: 0,
    };

    [0, 1]
        .into_iter()
        .cycle()
        .zip((0..).chunks(3).into_iter())
        .scan(state, |state, (player, moves)| {
            for round in moves {
                let roll = ((round % 100) + 1) as u8;
                state.position[player] = 1 + (state.position[player] + roll - 1) % 10;
                state.rolls += 1;
            }

            let other = (player + 1) % 2;
            state.scores[player] += state.position[player] as usize;
            state.previous_score = state.scores[other];

            let player_score = state.scores[player];
            Some((state.previous_score, player_score, state.rolls))
        })
        .take_while(|&(score, _, _)| score < 1000)
        .last()
        .map(|(score, _, rolls)| score * rolls)
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct PlayerState {
    score: u8,
    position: u8,
}

type Cache = HashMap<(PlayerState, PlayerState), (usize, usize)>;

const FREQUENCIES: [(u8, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn solve_quantum(
    players @ (active, other): (PlayerState, PlayerState),
    cache: &mut Cache,
) -> (usize, usize) {
    if let Some(&scores) = cache.get(&players) {
        return scores;
    }

    if active.score >= 21 {
        return (1, 0);
    } else if other.score >= 21 {
        return (0, 1);
    }

    let mut scores = [0; 2];
    for (roll, frequency) in FREQUENCIES {
        let position = 1 + (active.position + roll - 1) % 10;
        let score = active.score + position;
        let next = PlayerState { score, position };

        let (other_wins, active_wins) = solve_quantum((other, next), cache);

        scores[0] += active_wins * frequency;
        scores[1] += other_wins * frequency;
    }

    let score_tuple = (scores[0], scores[1]);
    cache.insert(players, score_tuple);

    score_tuple
}

#[aoc(day21, part2)]
fn part2(input: &[u8]) -> usize {
    let mut cache = HashMap::new();
    let player1 = PlayerState {
        score: 0,
        position: input[0] as u8,
    };
    let player2 = PlayerState {
        score: 0,
        position: input[1] as u8,
    };
    let (player1_wins, player2_wins) = solve_quantum((player1, player2), &mut cache);

    player1_wins.max(player2_wins)
}
