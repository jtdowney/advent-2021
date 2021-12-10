enum Syntax {
    Corrupt(char),
    Incomplete(Vec<char>),
}

#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn check_syntax(line: &str) -> Syntax {
    let queue = line.chars().try_fold(vec![], |mut queue, c| {
        if queue.last().is_none() {
            queue.push(c);
            return Ok(queue);
        }

        match (c, queue.last()) {
            ('(' | '[' | '{' | '<', _) => queue.push(c),
            (')', Some('(')) | (']', Some('[')) | ('}', Some('{')) | ('>', Some('<')) => {
                queue.pop();
            }
            _ => return Err(c),
        }

        Ok(queue)
    });

    match queue {
        Ok(queue) => Syntax::Incomplete(queue),
        Err(c) => Syntax::Corrupt(c),
    }
}

#[aoc(day10, part1)]
fn part1(input: &[String]) -> usize {
    input
        .iter()
        .filter_map(|line| match check_syntax(line) {
            Syntax::Corrupt(c) => Some(c),
            Syntax::Incomplete(_) => None,
        })
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &[String]) -> usize {
    let mut scores = input
        .iter()
        .filter_map(|line| match check_syntax(line) {
            Syntax::Corrupt(_) => None,
            Syntax::Incomplete(q) => Some(q),
        })
        .map(|queue| {
            queue
                .iter()
                .rev()
                .map(|&c| match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("found {}", c),
                })
                .fold(0, |acc, score| acc * 5 + score)
        })
        .collect::<Vec<usize>>();

    scores.sort_unstable();
    scores[scores.len() / 2]
}
