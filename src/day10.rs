#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn check_syntax(line: &str) -> Result<Vec<char>, char> {
    line.chars().try_fold(vec![], |mut stack, c| {
        match (c, stack.last()) {
            ('(' | '[' | '{' | '<', _) => stack.push(c),
            (')', Some('(')) | (']', Some('[')) | ('}', Some('{')) | ('>', Some('<')) => {
                stack.pop();
            }
            _ => return Err(c),
        }

        Ok(stack)
    })
}

#[aoc(day10, part1)]
fn part1(input: &[String]) -> u64 {
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
fn part2(input: &[String]) -> u64 {
    let mut scores = input
        .iter()
        .filter_map(|line| check_syntax(line).ok())
        .map(|stack| {
            stack
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
        .collect::<Vec<_>>();

    scores.sort_unstable();
    scores[scores.len() / 2]
}
