use eyre::ContextCompat;
use itertools::{iproduct, Itertools};
use std::{collections::HashMap, iter};

type Point = (i16, i16);

#[derive(Clone, Debug)]
struct Image {
    pixels: HashMap<Point, bool>,
    space_pixel: bool,
}

fn enhance(image: Image, enhancement: &[bool]) -> impl Iterator<Item = Image> + '_ {
    iter::successors(Some(image), |previous| {
        let (&(startx, _), &(endx, _)) = previous
            .pixels
            .keys()
            .minmax_by_key(|(x, _)| x)
            .into_option()?;
        let (&(_, starty), &(_, endy)) = previous
            .pixels
            .keys()
            .minmax_by_key(|(_, y)| y)
            .into_option()?;

        let pixels = iproduct!(((startx - 1)..=(endx + 1)), ((starty - 1)..=(endy + 1)))
            .map(|(x, y)| {
                let lookup = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (0, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ]
                .into_iter()
                .map(move |(dx, dy)| (x + dx, y + dy))
                .fold(0, |acc, point| {
                    let bit = previous
                        .pixels
                        .get(&point)
                        .copied()
                        .unwrap_or(previous.space_pixel) as usize;
                    (acc << 1) + bit
                });

                let value = enhancement[lookup];
                ((x, y), value)
            })
            .collect();

        let space_pixel = if previous.space_pixel {
            enhancement.last().copied().unwrap_or_default()
        } else {
            enhancement.first().copied().unwrap_or_default()
        };

        let image = Image {
            pixels,
            space_pixel,
        };

        Some(image)
    })
    .skip(1)
}

#[derive(Debug)]
struct Input {
    enhancement: Vec<bool>,
    image: Image,
}

fn parse_pixel(pixel: char) -> bool {
    match pixel {
        '#' => true,
        '.' => false,
        c => panic!("unrecognizable value {}", c),
    }
}

#[aoc_generator(day20)]
fn generator(input: &str) -> eyre::Result<Input> {
    let mut lines = input.lines();
    let enhancement = lines
        .next()
        .map(|line| line.chars().map(parse_pixel).collect())
        .context("unable to read enhancement")?;

    let pixels = lines
        .skip(1)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let value = parse_pixel(c);
                ((x as i16, y as i16), value)
            })
        })
        .collect();
    let image = Image {
        pixels,
        space_pixel: false,
    };

    Ok(Input { enhancement, image })
}

#[aoc(day20, part1)]
fn part1(input: &Input) -> Option<usize> {
    let image = input.image.clone();
    let count = enhance(image, &input.enhancement)
        .take(2)
        .last()?
        .pixels
        .values()
        .filter(|&v| *v)
        .count();

    Some(count)
}

#[aoc(day20, part2)]
fn part2(input: &Input) -> Option<usize> {
    let image = input.image.clone();
    let count = enhance(image, &input.enhancement)
        .take(50)
        .last()?
        .pixels
        .values()
        .filter(|&v| *v)
        .count();

    Some(count)
}
