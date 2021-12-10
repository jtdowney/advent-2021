#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn check_syntax(line: &str) -> Result<Vec<char>, char> {
    line.chars().try_fold(vec![], |mut queue, c| {
        match (c, queue.last()) {
            ('(' | '[' | '{' | '<', _) => queue.push(c),
            (')', Some('(')) | (']', Some('[')) | ('}', Some('{')) | ('>', Some('<')) => {
                queue.pop();
            }
            _ => return Err(c),
        }

        Ok(queue)
    })
}

#[aoc(day10, part1)]
fn part1(input: &[String]) -> usize {
    input
        .iter()
        .filter_map(|line| match check_syntax(line) {
            Ok(_) => None,
            Err(c) => Some(c),
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
        .filter_map(|line| check_syntax(line).ok())
        .map(|queue| {
            queue
                .iter()
                .rev()
                .map(|&c| match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                })
                .fold(0, |acc, score| acc * 5 + score)
        })
        .collect::<Vec<usize>>();

    scores.sort_unstable();
    scores[scores.len() / 2]
}
