use eyre::bail;
use itertools::Itertools;
use std::{ops::Add, str::FromStr};

#[derive(Clone, Debug, Default, PartialEq)]
struct SnailNumber {
    values: Vec<u8>,
    depths: Vec<u8>,
}

impl SnailNumber {
    fn explode(&mut self) -> bool {
        let position = self.depths.iter().position(|&depth| depth == 5);
        if let Some(index) = position {
            if index > 0 {
                self.values[index - 1] += self.values[index];
            }

            if index + 2 < self.values.len() {
                self.values[index + 2] += self.values[index + 1];
            }

            self.values.remove(index);
            self.depths.remove(index);

            self.values[index] = 0;
            self.depths[index] -= 1;

            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        let position = self.values.iter().position(|&value| value >= 10);
        if let Some(index) = position {
            let value = self.values[index];

            let left = value / 2;
            self.values[index] = left;
            self.depths[index] += 1;

            let right = value - left;
            self.values.insert(index + 1, right);
            self.depths.insert(index + 1, self.depths[index]);

            true
        } else {
            false
        }
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn magnitude(&self) -> usize {
        let mut values = self
            .values
            .iter()
            .copied()
            .map(usize::from)
            .collect::<Vec<_>>();
        let mut depths = self.depths.clone();
        let max_depth = depths.iter().max().copied().unwrap_or_default();

        for depth in (1..=max_depth).rev() {
            while let Some(index) = depths.iter().position(|&d| d == depth) {
                let right = values.remove(index + 1);
                depths.remove(index + 1);

                depths[index] -= 1;
                values[index] = values[index] * 3 + right * 2;
            }
        }

        values[0]
    }
}

impl Add for SnailNumber {
    type Output = SnailNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut values = self.values.clone();
        values.extend_from_slice(&rhs.values);

        let mut depths = self.depths;
        depths.extend_from_slice(&rhs.depths);
        depths.iter_mut().for_each(|depth| *depth += 1);

        let mut number = SnailNumber { values, depths };
        number.reduce();

        number
    }
}

#[derive(Debug, Default)]
struct ParserState {
    current_depth: u8,
    values: Vec<u8>,
    depths: Vec<u8>,
}

impl From<ParserState> for SnailNumber {
    fn from(ParserState { values, depths, .. }: ParserState) -> Self {
        SnailNumber { values, depths }
    }
}

impl FromStr for SnailNumber {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = s.chars().try_fold(ParserState::default(), |mut state, c| {
            match c {
                '[' => state.current_depth += 1,
                ']' => state.current_depth -= 1,
                ',' => {}
                c if c.is_ascii_digit() => {
                    state.values.push(c.to_digit(10).unwrap() as u8);
                    state.depths.push(state.current_depth);
                }
                _ => bail!("unexpected {}", c),
            }

            Ok(state)
        })?;

        Ok(state.into())
    }
}

#[aoc_generator(day18)]
fn generator(input: &str) -> eyre::Result<Vec<SnailNumber>> {
    input.lines().map(str::parse).collect()
}

#[aoc(day18, part1)]
fn part1(input: &[SnailNumber]) -> Option<usize> {
    input
        .to_vec()
        .into_iter()
        .reduce(|acc, number| acc + number)
        .map(|number| number.magnitude())
}

#[aoc(day18, part2)]
fn part2(input: &[SnailNumber]) -> Option<usize> {
    input
        .to_vec()
        .into_iter()
        .permutations(2)
        .map(|numbers| {
            let result = numbers[0].clone() + numbers[1].clone();
            result.magnitude()
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() -> eyre::Result<()> {
        let actual: SnailNumber = "[[1,[2,3]],[[4,5],6]]".parse()?;
        let expected = SnailNumber {
            values: vec![1, 2, 3, 4, 5, 6],
            depths: vec![2, 3, 3, 3, 3, 2],
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_expode() -> eyre::Result<()> {
        let expected: SnailNumber = "[[[[0,9],2],3],4]".parse()?;
        let mut number: SnailNumber = "[[[[[9,8],1],2],3],4]".parse()?;
        assert!(number.explode());
        assert_eq!(number, expected);

        let expected: SnailNumber = "[7,[6,[5,[7,0]]]]".parse()?;
        let mut number: SnailNumber = "[7,[6,[5,[4,[3,2]]]]]".parse()?;
        assert!(number.explode());
        assert_eq!(number, expected);

        let expected: SnailNumber = "[[6,[5,[7,0]]],3]".parse()?;
        let mut number: SnailNumber = "[[6,[5,[4,[3,2]]]],1]".parse()?;
        assert!(number.explode());
        assert_eq!(number, expected);

        let expected: SnailNumber = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse()?;
        let mut number: SnailNumber = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse()?;
        assert!(number.explode());
        assert_eq!(number, expected);

        let expected: SnailNumber = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse()?;
        let mut number: SnailNumber = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse()?;
        assert!(number.explode());
        assert_eq!(number, expected);

        Ok(())
    }

    #[test]
    fn test_split() -> eyre::Result<()> {
        let expected: SnailNumber = "[[5,5],1]".parse()?;
        let mut number: SnailNumber = "[10,1]".parse()?;
        assert!(number.split());
        assert_eq!(number, expected);

        Ok(())
    }

    #[test]
    fn test_add() -> eyre::Result<()> {
        let left: SnailNumber = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse()?;
        let right: SnailNumber = "[1,1]".parse()?;
        let actual = left + right;
        let expected: SnailNumber = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse()?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_add_example() -> eyre::Result<()> {
        let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
        let numbers = generator(input)?;
        let actual = numbers
            .into_iter()
            .reduce(|acc, number| acc + number)
            .unwrap();
        let expected: SnailNumber =
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse()?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_magnitude() -> eyre::Result<()> {
        let number: SnailNumber = "[[1,2],[[3,4],5]]".parse()?;
        assert_eq!(number.magnitude(), 143);

        let number: SnailNumber = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse()?;
        assert_eq!(number.magnitude(), 1384);

        let number: SnailNumber = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse()?;
        assert_eq!(number.magnitude(), 445);

        let number: SnailNumber = "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse()?;
        assert_eq!(number.magnitude(), 791);

        let number: SnailNumber = "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse()?;
        assert_eq!(number.magnitude(), 1137);

        let number: SnailNumber =
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse()?;
        assert_eq!(number.magnitude(), 3488);

        Ok(())
    }
}
