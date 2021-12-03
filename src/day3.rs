use itertools::Itertools;

#[aoc_generator(day3)]
fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<char>]) -> eyre::Result<u32> {
    let digits = input[0].len();

    let gamma = (0..digits)
        .map(|i| {
            let counts = input.iter().map(|line| line[i]).counts();
            let (&digit, _) = counts.iter().max_by_key(|(_, &v)| v).unwrap();
            digit
        })
        .collect::<String>();
    let gamma = u32::from_str_radix(&gamma, 2)?;

    let epsilon = (0..digits)
        .map(|i| {
            let counts = input.iter().map(|line| line[i]).counts();
            let (&digit, _) = counts.iter().min_by_key(|(_, &v)| v).unwrap();
            digit
        })
        .collect::<String>();
    let epsilon = u32::from_str_radix(&epsilon, 2)?;

    Ok(gamma * epsilon)
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<char>]) -> eyre::Result<u32> {
    let mut entries = input.to_vec();
    let mut digit = 0;
    loop {
        if entries.len() == 1 {
            break;
        }

        let counts = entries.iter().map(|line| line[digit]).counts();
        let pick = if counts[&'0'] == counts[&'1'] {
            '1'
        } else {
            let (&pick, _) = counts.iter().max_by_key(|(_, &v)| v).unwrap();
            pick
        };

        entries.retain(|entry| entry[digit] == pick);
        digit += 1;
    }
    let generator = entries[0].iter().collect::<String>();
    let generator = u32::from_str_radix(&generator, 2)?;

    let mut entries = input.to_vec();
    let mut digit = 0;
    loop {
        if entries.len() == 1 {
            break;
        }

        let counts = entries.iter().map(|line| line[digit]).counts();
        let pick = if counts[&'0'] == counts[&'1'] {
            '0'
        } else {
            let (&pick, _) = counts.iter().min_by_key(|(_, &v)| v).unwrap();
            pick
        };

        entries.retain(|entry| entry[digit] == pick);
        digit += 1;
    }
    let scrubber = entries[0].iter().collect::<String>();
    let scrubber = u32::from_str_radix(&scrubber, 2)?;

    Ok(generator * scrubber)
}
